//! 推理 worker 池。
//!
//! HTTP 线程只做协议层的事（解析 multipart、写响应），真正的 CPU 推理交给
//! 独立线程池：请求被投递到有界队列，由常驻 worker 线程消费，结果通过
//! `oneshot` 回传。模型只加载一次，被所有 worker 共享（只读）。

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread;

use image::RgbImage;
use tokio::sync::oneshot;

use crate::document::Document;
use crate::pipeline::Pipeline;

/// worker / 模型加载线程的栈大小：burn 生成的模型代码栈帧很大。
const STACK_SIZE: usize = 64 << 20;
/// 默认 worker 数的上限，避免在核心很多的机器上一次吃满 CPU。
const MAX_DEFAULT_WORKERS: usize = 4;

/// 一次推理任务。
struct Job {
    image: RgbImage,
    use_layout: bool,
    reply: oneshot::Sender<Result<Document, String>>,
}

/// 提交失败的原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmitError {
    /// 队列已满（所有 worker 都在忙，积压达到上限）
    Busy,
    /// worker 全部退出，池不可用
    Closed,
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Busy => f.write_str("推理队列已满"),
            Self::Closed => f.write_str("推理 worker 不可用"),
        }
    }
}

pub struct WorkerPool {
    /// `None` 表示已关闭（Drop 时置空以通知 worker 退出）
    tx: Option<SyncSender<Job>>,
    handles: Vec<thread::JoinHandle<()>>,
    workers: usize,
}

impl WorkerPool {
    /// `workers = 0` 按 CPU 核数自动选择，`queue = 0` 取 `workers * 2`。
    ///
    /// 模型在专用大栈线程里加载一次，再由 `Arc` 共享给所有 worker。
    pub fn new(workers: usize, queue: usize) -> std::io::Result<Self> {
        let workers = if workers == 0 {
            default_workers()
        } else {
            workers
        };
        let queue = if queue == 0 { workers * 2 } else { queue };

        let pipeline = load_pipeline()?;

        let (tx, rx) = mpsc::sync_channel::<Job>(queue);
        let rx = Arc::new(Mutex::new(rx));
        let mut handles = Vec::with_capacity(workers);
        for i in 0..workers {
            let rx = Arc::clone(&rx);
            let pipeline = Arc::clone(&pipeline);
            let handle = thread::Builder::new()
                .name(format!("ocr-worker-{i}"))
                .stack_size(STACK_SIZE)
                .spawn(move || worker_loop(&rx, &pipeline))?;
            handles.push(handle);
        }

        Ok(Self {
            tx: Some(tx),
            handles,
            workers,
        })
    }

    /// 实际启动的 worker 数。
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// 投递任务，不阻塞：队列满立即返回 [`SubmitError::Busy`]，
    /// 由调用方决定排队或直接回 503。
    pub fn submit(
        &self,
        image: RgbImage,
        use_layout: bool,
    ) -> Result<oneshot::Receiver<Result<Document, String>>, SubmitError> {
        let (reply, rx) = oneshot::channel();
        let job = Job {
            image,
            use_layout,
            reply,
        };
        let Some(tx) = self.tx.as_ref() else {
            return Err(SubmitError::Closed);
        };
        match tx.try_send(job) {
            Ok(()) => Ok(rx),
            Err(TrySendError::Full(_)) => Err(SubmitError::Busy),
            Err(TrySendError::Disconnected(_)) => Err(SubmitError::Closed),
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        // 关掉发送端，worker 的 recv 返回 Err 后自然退出
        self.tx.take();
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

fn worker_loop(rx: &Mutex<Receiver<Job>>, pipeline: &Pipeline) {
    loop {
        // 只在取任务时持锁：worker 阻塞在 recv 上时不会妨碍其他 worker 取任务
        let job = {
            let Ok(guard) = rx.lock() else {
                return;
            };
            match guard.recv() {
                Ok(job) => job,
                Err(_) => return,
            }
        };

        // 单个请求 panic 不能带走整个 worker
        let result = catch_unwind(AssertUnwindSafe(|| {
            pipeline.analyze(&job.image, job.use_layout)
        }))
        .map_err(|_| "推理线程异常退出".to_owned());
        let _ = job.reply.send(result);
    }
}

/// 在大栈线程里加载模型，避免主线程/运行时线程栈溢出。
///
/// 返回 `Arc`：`Pipeline` 自带约 120 KB 的模型结构体，直接按值穿栈（主线程默认
/// 只有 1 MB 栈）会栈溢出，先放堆上再传指针才安全。
fn load_pipeline() -> std::io::Result<Arc<Pipeline>> {
    let (tx, rx) = mpsc::sync_channel(1);
    thread::Builder::new()
        .name("ocr-loader".to_owned())
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let _ = tx.send(Arc::new(Pipeline::new()));
        })?;
    rx.recv().map_err(|_| std::io::Error::other("模型加载失败"))
}

fn default_workers() -> usize {
    std::env::var("LIEOCR_WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
                .clamp(1, MAX_DEFAULT_WORKERS)
        })
}
