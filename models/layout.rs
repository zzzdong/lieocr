// Generated from ONNX "models/layout.onnx" by burn-onnx
extern crate alloc;
use burn::prelude::*;
use burn::nn::BatchNorm;
use burn::nn::BatchNormConfig;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::nn::PaddingConfig2d;
use burn::nn::conv::Conv2d;
use burn::nn::conv::Conv2dConfig;
use burn::nn::pool::MaxPool2d;
use burn::nn::pool::MaxPool2dConfig;
use burn::tensor::Bytes;
use burn_store::BurnpackStore;
use burn_store::ModuleSnapshot;


#[derive(Module, Debug)]
pub struct Submodule1 {
    conv2d1: Conv2d,
    batchnormalization1: BatchNorm,
    conv2d2: Conv2d,
    batchnormalization2: BatchNorm,
    conv2d3: Conv2d,
    batchnormalization3: BatchNorm,
    maxpool2d1: MaxPool2d,
    conv2d4: Conv2d,
    batchnormalization4: BatchNorm,
    conv2d5: Conv2d,
    batchnormalization5: BatchNorm,
    conv2d6: Conv2d,
    batchnormalization6: BatchNorm,
    conv2d7: Conv2d,
    batchnormalization7: BatchNorm,
    conv2d8: Conv2d,
    batchnormalization8: BatchNorm,
    conv2d9: Conv2d,
    batchnormalization9: BatchNorm,
    conv2d10: Conv2d,
    batchnormalization10: BatchNorm,
    conv2d11: Conv2d,
    batchnormalization11: BatchNorm,
    conv2d12: Conv2d,
    batchnormalization12: BatchNorm,
    conv2d13: Conv2d,
    batchnormalization13: BatchNorm,
    conv2d14: Conv2d,
    batchnormalization14: BatchNorm,
    conv2d15: Conv2d,
    batchnormalization15: BatchNorm,
    conv2d16: Conv2d,
    batchnormalization16: BatchNorm,
    conv2d17: Conv2d,
    batchnormalization17: BatchNorm,
    conv2d18: Conv2d,
    batchnormalization18: BatchNorm,
    conv2d19: Conv2d,
    batchnormalization19: BatchNorm,
    conv2d20: Conv2d,
    batchnormalization20: BatchNorm,
    conv2d21: Conv2d,
    #[module(skip)]
    device: Device,
}
impl Submodule1 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let conv2d1 = Conv2dConfig::new([3, 32], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization1 = BatchNormConfig::new(32)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d2 = Conv2dConfig::new([32, 16], [2, 2])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(0, 0, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization2 = BatchNormConfig::new(16)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d3 = Conv2dConfig::new([16, 32], [2, 2])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(0, 0, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization3 = BatchNormConfig::new(32)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let maxpool2d1 = MaxPool2dConfig::new([2, 2])
            .with_strides([1, 1])
            .with_padding(PaddingConfig2d::Explicit(0, 0, 1, 1))
            .with_dilation([1, 1])
            .with_ceil_mode(true)
            .init();
        let conv2d4 = Conv2dConfig::new([64, 32], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization4 = BatchNormConfig::new(32)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d5 = Conv2dConfig::new([32, 48], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization5 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d6 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization6 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d7 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization7 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d8 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization8 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d9 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization9 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d10 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization10 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d11 = Conv2dConfig::new([48, 48], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization11 = BatchNormConfig::new(48)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d12 = Conv2dConfig::new([336, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization12 = BatchNormConfig::new(64)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d13 = Conv2dConfig::new([64, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization13 = BatchNormConfig::new(128)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d14 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(128)
            .with_bias(false)
            .init(device);
        let batchnormalization14 = BatchNormConfig::new(128)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d15 = Conv2dConfig::new([128, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization15 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d16 = Conv2dConfig::new([96, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization16 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d17 = Conv2dConfig::new([96, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization17 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d18 = Conv2dConfig::new([96, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization18 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d19 = Conv2dConfig::new([96, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization19 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d20 = Conv2dConfig::new([96, 96], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization20 = BatchNormConfig::new(96)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d21 = Conv2dConfig::new([704, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        Self {
            conv2d1,
            batchnormalization1,
            conv2d2,
            batchnormalization2,
            conv2d3,
            batchnormalization3,
            maxpool2d1,
            conv2d4,
            batchnormalization4,
            conv2d5,
            batchnormalization5,
            conv2d6,
            batchnormalization6,
            conv2d7,
            batchnormalization7,
            conv2d8,
            batchnormalization8,
            conv2d9,
            batchnormalization9,
            conv2d10,
            batchnormalization10,
            conv2d11,
            batchnormalization11,
            conv2d12,
            batchnormalization12,
            conv2d13,
            batchnormalization13,
            conv2d14,
            batchnormalization14,
            conv2d15,
            batchnormalization15,
            conv2d16,
            batchnormalization16,
            conv2d17,
            batchnormalization17,
            conv2d18,
            batchnormalization18,
            conv2d19,
            batchnormalization19,
            conv2d20,
            batchnormalization20,
            conv2d21,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, image: Tensor<4>) -> Tensor<4> {
        let conv2d1_out1 = self.conv2d1.forward(image);
        let batchnormalization1_out1 = self.batchnormalization1.forward(conv2d1_out1);
        let relu1_out1 = burn::tensor::activation::relu(batchnormalization1_out1);
        let conv2d2_out1 = self.conv2d2.forward(relu1_out1.clone());
        let batchnormalization2_out1 = self.batchnormalization2.forward(conv2d2_out1);
        let relu2_out1 = burn::tensor::activation::relu(batchnormalization2_out1);
        let conv2d3_out1 = self.conv2d3.forward(relu2_out1);
        let batchnormalization3_out1 = self.batchnormalization3.forward(conv2d3_out1);
        let relu3_out1 = burn::tensor::activation::relu(batchnormalization3_out1);
        let maxpool2d1_out1 = self.maxpool2d1.forward(relu1_out1);
        let concat1_out1 = burn::tensor::Tensor::cat(
            [maxpool2d1_out1, relu3_out1].into(),
            1,
        );
        let conv2d4_out1 = self.conv2d4.forward(concat1_out1);
        let batchnormalization4_out1 = self.batchnormalization4.forward(conv2d4_out1);
        let relu4_out1 = burn::tensor::activation::relu(batchnormalization4_out1);
        let conv2d5_out1 = self.conv2d5.forward(relu4_out1);
        let batchnormalization5_out1 = self.batchnormalization5.forward(conv2d5_out1);
        let relu5_out1 = burn::tensor::activation::relu(batchnormalization5_out1);
        let conv2d6_out1 = self.conv2d6.forward(relu5_out1.clone());
        let batchnormalization6_out1 = self.batchnormalization6.forward(conv2d6_out1);
        let relu6_out1 = burn::tensor::activation::relu(batchnormalization6_out1);
        let conv2d7_out1 = self.conv2d7.forward(relu6_out1.clone());
        let batchnormalization7_out1 = self.batchnormalization7.forward(conv2d7_out1);
        let relu7_out1 = burn::tensor::activation::relu(batchnormalization7_out1);
        let conv2d8_out1 = self.conv2d8.forward(relu7_out1.clone());
        let batchnormalization8_out1 = self.batchnormalization8.forward(conv2d8_out1);
        let relu8_out1 = burn::tensor::activation::relu(batchnormalization8_out1);
        let conv2d9_out1 = self.conv2d9.forward(relu8_out1.clone());
        let batchnormalization9_out1 = self.batchnormalization9.forward(conv2d9_out1);
        let relu9_out1 = burn::tensor::activation::relu(batchnormalization9_out1);
        let conv2d10_out1 = self.conv2d10.forward(relu9_out1.clone());
        let batchnormalization10_out1 = self.batchnormalization10.forward(conv2d10_out1);
        let relu10_out1 = burn::tensor::activation::relu(batchnormalization10_out1);
        let conv2d11_out1 = self.conv2d11.forward(relu10_out1.clone());
        let batchnormalization11_out1 = self.batchnormalization11.forward(conv2d11_out1);
        let relu11_out1 = burn::tensor::activation::relu(batchnormalization11_out1);
        let concat2_out1 = burn::tensor::Tensor::cat(
            [
                relu5_out1,
                relu6_out1,
                relu7_out1,
                relu8_out1,
                relu9_out1,
                relu10_out1,
                relu11_out1,
            ]
                .into(),
            1,
        );
        let conv2d12_out1 = self.conv2d12.forward(concat2_out1);
        let batchnormalization12_out1 = self.batchnormalization12.forward(conv2d12_out1);
        let relu12_out1 = burn::tensor::activation::relu(batchnormalization12_out1);
        let conv2d13_out1 = self.conv2d13.forward(relu12_out1);
        let batchnormalization13_out1 = self.batchnormalization13.forward(conv2d13_out1);
        let relu13_out1 = burn::tensor::activation::relu(batchnormalization13_out1);
        let conv2d14_out1 = self.conv2d14.forward(relu13_out1);
        let batchnormalization14_out1 = self.batchnormalization14.forward(conv2d14_out1);
        let conv2d15_out1 = self.conv2d15.forward(batchnormalization14_out1.clone());
        let batchnormalization15_out1 = self.batchnormalization15.forward(conv2d15_out1);
        let relu14_out1 = burn::tensor::activation::relu(batchnormalization15_out1);
        let conv2d16_out1 = self.conv2d16.forward(relu14_out1.clone());
        let batchnormalization16_out1 = self.batchnormalization16.forward(conv2d16_out1);
        let relu15_out1 = burn::tensor::activation::relu(batchnormalization16_out1);
        let conv2d17_out1 = self.conv2d17.forward(relu15_out1.clone());
        let batchnormalization17_out1 = self.batchnormalization17.forward(conv2d17_out1);
        let relu16_out1 = burn::tensor::activation::relu(batchnormalization17_out1);
        let conv2d18_out1 = self.conv2d18.forward(relu16_out1.clone());
        let batchnormalization18_out1 = self.batchnormalization18.forward(conv2d18_out1);
        let relu17_out1 = burn::tensor::activation::relu(batchnormalization18_out1);
        let conv2d19_out1 = self.conv2d19.forward(relu17_out1.clone());
        let batchnormalization19_out1 = self.batchnormalization19.forward(conv2d19_out1);
        let relu18_out1 = burn::tensor::activation::relu(batchnormalization19_out1);
        let conv2d20_out1 = self.conv2d20.forward(relu18_out1.clone());
        let batchnormalization20_out1 = self.batchnormalization20.forward(conv2d20_out1);
        let relu19_out1 = burn::tensor::activation::relu(batchnormalization20_out1);
        let concat3_out1 = burn::tensor::Tensor::cat(
            [
                batchnormalization14_out1,
                relu14_out1,
                relu15_out1,
                relu16_out1,
                relu17_out1,
                relu18_out1,
                relu19_out1,
            ]
                .into(),
            1,
        );
        let conv2d21_out1 = self.conv2d21.forward(concat3_out1);
        conv2d21_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule2 {
    batchnormalization21: BatchNorm,
    conv2d22: Conv2d,
    batchnormalization22: BatchNorm,
    conv2d23: Conv2d,
    batchnormalization23: BatchNorm,
    conv2d24: Conv2d,
    batchnormalization24: BatchNorm,
    conv2d25: Conv2d,
    batchnormalization25: BatchNorm,
    conv2d26: Conv2d,
    batchnormalization26: BatchNorm,
    conv2d27: Conv2d,
    batchnormalization27: BatchNorm,
    conv2d28: Conv2d,
    batchnormalization28: BatchNorm,
    conv2d29: Conv2d,
    batchnormalization29: BatchNorm,
    conv2d30: Conv2d,
    batchnormalization30: BatchNorm,
    conv2d31: Conv2d,
    batchnormalization31: BatchNorm,
    conv2d32: Conv2d,
    batchnormalization32: BatchNorm,
    conv2d33: Conv2d,
    batchnormalization33: BatchNorm,
    conv2d34: Conv2d,
    batchnormalization34: BatchNorm,
    conv2d35: Conv2d,
    batchnormalization35: BatchNorm,
    conv2d36: Conv2d,
    batchnormalization36: BatchNorm,
    conv2d37: Conv2d,
    batchnormalization37: BatchNorm,
    conv2d38: Conv2d,
    batchnormalization38: BatchNorm,
    conv2d39: Conv2d,
    batchnormalization39: BatchNorm,
    conv2d40: Conv2d,
    batchnormalization40: BatchNorm,
    conv2d41: Conv2d,
    batchnormalization41: BatchNorm,
    conv2d42: Conv2d,
    batchnormalization42: BatchNorm,
    conv2d43: Conv2d,
    batchnormalization43: BatchNorm,
    conv2d44: Conv2d,
    batchnormalization44: BatchNorm,
    conv2d45: Conv2d,
    batchnormalization45: BatchNorm,
    conv2d46: Conv2d,
    batchnormalization46: BatchNorm,
    conv2d47: Conv2d,
    batchnormalization47: BatchNorm,
    conv2d48: Conv2d,
    batchnormalization48: BatchNorm,
    conv2d49: Conv2d,
    batchnormalization49: BatchNorm,
    conv2d50: Conv2d,
    batchnormalization50: BatchNorm,
    conv2d51: Conv2d,
    batchnormalization51: BatchNorm,
    #[module(skip)]
    device: Device,
}
impl Submodule2 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let batchnormalization21 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d22 = Conv2dConfig::new([256, 512], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization22 = BatchNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d23 = Conv2dConfig::new([512, 512], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(512)
            .with_bias(false)
            .init(device);
        let batchnormalization23 = BatchNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d24 = Conv2dConfig::new([512, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization24 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d25 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization25 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d26 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization26 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d27 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization27 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d28 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization28 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d29 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization29 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d30 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization30 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d31 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization31 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d32 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization32 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d33 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization33 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d34 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization34 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d35 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization35 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d36 = Conv2dConfig::new([1664, 512], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization36 = BatchNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d37 = Conv2dConfig::new([512, 1024], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization37 = BatchNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d38 = Conv2dConfig::new([1024, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization38 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d39 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization39 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d40 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization40 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d41 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization41 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d42 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization42 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d43 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization43 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d44 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization44 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d45 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization45 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d46 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization46 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d47 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization47 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d48 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization48 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d49 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization49 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d50 = Conv2dConfig::new([2176, 512], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization50 = BatchNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d51 = Conv2dConfig::new([512, 1024], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization51 = BatchNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        Self {
            batchnormalization21,
            conv2d22,
            batchnormalization22,
            conv2d23,
            batchnormalization23,
            conv2d24,
            batchnormalization24,
            conv2d25,
            batchnormalization25,
            conv2d26,
            batchnormalization26,
            conv2d27,
            batchnormalization27,
            conv2d28,
            batchnormalization28,
            conv2d29,
            batchnormalization29,
            conv2d30,
            batchnormalization30,
            conv2d31,
            batchnormalization31,
            conv2d32,
            batchnormalization32,
            conv2d33,
            batchnormalization33,
            conv2d34,
            batchnormalization34,
            conv2d35,
            batchnormalization35,
            conv2d36,
            batchnormalization36,
            conv2d37,
            batchnormalization37,
            conv2d38,
            batchnormalization38,
            conv2d39,
            batchnormalization39,
            conv2d40,
            batchnormalization40,
            conv2d41,
            batchnormalization41,
            conv2d42,
            batchnormalization42,
            conv2d43,
            batchnormalization43,
            conv2d44,
            batchnormalization44,
            conv2d45,
            batchnormalization45,
            conv2d46,
            batchnormalization46,
            conv2d47,
            batchnormalization47,
            conv2d48,
            batchnormalization48,
            conv2d49,
            batchnormalization49,
            conv2d50,
            batchnormalization50,
            conv2d51,
            batchnormalization51,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, conv2d21_out1: Tensor<4>) -> (Tensor<4>, Tensor<4>) {
        let batchnormalization21_out1 = self.batchnormalization21.forward(conv2d21_out1);
        let relu20_out1 = burn::tensor::activation::relu(batchnormalization21_out1);
        let conv2d22_out1 = self.conv2d22.forward(relu20_out1);
        let batchnormalization22_out1 = self.batchnormalization22.forward(conv2d22_out1);
        let relu21_out1 = burn::tensor::activation::relu(batchnormalization22_out1);
        let conv2d23_out1 = self.conv2d23.forward(relu21_out1.clone());
        let batchnormalization23_out1 = self.batchnormalization23.forward(conv2d23_out1);
        let conv2d24_out1 = self.conv2d24.forward(batchnormalization23_out1.clone());
        let batchnormalization24_out1 = self.batchnormalization24.forward(conv2d24_out1);
        let conv2d25_out1 = self.conv2d25.forward(batchnormalization24_out1);
        let batchnormalization25_out1 = self.batchnormalization25.forward(conv2d25_out1);
        let relu22_out1 = burn::tensor::activation::relu(batchnormalization25_out1);
        let conv2d26_out1 = self.conv2d26.forward(relu22_out1.clone());
        let batchnormalization26_out1 = self.batchnormalization26.forward(conv2d26_out1);
        let conv2d27_out1 = self.conv2d27.forward(batchnormalization26_out1);
        let batchnormalization27_out1 = self.batchnormalization27.forward(conv2d27_out1);
        let relu23_out1 = burn::tensor::activation::relu(batchnormalization27_out1);
        let conv2d28_out1 = self.conv2d28.forward(relu23_out1.clone());
        let batchnormalization28_out1 = self.batchnormalization28.forward(conv2d28_out1);
        let conv2d29_out1 = self.conv2d29.forward(batchnormalization28_out1);
        let batchnormalization29_out1 = self.batchnormalization29.forward(conv2d29_out1);
        let relu24_out1 = burn::tensor::activation::relu(batchnormalization29_out1);
        let conv2d30_out1 = self.conv2d30.forward(relu24_out1.clone());
        let batchnormalization30_out1 = self.batchnormalization30.forward(conv2d30_out1);
        let conv2d31_out1 = self.conv2d31.forward(batchnormalization30_out1);
        let batchnormalization31_out1 = self.batchnormalization31.forward(conv2d31_out1);
        let relu25_out1 = burn::tensor::activation::relu(batchnormalization31_out1);
        let conv2d32_out1 = self.conv2d32.forward(relu25_out1.clone());
        let batchnormalization32_out1 = self.batchnormalization32.forward(conv2d32_out1);
        let conv2d33_out1 = self.conv2d33.forward(batchnormalization32_out1);
        let batchnormalization33_out1 = self.batchnormalization33.forward(conv2d33_out1);
        let relu26_out1 = burn::tensor::activation::relu(batchnormalization33_out1);
        let conv2d34_out1 = self.conv2d34.forward(relu26_out1.clone());
        let batchnormalization34_out1 = self.batchnormalization34.forward(conv2d34_out1);
        let conv2d35_out1 = self.conv2d35.forward(batchnormalization34_out1);
        let batchnormalization35_out1 = self.batchnormalization35.forward(conv2d35_out1);
        let relu27_out1 = burn::tensor::activation::relu(batchnormalization35_out1);
        let concat4_out1 = burn::tensor::Tensor::cat(
            [
                batchnormalization23_out1,
                relu22_out1,
                relu23_out1,
                relu24_out1,
                relu25_out1,
                relu26_out1,
                relu27_out1,
            ]
                .into(),
            1,
        );
        let conv2d36_out1 = self.conv2d36.forward(concat4_out1);
        let batchnormalization36_out1 = self.batchnormalization36.forward(conv2d36_out1);
        let relu28_out1 = burn::tensor::activation::relu(batchnormalization36_out1);
        let conv2d37_out1 = self.conv2d37.forward(relu28_out1);
        let batchnormalization37_out1 = self.batchnormalization37.forward(conv2d37_out1);
        let relu29_out1 = burn::tensor::activation::relu(batchnormalization37_out1);
        let conv2d38_out1 = self.conv2d38.forward(relu29_out1.clone());
        let batchnormalization38_out1 = self.batchnormalization38.forward(conv2d38_out1);
        let conv2d39_out1 = self.conv2d39.forward(batchnormalization38_out1);
        let batchnormalization39_out1 = self.batchnormalization39.forward(conv2d39_out1);
        let relu30_out1 = burn::tensor::activation::relu(batchnormalization39_out1);
        let conv2d40_out1 = self.conv2d40.forward(relu30_out1.clone());
        let batchnormalization40_out1 = self.batchnormalization40.forward(conv2d40_out1);
        let conv2d41_out1 = self.conv2d41.forward(batchnormalization40_out1);
        let batchnormalization41_out1 = self.batchnormalization41.forward(conv2d41_out1);
        let relu31_out1 = burn::tensor::activation::relu(batchnormalization41_out1);
        let conv2d42_out1 = self.conv2d42.forward(relu31_out1.clone());
        let batchnormalization42_out1 = self.batchnormalization42.forward(conv2d42_out1);
        let conv2d43_out1 = self.conv2d43.forward(batchnormalization42_out1);
        let batchnormalization43_out1 = self.batchnormalization43.forward(conv2d43_out1);
        let relu32_out1 = burn::tensor::activation::relu(batchnormalization43_out1);
        let conv2d44_out1 = self.conv2d44.forward(relu32_out1.clone());
        let batchnormalization44_out1 = self.batchnormalization44.forward(conv2d44_out1);
        let conv2d45_out1 = self.conv2d45.forward(batchnormalization44_out1);
        let batchnormalization45_out1 = self.batchnormalization45.forward(conv2d45_out1);
        let relu33_out1 = burn::tensor::activation::relu(batchnormalization45_out1);
        let conv2d46_out1 = self.conv2d46.forward(relu33_out1.clone());
        let batchnormalization46_out1 = self.batchnormalization46.forward(conv2d46_out1);
        let conv2d47_out1 = self.conv2d47.forward(batchnormalization46_out1);
        let batchnormalization47_out1 = self.batchnormalization47.forward(conv2d47_out1);
        let relu34_out1 = burn::tensor::activation::relu(batchnormalization47_out1);
        let conv2d48_out1 = self.conv2d48.forward(relu34_out1.clone());
        let batchnormalization48_out1 = self.batchnormalization48.forward(conv2d48_out1);
        let conv2d49_out1 = self.conv2d49.forward(batchnormalization48_out1);
        let batchnormalization49_out1 = self.batchnormalization49.forward(conv2d49_out1);
        let relu35_out1 = burn::tensor::activation::relu(batchnormalization49_out1);
        let concat5_out1 = burn::tensor::Tensor::cat(
            [
                relu29_out1.clone(),
                relu30_out1,
                relu31_out1,
                relu32_out1,
                relu33_out1,
                relu34_out1,
                relu35_out1,
            ]
                .into(),
            1,
        );
        let conv2d50_out1 = self.conv2d50.forward(concat5_out1);
        let batchnormalization50_out1 = self.batchnormalization50.forward(conv2d50_out1);
        let relu36_out1 = burn::tensor::activation::relu(batchnormalization50_out1);
        let conv2d51_out1 = self.conv2d51.forward(relu36_out1);
        let batchnormalization51_out1 = self.batchnormalization51.forward(conv2d51_out1);
        let relu37_out1 = burn::tensor::activation::relu(batchnormalization51_out1);
        let add1_out1 = relu37_out1.add(relu29_out1);
        (add1_out1, relu21_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule3 {
    conv2d52: Conv2d,
    batchnormalization52: BatchNorm,
    conv2d53: Conv2d,
    batchnormalization53: BatchNorm,
    conv2d54: Conv2d,
    batchnormalization54: BatchNorm,
    conv2d55: Conv2d,
    batchnormalization55: BatchNorm,
    conv2d56: Conv2d,
    batchnormalization56: BatchNorm,
    conv2d57: Conv2d,
    batchnormalization57: BatchNorm,
    conv2d58: Conv2d,
    batchnormalization58: BatchNorm,
    conv2d59: Conv2d,
    batchnormalization59: BatchNorm,
    conv2d60: Conv2d,
    batchnormalization60: BatchNorm,
    conv2d61: Conv2d,
    batchnormalization61: BatchNorm,
    conv2d62: Conv2d,
    batchnormalization62: BatchNorm,
    conv2d63: Conv2d,
    batchnormalization63: BatchNorm,
    conv2d64: Conv2d,
    batchnormalization64: BatchNorm,
    conv2d65: Conv2d,
    batchnormalization65: BatchNorm,
    conv2d66: Conv2d,
    batchnormalization66: BatchNorm,
    conv2d67: Conv2d,
    batchnormalization67: BatchNorm,
    conv2d68: Conv2d,
    batchnormalization68: BatchNorm,
    conv2d69: Conv2d,
    batchnormalization69: BatchNorm,
    conv2d70: Conv2d,
    batchnormalization70: BatchNorm,
    conv2d71: Conv2d,
    batchnormalization71: BatchNorm,
    conv2d72: Conv2d,
    batchnormalization72: BatchNorm,
    conv2d73: Conv2d,
    batchnormalization73: BatchNorm,
    conv2d74: Conv2d,
    batchnormalization74: BatchNorm,
    conv2d75: Conv2d,
    batchnormalization75: BatchNorm,
    conv2d76: Conv2d,
    batchnormalization76: BatchNorm,
    conv2d77: Conv2d,
    batchnormalization77: BatchNorm,
    conv2d78: Conv2d,
    batchnormalization78: BatchNorm,
    #[module(skip)]
    device: Device,
}
impl Submodule3 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let conv2d52 = Conv2dConfig::new([1024, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization52 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d53 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization53 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d54 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization54 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d55 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization55 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d56 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization56 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d57 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization57 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d58 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization58 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d59 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization59 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d60 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization60 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d61 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization61 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d62 = Conv2dConfig::new([192, 192], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization62 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d63 = Conv2dConfig::new([192, 192], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(192)
            .with_bias(false)
            .init(device);
        let batchnormalization63 = BatchNormConfig::new(192)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d64 = Conv2dConfig::new([2176, 512], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization64 = BatchNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d65 = Conv2dConfig::new([512, 1024], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization65 = BatchNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d66 = Conv2dConfig::new([1024, 1024], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1024)
            .with_bias(false)
            .init(device);
        let batchnormalization66 = BatchNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d67 = Conv2dConfig::new([1024, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization67 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d68 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization68 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d69 = Conv2dConfig::new([384, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization69 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d70 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization70 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d71 = Conv2dConfig::new([384, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization71 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d72 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization72 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d73 = Conv2dConfig::new([384, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization73 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d74 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization74 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d75 = Conv2dConfig::new([384, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization75 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d76 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization76 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d77 = Conv2dConfig::new([384, 384], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization77 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d78 = Conv2dConfig::new([384, 384], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(384)
            .with_bias(false)
            .init(device);
        let batchnormalization78 = BatchNormConfig::new(384)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        Self {
            conv2d52,
            batchnormalization52,
            conv2d53,
            batchnormalization53,
            conv2d54,
            batchnormalization54,
            conv2d55,
            batchnormalization55,
            conv2d56,
            batchnormalization56,
            conv2d57,
            batchnormalization57,
            conv2d58,
            batchnormalization58,
            conv2d59,
            batchnormalization59,
            conv2d60,
            batchnormalization60,
            conv2d61,
            batchnormalization61,
            conv2d62,
            batchnormalization62,
            conv2d63,
            batchnormalization63,
            conv2d64,
            batchnormalization64,
            conv2d65,
            batchnormalization65,
            conv2d66,
            batchnormalization66,
            conv2d67,
            batchnormalization67,
            conv2d68,
            batchnormalization68,
            conv2d69,
            batchnormalization69,
            conv2d70,
            batchnormalization70,
            conv2d71,
            batchnormalization71,
            conv2d72,
            batchnormalization72,
            conv2d73,
            batchnormalization73,
            conv2d74,
            batchnormalization74,
            conv2d75,
            batchnormalization75,
            conv2d76,
            batchnormalization76,
            conv2d77,
            batchnormalization77,
            conv2d78,
            batchnormalization78,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, add1_out1: Tensor<4>) -> (Tensor<4>, Tensor<4>) {
        let conv2d52_out1 = self.conv2d52.forward(add1_out1.clone());
        let batchnormalization52_out1 = self.batchnormalization52.forward(conv2d52_out1);
        let conv2d53_out1 = self.conv2d53.forward(batchnormalization52_out1);
        let batchnormalization53_out1 = self.batchnormalization53.forward(conv2d53_out1);
        let relu38_out1 = burn::tensor::activation::relu(batchnormalization53_out1);
        let conv2d54_out1 = self.conv2d54.forward(relu38_out1.clone());
        let batchnormalization54_out1 = self.batchnormalization54.forward(conv2d54_out1);
        let conv2d55_out1 = self.conv2d55.forward(batchnormalization54_out1);
        let batchnormalization55_out1 = self.batchnormalization55.forward(conv2d55_out1);
        let relu39_out1 = burn::tensor::activation::relu(batchnormalization55_out1);
        let conv2d56_out1 = self.conv2d56.forward(relu39_out1.clone());
        let batchnormalization56_out1 = self.batchnormalization56.forward(conv2d56_out1);
        let conv2d57_out1 = self.conv2d57.forward(batchnormalization56_out1);
        let batchnormalization57_out1 = self.batchnormalization57.forward(conv2d57_out1);
        let relu40_out1 = burn::tensor::activation::relu(batchnormalization57_out1);
        let conv2d58_out1 = self.conv2d58.forward(relu40_out1.clone());
        let batchnormalization58_out1 = self.batchnormalization58.forward(conv2d58_out1);
        let conv2d59_out1 = self.conv2d59.forward(batchnormalization58_out1);
        let batchnormalization59_out1 = self.batchnormalization59.forward(conv2d59_out1);
        let relu41_out1 = burn::tensor::activation::relu(batchnormalization59_out1);
        let conv2d60_out1 = self.conv2d60.forward(relu41_out1.clone());
        let batchnormalization60_out1 = self.batchnormalization60.forward(conv2d60_out1);
        let conv2d61_out1 = self.conv2d61.forward(batchnormalization60_out1);
        let batchnormalization61_out1 = self.batchnormalization61.forward(conv2d61_out1);
        let relu42_out1 = burn::tensor::activation::relu(batchnormalization61_out1);
        let conv2d62_out1 = self.conv2d62.forward(relu42_out1.clone());
        let batchnormalization62_out1 = self.batchnormalization62.forward(conv2d62_out1);
        let conv2d63_out1 = self.conv2d63.forward(batchnormalization62_out1);
        let batchnormalization63_out1 = self.batchnormalization63.forward(conv2d63_out1);
        let relu43_out1 = burn::tensor::activation::relu(batchnormalization63_out1);
        let concat6_out1 = burn::tensor::Tensor::cat(
            [
                add1_out1.clone(),
                relu38_out1,
                relu39_out1,
                relu40_out1,
                relu41_out1,
                relu42_out1,
                relu43_out1,
            ]
                .into(),
            1,
        );
        let conv2d64_out1 = self.conv2d64.forward(concat6_out1);
        let batchnormalization64_out1 = self.batchnormalization64.forward(conv2d64_out1);
        let relu44_out1 = burn::tensor::activation::relu(batchnormalization64_out1);
        let conv2d65_out1 = self.conv2d65.forward(relu44_out1);
        let batchnormalization65_out1 = self.batchnormalization65.forward(conv2d65_out1);
        let relu45_out1 = burn::tensor::activation::relu(batchnormalization65_out1);
        let add2_out1 = relu45_out1.add(add1_out1);
        let conv2d66_out1 = self.conv2d66.forward(add2_out1.clone());
        let batchnormalization66_out1 = self.batchnormalization66.forward(conv2d66_out1);
        let conv2d67_out1 = self.conv2d67.forward(batchnormalization66_out1.clone());
        let batchnormalization67_out1 = self.batchnormalization67.forward(conv2d67_out1);
        let conv2d68_out1 = self.conv2d68.forward(batchnormalization67_out1);
        let batchnormalization68_out1 = self.batchnormalization68.forward(conv2d68_out1);
        let relu46_out1 = burn::tensor::activation::relu(batchnormalization68_out1);
        let conv2d69_out1 = self.conv2d69.forward(relu46_out1.clone());
        let batchnormalization69_out1 = self.batchnormalization69.forward(conv2d69_out1);
        let conv2d70_out1 = self.conv2d70.forward(batchnormalization69_out1);
        let batchnormalization70_out1 = self.batchnormalization70.forward(conv2d70_out1);
        let relu47_out1 = burn::tensor::activation::relu(batchnormalization70_out1);
        let conv2d71_out1 = self.conv2d71.forward(relu47_out1.clone());
        let batchnormalization71_out1 = self.batchnormalization71.forward(conv2d71_out1);
        let conv2d72_out1 = self.conv2d72.forward(batchnormalization71_out1);
        let batchnormalization72_out1 = self.batchnormalization72.forward(conv2d72_out1);
        let relu48_out1 = burn::tensor::activation::relu(batchnormalization72_out1);
        let conv2d73_out1 = self.conv2d73.forward(relu48_out1.clone());
        let batchnormalization73_out1 = self.batchnormalization73.forward(conv2d73_out1);
        let conv2d74_out1 = self.conv2d74.forward(batchnormalization73_out1);
        let batchnormalization74_out1 = self.batchnormalization74.forward(conv2d74_out1);
        let relu49_out1 = burn::tensor::activation::relu(batchnormalization74_out1);
        let conv2d75_out1 = self.conv2d75.forward(relu49_out1.clone());
        let batchnormalization75_out1 = self.batchnormalization75.forward(conv2d75_out1);
        let conv2d76_out1 = self.conv2d76.forward(batchnormalization75_out1);
        let batchnormalization76_out1 = self.batchnormalization76.forward(conv2d76_out1);
        let relu50_out1 = burn::tensor::activation::relu(batchnormalization76_out1);
        let conv2d77_out1 = self.conv2d77.forward(relu50_out1.clone());
        let batchnormalization77_out1 = self.batchnormalization77.forward(conv2d77_out1);
        let conv2d78_out1 = self.conv2d78.forward(batchnormalization77_out1);
        let batchnormalization78_out1 = self.batchnormalization78.forward(conv2d78_out1);
        let relu51_out1 = burn::tensor::activation::relu(batchnormalization78_out1);
        let concat7_out1 = burn::tensor::Tensor::cat(
            [
                batchnormalization66_out1,
                relu46_out1,
                relu47_out1,
                relu48_out1,
                relu49_out1,
                relu50_out1,
                relu51_out1,
            ]
                .into(),
            1,
        );
        (concat7_out1, add2_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule4 {
    conv2d79: Conv2d,
    batchnormalization79: BatchNorm,
    conv2d80: Conv2d,
    batchnormalization80: BatchNorm,
    conv2d81: Conv2d,
    batchnormalization81: BatchNorm,
    conv2d82: Conv2d,
    batchnormalization82: BatchNorm,
    conv2d83: Conv2d,
    batchnormalization83: BatchNorm,
    constant420: burn::module::Param<Tensor<3>>,
    linear1: Linear,
    linear2: Linear,
    linear3: Linear,
    constant430: burn::module::Param<Tensor<1>>,
    constant431: burn::module::Param<Tensor<1>>,
    linear4: Linear,
    constant435: burn::module::Param<Tensor<1>>,
    constant436: burn::module::Param<Tensor<1>>,
    constant437: burn::module::Param<Tensor<1>>,
    constant438: burn::module::Param<Tensor<1>>,
    linear5: Linear,
    constant441: burn::module::Param<Tensor<1>>,
    constant442: burn::module::Param<Tensor<1>>,
    constant443: burn::module::Param<Tensor<1>>,
    linear6: Linear,
    constant446: burn::module::Param<Tensor<1>>,
    constant447: burn::module::Param<Tensor<1>>,
    constant448: burn::module::Param<Tensor<1>>,
    constant449: burn::module::Param<Tensor<1>>,
    conv2d84: Conv2d,
    batchnormalization84: BatchNorm,
    resize1: burn::nn::interpolate::Interpolate2d,
    conv2d85: Conv2d,
    batchnormalization85: BatchNorm,
    conv2d86: Conv2d,
    constant464: burn::module::Param<Tensor<4>>,
    conv2d87: Conv2d,
    constant466: burn::module::Param<Tensor<4>>,
    conv2d88: Conv2d,
    constant468: burn::module::Param<Tensor<4>>,
    conv2d89: Conv2d,
    batchnormalization86: BatchNorm,
    conv2d90: Conv2d,
    batchnormalization87: BatchNorm,
    resize2: burn::nn::interpolate::Interpolate2d,
    conv2d91: Conv2d,
    batchnormalization88: BatchNorm,
    conv2d92: Conv2d,
    constant487: burn::module::Param<Tensor<4>>,
    conv2d93: Conv2d,
    constant489: burn::module::Param<Tensor<4>>,
    conv2d94: Conv2d,
    constant491: burn::module::Param<Tensor<4>>,
    conv2d95: Conv2d,
    batchnormalization89: BatchNorm,
    conv2d96: Conv2d,
    batchnormalization90: BatchNorm,
    conv2d97: Conv2d,
    batchnormalization91: BatchNorm,
    conv2d98: Conv2d,
    constant508: burn::module::Param<Tensor<4>>,
    conv2d99: Conv2d,
    constant510: burn::module::Param<Tensor<4>>,
    conv2d100: Conv2d,
    constant512: burn::module::Param<Tensor<4>>,
    conv2d101: Conv2d,
    batchnormalization92: BatchNorm,
    conv2d102: Conv2d,
    batchnormalization93: BatchNorm,
    conv2d103: Conv2d,
    batchnormalization94: BatchNorm,
    conv2d104: Conv2d,
    constant529: burn::module::Param<Tensor<4>>,
    conv2d105: Conv2d,
    constant531: burn::module::Param<Tensor<4>>,
    conv2d106: Conv2d,
    constant533: burn::module::Param<Tensor<4>>,
    conv2d107: Conv2d,
    batchnormalization95: BatchNorm,
    conv2d108: Conv2d,
    batchnormalization96: BatchNorm,
    conv2d109: Conv2d,
    batchnormalization97: BatchNorm,
    conv2d110: Conv2d,
    batchnormalization98: BatchNorm,
    #[module(skip)]
    device: Device,
}
impl Submodule4 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let conv2d79 = Conv2dConfig::new([3328, 1024], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization79 = BatchNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d80 = Conv2dConfig::new([1024, 2048], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization80 = BatchNormConfig::new(2048)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d81 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization81 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d82 = Conv2dConfig::new([1024, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization82 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d83 = Conv2dConfig::new([2048, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization83 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let constant420: burn::module::Param<Tensor<3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                3,
            >::zeros([1, 625, 256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 625, 256].into(),
        );
        let linear1 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear2 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear3 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant430: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant431: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear4 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant435: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant436: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant437: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant438: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear5 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let constant441: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1.4142135381698608f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant442: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant443: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.5f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear6 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant446: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant447: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant448: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant449: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let conv2d84 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization84 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let resize1 = burn::nn::interpolate::Interpolate2dConfig::new()
            .with_output_size(None)
            .with_scale_factor(Some([2.0, 2.0]))
            .with_mode(burn::nn::interpolate::InterpolateMode::Nearest)
            .with_align_corners(false)
            .init();
        let conv2d85 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization85 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d86 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant464: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d87 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant466: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d88 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant468: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d89 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization86 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d90 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization87 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let resize2 = burn::nn::interpolate::Interpolate2dConfig::new()
            .with_output_size(None)
            .with_scale_factor(Some([2.0, 2.0]))
            .with_mode(burn::nn::interpolate::InterpolateMode::Nearest)
            .with_align_corners(false)
            .init();
        let conv2d91 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization88 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d92 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant487: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d93 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant489: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d94 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant491: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d95 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization89 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d96 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization90 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d97 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization91 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d98 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant508: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d99 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant510: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d100 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant512: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d101 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization92 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d102 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization93 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d103 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization94 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d104 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant529: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d105 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant531: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d106 = Conv2dConfig::new([256, 256], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant533: burn::module::Param<Tensor<4>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                4,
            >::zeros([1, 256, 1, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 256, 1, 1].into(),
        );
        let conv2d107 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization95 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d108 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization96 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d109 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization97 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        let conv2d110 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let batchnormalization98 = BatchNormConfig::new(256)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init(device);
        Self {
            conv2d79,
            batchnormalization79,
            conv2d80,
            batchnormalization80,
            conv2d81,
            batchnormalization81,
            conv2d82,
            batchnormalization82,
            conv2d83,
            batchnormalization83,
            constant420,
            linear1,
            linear2,
            linear3,
            constant430,
            constant431,
            linear4,
            constant435,
            constant436,
            constant437,
            constant438,
            linear5,
            constant441,
            constant442,
            constant443,
            linear6,
            constant446,
            constant447,
            constant448,
            constant449,
            conv2d84,
            batchnormalization84,
            resize1,
            conv2d85,
            batchnormalization85,
            conv2d86,
            constant464,
            conv2d87,
            constant466,
            conv2d88,
            constant468,
            conv2d89,
            batchnormalization86,
            conv2d90,
            batchnormalization87,
            resize2,
            conv2d91,
            batchnormalization88,
            conv2d92,
            constant487,
            conv2d93,
            constant489,
            conv2d94,
            constant491,
            conv2d95,
            batchnormalization89,
            conv2d96,
            batchnormalization90,
            conv2d97,
            batchnormalization91,
            conv2d98,
            constant508,
            conv2d99,
            constant510,
            conv2d100,
            constant512,
            conv2d101,
            batchnormalization92,
            conv2d102,
            batchnormalization93,
            conv2d103,
            batchnormalization94,
            conv2d104,
            constant529,
            conv2d105,
            constant531,
            conv2d106,
            constant533,
            conv2d107,
            batchnormalization95,
            conv2d108,
            batchnormalization96,
            conv2d109,
            batchnormalization97,
            conv2d110,
            batchnormalization98,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        concat7_out1: Tensor<4>,
        relu21_out1: Tensor<4>,
        add2_out1: Tensor<4>,
    ) -> Tensor<3> {
        let conv2d79_out1 = self.conv2d79.forward(concat7_out1);
        let batchnormalization79_out1 = self.batchnormalization79.forward(conv2d79_out1);
        let relu52_out1 = burn::tensor::activation::relu(batchnormalization79_out1);
        let conv2d80_out1 = self.conv2d80.forward(relu52_out1);
        let batchnormalization80_out1 = self.batchnormalization80.forward(conv2d80_out1);
        let relu53_out1 = burn::tensor::activation::relu(batchnormalization80_out1);
        let conv2d81_out1 = self.conv2d81.forward(relu21_out1);
        let batchnormalization81_out1 = self.batchnormalization81.forward(conv2d81_out1);
        let conv2d82_out1 = self.conv2d82.forward(add2_out1);
        let batchnormalization82_out1 = self.batchnormalization82.forward(conv2d82_out1);
        let conv2d83_out1 = self.conv2d83.forward(relu53_out1);
        let batchnormalization83_out1 = self.batchnormalization83.forward(conv2d83_out1);
        let shape1_out1: [i64; 4] = {
            let axes = &batchnormalization83_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice1_out1: [i64; 2] = shape1_out1[0..2].try_into().unwrap();
        let constant419_out1: [i64; 1] = [-1i64];
        let concat8_out1: [i64; 3usize] = [&slice1_out1[..], &constant419_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape1_out1 = batchnormalization83_out1.reshape(concat8_out1);
        let transpose1_out1 = reshape1_out1.permute([0, 2, 1]);
        let constant420_out1 = self.constant420.val();
        let add3_out1 = transpose1_out1.clone().add(constant420_out1);
        let linear1_out1 = self.linear1.forward(add3_out1.clone());
        let reshape2_out1 = linear1_out1.reshape([0, 0, 8, 32]);
        let transpose2_out1 = reshape2_out1.permute([0, 2, 1, 3]);
        let linear2_out1 = self.linear2.forward(add3_out1);
        let reshape3_out1 = linear2_out1.reshape([0, 0, 8, 32]);
        let transpose3_out1 = reshape3_out1.permute([0, 2, 1, 3]);
        let linear3_out1 = self.linear3.forward(transpose1_out1.clone());
        let reshape4_out1 = linear3_out1.reshape([0, 0, 8, 32]);
        let transpose4_out1 = reshape4_out1.permute([0, 2, 1, 3]);
        let transpose5_out1 = transpose3_out1.permute([0, 1, 3, 2]);
        let matmul4_out1 = transpose2_out1.matmul(transpose5_out1);
        let constant430_out1 = self.constant430.val();
        let mul1_out1 = matmul4_out1
            .mul((constant430_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant431_out1 = self.constant431.val();
        let add4_out1 = mul1_out1
            .add((constant431_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax1_out1 = burn::tensor::activation::softmax(add4_out1, 3);
        let matmul5_out1 = softmax1_out1.matmul(transpose4_out1);
        let transpose6_out1 = matmul5_out1.permute([0, 2, 1, 3]);
        let reshape5_out1 = transpose6_out1.reshape([0, 0, 256]);
        let linear4_out1 = self.linear4.forward(reshape5_out1);
        let add5_out1 = transpose1_out1.add(linear4_out1);
        let reducemean1_out1 = { add5_out1.clone().mean_dim(2usize) };
        let sub1_out1 = add5_out1.sub(reducemean1_out1);
        let constant435_out1 = self.constant435.val();
        let pow1_out1 = sub1_out1
            .clone()
            .powf((constant435_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean2_out1 = { pow1_out1.mean_dim(2usize) };
        let constant436_out1 = self.constant436.val();
        let add6_out1 = reducemean2_out1
            .add((constant436_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt1_out1 = add6_out1.sqrt();
        let div1_out1 = sub1_out1.div(sqrt1_out1);
        let constant437_out1 = self.constant437.val();
        let mul2_out1 = div1_out1
            .mul((constant437_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant438_out1 = self.constant438.val();
        let add7_out1 = mul2_out1
            .add((constant438_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear5_out1 = self.linear5.forward(add7_out1.clone());
        let constant441_out1 = self.constant441.val();
        let div2_out1 = linear5_out1
            .clone()
            .div((constant441_out1).unsqueeze_dims(&[0isize, 1isize]));
        let erf1_out1 = div2_out1.erf();
        let constant442_out1 = self.constant442.val();
        let add8_out1 = erf1_out1
            .add((constant442_out1).unsqueeze_dims(&[0isize, 1isize]));
        let mul3_out1 = linear5_out1.mul(add8_out1);
        let constant443_out1 = self.constant443.val();
        let mul4_out1 = mul3_out1
            .mul((constant443_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear6_out1 = self.linear6.forward(mul4_out1);
        let add9_out1 = add7_out1.add(linear6_out1);
        let reducemean3_out1 = { add9_out1.clone().mean_dim(2usize) };
        let sub2_out1 = add9_out1.sub(reducemean3_out1);
        let constant446_out1 = self.constant446.val();
        let pow2_out1 = sub2_out1
            .clone()
            .powf((constant446_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean4_out1 = { pow2_out1.mean_dim(2usize) };
        let constant447_out1 = self.constant447.val();
        let add10_out1 = reducemean4_out1
            .add((constant447_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt2_out1 = add10_out1.sqrt();
        let div3_out1 = sub2_out1.div(sqrt2_out1);
        let constant448_out1 = self.constant448.val();
        let mul5_out1 = div3_out1
            .mul((constant448_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant449_out1 = self.constant449.val();
        let add11_out1 = mul5_out1
            .add((constant449_out1).unsqueeze_dims(&[0isize, 1isize]));
        let transpose7_out1 = add11_out1.permute([0, 2, 1]);
        let reshape6_out1 = transpose7_out1.reshape([-1, 256, 25, 25]);
        let conv2d84_out1 = self.conv2d84.forward(reshape6_out1);
        let batchnormalization84_out1 = self.batchnormalization84.forward(conv2d84_out1);
        let sigmoid1_out1 = burn::tensor::activation::sigmoid(
            batchnormalization84_out1.clone(),
        );
        let mul6_out1 = batchnormalization84_out1.mul(sigmoid1_out1);
        let resize1_out1 = self.resize1.forward(mul6_out1.clone());
        let concat9_out1 = burn::tensor::Tensor::cat(
            [resize1_out1, batchnormalization82_out1].into(),
            1,
        );
        let conv2d85_out1 = self.conv2d85.forward(concat9_out1.clone());
        let batchnormalization85_out1 = self.batchnormalization85.forward(conv2d85_out1);
        let sigmoid2_out1 = burn::tensor::activation::sigmoid(
            batchnormalization85_out1.clone(),
        );
        let mul7_out1 = batchnormalization85_out1.mul(sigmoid2_out1);
        let conv2d86_out1 = self.conv2d86.forward(mul7_out1);
        let constant464_out1 = self.constant464.val();
        let add12_out1 = conv2d86_out1.add(constant464_out1);
        let sigmoid3_out1 = burn::tensor::activation::sigmoid(add12_out1.clone());
        let mul8_out1 = add12_out1.mul(sigmoid3_out1);
        let conv2d87_out1 = self.conv2d87.forward(mul8_out1);
        let constant466_out1 = self.constant466.val();
        let add13_out1 = conv2d87_out1.add(constant466_out1);
        let sigmoid4_out1 = burn::tensor::activation::sigmoid(add13_out1.clone());
        let mul9_out1 = add13_out1.mul(sigmoid4_out1);
        let conv2d88_out1 = self.conv2d88.forward(mul9_out1);
        let constant468_out1 = self.constant468.val();
        let add14_out1 = conv2d88_out1.add(constant468_out1);
        let sigmoid5_out1 = burn::tensor::activation::sigmoid(add14_out1.clone());
        let mul10_out1 = add14_out1.mul(sigmoid5_out1);
        let conv2d89_out1 = self.conv2d89.forward(concat9_out1);
        let batchnormalization86_out1 = self.batchnormalization86.forward(conv2d89_out1);
        let sigmoid6_out1 = burn::tensor::activation::sigmoid(
            batchnormalization86_out1.clone(),
        );
        let mul11_out1 = batchnormalization86_out1.mul(sigmoid6_out1);
        let add15_out1 = mul10_out1.add(mul11_out1);
        let conv2d90_out1 = self.conv2d90.forward(add15_out1);
        let batchnormalization87_out1 = self.batchnormalization87.forward(conv2d90_out1);
        let sigmoid7_out1 = burn::tensor::activation::sigmoid(
            batchnormalization87_out1.clone(),
        );
        let mul12_out1 = batchnormalization87_out1.mul(sigmoid7_out1);
        let resize2_out1 = self.resize2.forward(mul12_out1.clone());
        let concat10_out1 = burn::tensor::Tensor::cat(
            [resize2_out1, batchnormalization81_out1].into(),
            1,
        );
        let conv2d91_out1 = self.conv2d91.forward(concat10_out1.clone());
        let batchnormalization88_out1 = self.batchnormalization88.forward(conv2d91_out1);
        let sigmoid8_out1 = burn::tensor::activation::sigmoid(
            batchnormalization88_out1.clone(),
        );
        let mul13_out1 = batchnormalization88_out1.mul(sigmoid8_out1);
        let conv2d92_out1 = self.conv2d92.forward(mul13_out1);
        let constant487_out1 = self.constant487.val();
        let add16_out1 = conv2d92_out1.add(constant487_out1);
        let sigmoid9_out1 = burn::tensor::activation::sigmoid(add16_out1.clone());
        let mul14_out1 = add16_out1.mul(sigmoid9_out1);
        let conv2d93_out1 = self.conv2d93.forward(mul14_out1);
        let constant489_out1 = self.constant489.val();
        let add17_out1 = conv2d93_out1.add(constant489_out1);
        let sigmoid10_out1 = burn::tensor::activation::sigmoid(add17_out1.clone());
        let mul15_out1 = add17_out1.mul(sigmoid10_out1);
        let conv2d94_out1 = self.conv2d94.forward(mul15_out1);
        let constant491_out1 = self.constant491.val();
        let add18_out1 = conv2d94_out1.add(constant491_out1);
        let sigmoid11_out1 = burn::tensor::activation::sigmoid(add18_out1.clone());
        let mul16_out1 = add18_out1.mul(sigmoid11_out1);
        let conv2d95_out1 = self.conv2d95.forward(concat10_out1);
        let batchnormalization89_out1 = self.batchnormalization89.forward(conv2d95_out1);
        let sigmoid12_out1 = burn::tensor::activation::sigmoid(
            batchnormalization89_out1.clone(),
        );
        let mul17_out1 = batchnormalization89_out1.mul(sigmoid12_out1);
        let add19_out1 = mul16_out1.add(mul17_out1);
        let conv2d96_out1 = self.conv2d96.forward(add19_out1.clone());
        let batchnormalization90_out1 = self.batchnormalization90.forward(conv2d96_out1);
        let sigmoid13_out1 = burn::tensor::activation::sigmoid(
            batchnormalization90_out1.clone(),
        );
        let mul18_out1 = batchnormalization90_out1.mul(sigmoid13_out1);
        let concat11_out1 = burn::tensor::Tensor::cat(
            [mul18_out1, mul12_out1].into(),
            1,
        );
        let conv2d97_out1 = self.conv2d97.forward(concat11_out1.clone());
        let batchnormalization91_out1 = self.batchnormalization91.forward(conv2d97_out1);
        let sigmoid14_out1 = burn::tensor::activation::sigmoid(
            batchnormalization91_out1.clone(),
        );
        let mul19_out1 = batchnormalization91_out1.mul(sigmoid14_out1);
        let conv2d98_out1 = self.conv2d98.forward(mul19_out1);
        let constant508_out1 = self.constant508.val();
        let add20_out1 = conv2d98_out1.add(constant508_out1);
        let sigmoid15_out1 = burn::tensor::activation::sigmoid(add20_out1.clone());
        let mul20_out1 = add20_out1.mul(sigmoid15_out1);
        let conv2d99_out1 = self.conv2d99.forward(mul20_out1);
        let constant510_out1 = self.constant510.val();
        let add21_out1 = conv2d99_out1.add(constant510_out1);
        let sigmoid16_out1 = burn::tensor::activation::sigmoid(add21_out1.clone());
        let mul21_out1 = add21_out1.mul(sigmoid16_out1);
        let conv2d100_out1 = self.conv2d100.forward(mul21_out1);
        let constant512_out1 = self.constant512.val();
        let add22_out1 = conv2d100_out1.add(constant512_out1);
        let sigmoid17_out1 = burn::tensor::activation::sigmoid(add22_out1.clone());
        let mul22_out1 = add22_out1.mul(sigmoid17_out1);
        let conv2d101_out1 = self.conv2d101.forward(concat11_out1);
        let batchnormalization92_out1 = self
            .batchnormalization92
            .forward(conv2d101_out1);
        let sigmoid18_out1 = burn::tensor::activation::sigmoid(
            batchnormalization92_out1.clone(),
        );
        let mul23_out1 = batchnormalization92_out1.mul(sigmoid18_out1);
        let add23_out1 = mul22_out1.add(mul23_out1);
        let conv2d102_out1 = self.conv2d102.forward(add23_out1.clone());
        let batchnormalization93_out1 = self
            .batchnormalization93
            .forward(conv2d102_out1);
        let sigmoid19_out1 = burn::tensor::activation::sigmoid(
            batchnormalization93_out1.clone(),
        );
        let mul24_out1 = batchnormalization93_out1.mul(sigmoid19_out1);
        let concat12_out1 = burn::tensor::Tensor::cat([mul24_out1, mul6_out1].into(), 1);
        let conv2d103_out1 = self.conv2d103.forward(concat12_out1.clone());
        let batchnormalization94_out1 = self
            .batchnormalization94
            .forward(conv2d103_out1);
        let sigmoid20_out1 = burn::tensor::activation::sigmoid(
            batchnormalization94_out1.clone(),
        );
        let mul25_out1 = batchnormalization94_out1.mul(sigmoid20_out1);
        let conv2d104_out1 = self.conv2d104.forward(mul25_out1);
        let constant529_out1 = self.constant529.val();
        let add24_out1 = conv2d104_out1.add(constant529_out1);
        let sigmoid21_out1 = burn::tensor::activation::sigmoid(add24_out1.clone());
        let mul26_out1 = add24_out1.mul(sigmoid21_out1);
        let conv2d105_out1 = self.conv2d105.forward(mul26_out1);
        let constant531_out1 = self.constant531.val();
        let add25_out1 = conv2d105_out1.add(constant531_out1);
        let sigmoid22_out1 = burn::tensor::activation::sigmoid(add25_out1.clone());
        let mul27_out1 = add25_out1.mul(sigmoid22_out1);
        let conv2d106_out1 = self.conv2d106.forward(mul27_out1);
        let constant533_out1 = self.constant533.val();
        let add26_out1 = conv2d106_out1.add(constant533_out1);
        let sigmoid23_out1 = burn::tensor::activation::sigmoid(add26_out1.clone());
        let mul28_out1 = add26_out1.mul(sigmoid23_out1);
        let conv2d107_out1 = self.conv2d107.forward(concat12_out1);
        let batchnormalization95_out1 = self
            .batchnormalization95
            .forward(conv2d107_out1);
        let sigmoid24_out1 = burn::tensor::activation::sigmoid(
            batchnormalization95_out1.clone(),
        );
        let mul29_out1 = batchnormalization95_out1.mul(sigmoid24_out1);
        let add27_out1 = mul28_out1.add(mul29_out1);
        let conv2d108_out1 = self.conv2d108.forward(add19_out1);
        let batchnormalization96_out1 = self
            .batchnormalization96
            .forward(conv2d108_out1);
        let conv2d109_out1 = self.conv2d109.forward(add23_out1);
        let batchnormalization97_out1 = self
            .batchnormalization97
            .forward(conv2d109_out1);
        let conv2d110_out1 = self.conv2d110.forward(add27_out1);
        let batchnormalization98_out1 = self
            .batchnormalization98
            .forward(conv2d110_out1);
        let shape2_out1: [i64; 4] = {
            let axes = &batchnormalization96_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice2_out1: [i64; 2] = shape2_out1[0..2].try_into().unwrap();
        let constant557_out1: [i64; 1] = [-1i64];
        let concat13_out1: [i64; 3usize] = [&slice2_out1[..], &constant557_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape7_out1 = batchnormalization96_out1.reshape(concat13_out1);
        let transpose8_out1 = reshape7_out1.permute([0, 2, 1]);
        let shape3_out1: [i64; 4] = {
            let axes = &batchnormalization97_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice3_out1: [i64; 2] = shape3_out1[0..2].try_into().unwrap();
        let constant561_out1: [i64; 1] = [-1i64];
        let concat14_out1: [i64; 3usize] = [&slice3_out1[..], &constant561_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape8_out1 = batchnormalization97_out1.reshape(concat14_out1);
        let transpose9_out1 = reshape8_out1.permute([0, 2, 1]);
        let shape4_out1: [i64; 4] = {
            let axes = &batchnormalization98_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice4_out1: [i64; 2] = shape4_out1[0..2].try_into().unwrap();
        let constant565_out1: [i64; 1] = [-1i64];
        let concat15_out1: [i64; 3usize] = [&slice4_out1[..], &constant565_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape9_out1 = batchnormalization98_out1.reshape(concat15_out1);
        let transpose10_out1 = reshape9_out1.permute([0, 2, 1]);
        let concat16_out1 = burn::tensor::Tensor::cat(
            [transpose8_out1, transpose9_out1, transpose10_out1].into(),
            1,
        );
        concat16_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule5 {
    constant571: burn::module::Param<Tensor<1>>,
    constant572: burn::module::Param<Tensor<1>>,
    constant573: burn::module::Param<Tensor<3>>,
    constant574: burn::module::Param<Tensor<1>>,
    constant575: burn::module::Param<Tensor<3>>,
    linear7: Linear,
    constant578: burn::module::Param<Tensor<1>>,
    constant579: burn::module::Param<Tensor<1>>,
    constant580: burn::module::Param<Tensor<1>>,
    constant581: burn::module::Param<Tensor<1>>,
    linear8: Linear,
    linear9: Linear,
    linear10: Linear,
    linear11: Linear,
    constant590: burn::module::Param<Tensor<3>>,
    linear12: Linear,
    linear13: Linear,
    linear14: Linear,
    linear15: Linear,
    linear16: Linear,
    constant612: burn::module::Param<Tensor<1>>,
    constant613: burn::module::Param<Tensor<1>>,
    linear17: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule5 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant571: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant572: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant573: burn::module::Param<Tensor<3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                3,
            >::zeros([1, 13125, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 13125, 1].into(),
        );
        let constant574: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant575: burn::module::Param<Tensor<3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                3,
            >::zeros([1, 13125, 1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 13125, 1].into(),
        );
        let linear7 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant578: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant579: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant580: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant581: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear8 = LinearConfig::new(256, 20).with_bias(true).init(device);
        let linear9 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear10 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear11 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant590: burn::module::Param<Tensor<3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                3,
            >::zeros([1, 13125, 4], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1, 13125, 4].into(),
        );
        let linear12 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear13 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear14 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear15 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear16 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant612: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant613: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear17 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant571,
            constant572,
            constant573,
            constant574,
            constant575,
            linear7,
            constant578,
            constant579,
            constant580,
            constant581,
            linear8,
            linear9,
            linear10,
            linear11,
            constant590,
            linear12,
            linear13,
            linear14,
            linear15,
            linear16,
            constant612,
            constant613,
            linear17,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        concat16_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let shape5_out1: [i64; 3] = {
            let axes = &concat16_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice5_out1: [i64; 1] = shape5_out1[0..1].try_into().unwrap();
        let squeeze1_out1 = slice5_out1[0] as i64;
        let shape6_out1: [i64; 3] = {
            let axes = &concat16_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let constant571_out1 = self.constant571.val();
        let expand1_out1 = {
            let onnx_shape: [i64; 3usize] = shape6_out1;
            let input_dims = constant571_out1.dims();
            let mut shape = onnx_shape;
            #[allow(clippy::needless_range_loop)]
            for i in 0..1usize {
                let dim_offset = 2usize + i;
                if shape[dim_offset] == 1 && input_dims[i] > 1 {
                    shape[dim_offset] = input_dims[i] as i64;
                }
            }
            constant571_out1.expand(shape)
        };
        let constant572_out1 = self.constant572.val();
        let add28_out1 = expand1_out1
            .add((constant572_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant573_out1 = self.constant573.val();
        let add29_out1 = add28_out1.add(constant573_out1);
        let add30_out1 = concat16_out1.add(add29_out1.clone());
        let constant574_out1 = self.constant574.val();
        let add31_out1 = (constant574_out1)
            .unsqueeze_dims(&[0isize, 1isize])
            .add(add29_out1.clone());
        let constant575_out1 = self.constant575.val();
        let add32_out1 = constant575_out1.add(add29_out1);
        let cast1_out1 = add32_out1.bool();
        let where1_out1 = add31_out1.mask_where(cast1_out1, add30_out1);
        let linear7_out1 = self.linear7.forward(where1_out1);
        let reducemean5_out1 = { linear7_out1.clone().mean_dim(2usize) };
        let sub3_out1 = linear7_out1.sub(reducemean5_out1);
        let constant578_out1 = self.constant578.val();
        let pow3_out1 = sub3_out1
            .clone()
            .powf((constant578_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean6_out1 = { pow3_out1.mean_dim(2usize) };
        let constant579_out1 = self.constant579.val();
        let add33_out1 = reducemean6_out1
            .add((constant579_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt3_out1 = add33_out1.sqrt();
        let div4_out1 = sub3_out1.div(sqrt3_out1);
        let constant580_out1 = self.constant580.val();
        let mul30_out1 = div4_out1
            .mul((constant580_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant581_out1 = self.constant581.val();
        let add34_out1 = mul30_out1
            .add((constant581_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear8_out1 = self.linear8.forward(add34_out1.clone());
        let linear9_out1 = self.linear9.forward(add34_out1.clone());
        let relu54_out1 = burn::tensor::activation::relu(linear9_out1);
        let linear10_out1 = self.linear10.forward(relu54_out1);
        let relu55_out1 = burn::tensor::activation::relu(linear10_out1);
        let linear11_out1 = self.linear11.forward(relu55_out1);
        let constant590_out1 = self.constant590.val();
        let add35_out1 = linear11_out1.add(constant590_out1);
        let reducemax1_out1 = {
            linear8_out1.max_dim(2usize).squeeze_dims::<2usize>(&[2])
        };
        let (topk1_out1, topk1_out2) = {
            let (values, indices) = reducemax1_out1.topk_with_indices(300, 1);
            (values, indices.cast(burn::tensor::DType::I64))
        };
        let range1_out1 = {
            let start = 0i64;
            let delta = 1i64;
            assert!(delta != 0);
            let n = (((squeeze1_out1 as i128 - start as i128) as f64) / delta as f64)
                .ceil()
                .max(0.0) as i64;
            Tensor::arange(0..n, &self.device)
                .cast(burn::tensor::DType::I64)
                .mul_scalar(delta)
                .add_scalar(start)
        };
        let unsqueeze1_out1: Tensor<2, Int> = range1_out1.unsqueeze_dims::<2>(&[-1]);
        let tile1_out1 = unsqueeze1_out1.repeat(&[1, 300]);
        let unsqueeze2_out1: Tensor<3, Int> = tile1_out1.unsqueeze_dims::<3>(&[2]);
        let unsqueeze3_out1: Tensor<3, Int> = topk1_out2.unsqueeze_dims::<3>(&[2]);
        let concat17_out1 = burn::tensor::Tensor::cat(
            [unsqueeze2_out1, unsqueeze3_out1].into(),
            2,
        );
        let gathernd1_out1 = {
            let data_dims = add35_out1.dims();
            let indices_i64 = concat17_out1.clone().cast(burn::tensor::DType::I64);
            let idx_dims = indices_i64.dims();
            let k = idx_dims[3 - 1];
            let mut dim_sizes: alloc::vec::Vec<i64> = alloc::vec::Vec::with_capacity(k);
            for i in 0..k {
                dim_sizes.push(data_dims[0 + i] as i64);
            }
            let mut bcast_shape = [1usize; 3];
            bcast_shape[3 - 1] = k;
            let dims_tensor = Tensor::<
                1,
                Int,
            >::from_data(
                    burn::tensor::TensorData::from(dim_sizes.as_slice()),
                    (&self.device, burn::tensor::DType::I64),
                )
                .reshape(bcast_shape);
            let negative = indices_i64.clone().lower_elem(0i64);
            let corrected = indices_i64.clone() + dims_tensor;
            let indices_norm = indices_i64.mask_where(negative, corrected);
            let aug_indices = indices_norm;
            add35_out1.gather_nd(aug_indices)
        };
        let gathernd2_out1 = {
            let data_dims = add34_out1.dims();
            let indices_i64 = concat17_out1.cast(burn::tensor::DType::I64);
            let idx_dims = indices_i64.dims();
            let k = idx_dims[3 - 1];
            let mut dim_sizes: alloc::vec::Vec<i64> = alloc::vec::Vec::with_capacity(k);
            for i in 0..k {
                dim_sizes.push(data_dims[0 + i] as i64);
            }
            let mut bcast_shape = [1usize; 3];
            bcast_shape[3 - 1] = k;
            let dims_tensor = Tensor::<
                1,
                Int,
            >::from_data(
                    burn::tensor::TensorData::from(dim_sizes.as_slice()),
                    (&self.device, burn::tensor::DType::I64),
                )
                .reshape(bcast_shape);
            let negative = indices_i64.clone().lower_elem(0i64);
            let corrected = indices_i64.clone() + dims_tensor;
            let indices_norm = indices_i64.mask_where(negative, corrected);
            let aug_indices = indices_norm;
            add34_out1.gather_nd(aug_indices)
        };
        let sigmoid25_out1 = burn::tensor::activation::sigmoid(gathernd1_out1);
        let unsqueeze4_out1: Tensor<4> = sigmoid25_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear12_out1 = self.linear12.forward(sigmoid25_out1.clone());
        let relu56_out1 = burn::tensor::activation::relu(linear12_out1);
        let linear13_out1 = self.linear13.forward(relu56_out1);
        let add36_out1 = gathernd2_out1.clone().add(linear13_out1.clone());
        let linear14_out1 = self.linear14.forward(add36_out1.clone());
        let reshape10_out1 = linear14_out1.reshape([0, 0, 8, 32]);
        let transpose11_out1 = reshape10_out1.permute([0, 2, 1, 3]);
        let linear15_out1 = self.linear15.forward(add36_out1);
        let reshape11_out1 = linear15_out1.reshape([0, 0, 8, 32]);
        let transpose12_out1 = reshape11_out1.permute([0, 2, 1, 3]);
        let linear16_out1 = self.linear16.forward(gathernd2_out1.clone());
        let reshape12_out1 = linear16_out1.reshape([0, 0, 8, 32]);
        let transpose13_out1 = reshape12_out1.permute([0, 2, 1, 3]);
        let transpose14_out1 = transpose12_out1.permute([0, 1, 3, 2]);
        let matmul19_out1 = transpose11_out1.matmul(transpose14_out1);
        let constant612_out1 = self.constant612.val();
        let mul31_out1 = matmul19_out1
            .mul((constant612_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant613_out1 = self.constant613.val();
        let add37_out1 = mul31_out1
            .add((constant613_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax2_out1 = burn::tensor::activation::softmax(add37_out1, 3);
        let matmul20_out1 = softmax2_out1.matmul(transpose13_out1);
        let transpose15_out1 = matmul20_out1.permute([0, 2, 1, 3]);
        let reshape13_out1 = transpose15_out1.reshape([0, 0, 256]);
        let linear17_out1 = self.linear17.forward(reshape13_out1);
        let add38_out1 = gathernd2_out1.add(linear17_out1);
        (add38_out1, linear13_out1, unsqueeze4_out1, sigmoid25_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule6 {
    constant617: burn::module::Param<Tensor<1>>,
    constant618: burn::module::Param<Tensor<1>>,
    constant619: burn::module::Param<Tensor<1>>,
    constant620: burn::module::Param<Tensor<1>>,
    linear18: Linear,
    linear19: Linear,
    linear20: Linear,
    constant660: burn::module::Param<Tensor<1>>,
    constant661: burn::module::Param<Tensor<1>>,
    constant667: burn::module::Param<Tensor<1>>,
    constant668: burn::module::Param<Tensor<1>>,
    constant675: burn::module::Param<Tensor<1>>,
    constant676: burn::module::Param<Tensor<1>>,
    constant677: burn::module::Param<Tensor<1>>,
    constant678: burn::module::Param<Tensor<1>>,
    constant683: burn::module::Param<Tensor<1>>,
    constant684: burn::module::Param<Tensor<1>>,
    constant703: burn::module::Param<Tensor<1>>,
    constant704: burn::module::Param<Tensor<1>>,
    constant723: burn::module::Param<Tensor<1>>,
    constant724: burn::module::Param<Tensor<1>>,
    constant739: burn::module::Param<Tensor<1>>,
    constant740: burn::module::Param<Tensor<1>>,
    linear21: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule6 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant617: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant618: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant619: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant620: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear18 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear19 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear20 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant660: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant661: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant667: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant668: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant675: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant676: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant677: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant678: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant683: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant684: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant703: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant704: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant723: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant724: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant739: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant740: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear21 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant617,
            constant618,
            constant619,
            constant620,
            linear18,
            linear19,
            linear20,
            constant660,
            constant661,
            constant667,
            constant668,
            constant675,
            constant676,
            constant677,
            constant678,
            constant683,
            constant684,
            constant703,
            constant704,
            constant723,
            constant724,
            constant739,
            constant740,
            linear21,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add38_out1: Tensor<3>,
        linear13_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze4_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean7_out1 = { add38_out1.clone().mean_dim(2usize) };
        let sub4_out1 = add38_out1.sub(reducemean7_out1);
        let constant617_out1 = self.constant617.val();
        let pow4_out1 = sub4_out1
            .clone()
            .powf((constant617_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean8_out1 = { pow4_out1.mean_dim(2usize) };
        let constant618_out1 = self.constant618.val();
        let add39_out1 = reducemean8_out1
            .add((constant618_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt4_out1 = add39_out1.sqrt();
        let div5_out1 = sub4_out1.div(sqrt4_out1);
        let constant619_out1 = self.constant619.val();
        let mul32_out1 = div5_out1
            .mul((constant619_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant620_out1 = self.constant620.val();
        let add40_out1 = mul32_out1
            .add((constant620_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add41_out1 = add40_out1.clone().add(linear13_out1);
        let shape7_out1: [i64; 3] = {
            let axes = &add41_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice6_out1: [i64; 1] = shape7_out1[0..1].try_into().unwrap();
        let squeeze2_out1 = slice6_out1[0] as i64;
        let linear18_out1 = self.linear18.forward(concat16_out1);
        let reshape14_out1 = squeeze2_out1;
        let unsqueeze5_out1 = [reshape14_out1 as i64];
        let constant630_out1: [i64; 1] = [13125i64];
        let constant631_out1: [i64; 1] = [8i64];
        let constant632_out1: [i64; 1] = [32i64];
        let concat18_out1: [i64; 4usize] = [
            &unsqueeze5_out1[..],
            &constant630_out1[..],
            &constant631_out1[..],
            &constant632_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape15_out1 = linear18_out1.reshape(concat18_out1);
        let linear19_out1 = self.linear19.forward(add41_out1.clone());
        let reshape16_out1 = squeeze2_out1;
        let unsqueeze6_out1 = [reshape16_out1 as i64];
        let constant637_out1: [i64; 1] = [300i64];
        let constant638_out1: [i64; 1] = [8i64];
        let constant639_out1: [i64; 1] = [3i64];
        let constant640_out1: [i64; 1] = [4i64];
        let constant641_out1: [i64; 1] = [2i64];
        let concat19_out1: [i64; 6usize] = [
            &unsqueeze6_out1[..],
            &constant637_out1[..],
            &constant638_out1[..],
            &constant639_out1[..],
            &constant640_out1[..],
            &constant641_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape17_out1 = linear19_out1.reshape(concat19_out1);
        let linear20_out1 = self.linear20.forward(add41_out1);
        let reshape18_out1 = squeeze2_out1;
        let unsqueeze7_out1 = [reshape18_out1 as i64];
        let constant646_out1: [i64; 1] = [300i64];
        let constant647_out1: [i64; 1] = [8i64];
        let constant648_out1: [i64; 1] = [12i64];
        let concat20_out1: [i64; 4usize] = [
            &unsqueeze7_out1[..],
            &constant646_out1[..],
            &constant647_out1[..],
            &constant648_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape19_out1 = linear20_out1.reshape(concat20_out1);
        let softmax3_out1 = burn::tensor::activation::softmax(reshape19_out1, 3);
        let reshape20_out1 = squeeze2_out1;
        let unsqueeze8_out1 = [reshape20_out1 as i64];
        let constant651_out1: [i64; 1] = [300i64];
        let constant652_out1: [i64; 1] = [8i64];
        let constant653_out1: [i64; 1] = [3i64];
        let constant654_out1: [i64; 1] = [4i64];
        let concat21_out1: [i64; 5usize] = [
            &unsqueeze8_out1[..],
            &constant651_out1[..],
            &constant652_out1[..],
            &constant653_out1[..],
            &constant654_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape21_out1 = softmax3_out1.reshape(concat21_out1);
        let slice7_out1 = unsqueeze4_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze9_out1: Tensor<6> = slice7_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant660_out1 = self.constant660.val();
        let mul33_out1 = reshape17_out1
            .mul(
                (constant660_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant661_out1 = self.constant661.val();
        let add42_out1 = mul33_out1
            .add(
                (constant661_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice8_out1 = unsqueeze4_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze10_out1: Tensor<6> = slice8_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul34_out1 = add42_out1.mul(unsqueeze10_out1);
        let constant667_out1 = self.constant667.val();
        let mul35_out1 = mul34_out1
            .mul(
                (constant667_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant668_out1 = self.constant668.val();
        let add43_out1 = mul35_out1
            .add(
                (constant668_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add44_out1 = unsqueeze9_out1.add(add43_out1);
        let shape8_out1: [i64; 4] = {
            let axes = &reshape15_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice9_out1: [i64; 1] = shape8_out1[0..1].try_into().unwrap();
        let squeeze3_out1 = slice9_out1[0] as i64;
        let split_tensors = reshape15_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split1_out1, split1_out2, split1_out3] = split_tensors.try_into().unwrap();
        let constant675_out1 = self.constant675.val();
        let mul36_out1 = add44_out1
            .mul(
                (constant675_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant676_out1 = self.constant676.val();
        let add45_out1 = mul36_out1
            .add(
                (constant676_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant677_out1 = self.constant677.val();
        let mul37_out1 = add45_out1
            .mul(
                (constant677_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant678_out1 = self.constant678.val();
        let add46_out1 = mul37_out1
            .add(
                (constant678_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape9_out1: [i64; 4] = {
            let axes = &split1_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice10_out1: [i64; 2] = shape9_out1[0..2].try_into().unwrap();
        let constant682_out1: [i64; 1] = [-1i64];
        let concat22_out1: [i64; 3usize] = [&slice10_out1[..], &constant682_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape22_out1 = split1_out1.reshape(concat22_out1);
        let transpose16_out1 = reshape22_out1.permute([0, 2, 1]);
        let cast2_out1 = squeeze3_out1 as f32;
        let constant683_out1 = self.constant683.val();
        let mul38_out1 = cast2_out1 * constant683_out1;
        let constant684_out1 = self.constant684.val();
        let add47_out1 = mul38_out1.add(constant684_out1);
        let cast3_out1 = add47_out1.int().cast(burn::tensor::DType::I64);
        let reshape23_out1 = cast3_out1.into_scalar::<i64>();
        let unsqueeze11_out1 = [reshape23_out1 as i64];
        let constant687_out1: [i64; 1] = [32i64];
        let constant688_out1: [i64; 1] = [100i64];
        let constant689_out1: [i64; 1] = [100i64];
        let concat23_out1: [i64; 4usize] = [
            &unsqueeze11_out1[..],
            &constant687_out1[..],
            &constant688_out1[..],
            &constant689_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape24_out1 = transpose16_out1.reshape(concat23_out1);
        let slice11_out1 = add46_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze4_out1 = slice11_out1.squeeze_dims::<5>(&[3]);
        let transpose17_out1 = squeeze4_out1.permute([0, 2, 1, 3, 4]);
        let shape10_out1: [i64; 5] = {
            let axes = &transpose17_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice12_out1: [i64; 3] = shape10_out1[2..5].try_into().unwrap();
        let constant698_out1: [i64; 1] = [-1i64];
        let concat24_out1: [i64; 4usize] = [&constant698_out1[..], &slice12_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape25_out1 = transpose17_out1.reshape(concat24_out1);
        let gridsample1_out1 = reshape24_out1
            .grid_sample_2d(
                reshape25_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape11_out1: [i64; 4] = {
            let axes = &split1_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice13_out1: [i64; 2] = shape11_out1[0..2].try_into().unwrap();
        let constant702_out1: [i64; 1] = [-1i64];
        let concat25_out1: [i64; 3usize] = [&slice13_out1[..], &constant702_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape26_out1 = split1_out2.reshape(concat25_out1);
        let transpose18_out1 = reshape26_out1.permute([0, 2, 1]);
        let cast4_out1 = squeeze3_out1 as f32;
        let constant703_out1 = self.constant703.val();
        let mul39_out1 = cast4_out1 * constant703_out1;
        let constant704_out1 = self.constant704.val();
        let add48_out1 = mul39_out1.add(constant704_out1);
        let cast5_out1 = add48_out1.int().cast(burn::tensor::DType::I64);
        let reshape27_out1 = cast5_out1.into_scalar::<i64>();
        let unsqueeze12_out1 = [reshape27_out1 as i64];
        let constant707_out1: [i64; 1] = [32i64];
        let constant708_out1: [i64; 1] = [50i64];
        let constant709_out1: [i64; 1] = [50i64];
        let concat26_out1: [i64; 4usize] = [
            &unsqueeze12_out1[..],
            &constant707_out1[..],
            &constant708_out1[..],
            &constant709_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape28_out1 = transpose18_out1.reshape(concat26_out1);
        let slice14_out1 = add46_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze5_out1 = slice14_out1.squeeze_dims::<5>(&[3]);
        let transpose19_out1 = squeeze5_out1.permute([0, 2, 1, 3, 4]);
        let shape12_out1: [i64; 5] = {
            let axes = &transpose19_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice15_out1: [i64; 3] = shape12_out1[2..5].try_into().unwrap();
        let constant718_out1: [i64; 1] = [-1i64];
        let concat27_out1: [i64; 4usize] = [&constant718_out1[..], &slice15_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape29_out1 = transpose19_out1.reshape(concat27_out1);
        let gridsample2_out1 = reshape28_out1
            .grid_sample_2d(
                reshape29_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape13_out1: [i64; 4] = {
            let axes = &split1_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice16_out1: [i64; 2] = shape13_out1[0..2].try_into().unwrap();
        let constant722_out1: [i64; 1] = [-1i64];
        let concat28_out1: [i64; 3usize] = [&slice16_out1[..], &constant722_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape30_out1 = split1_out3.reshape(concat28_out1);
        let transpose20_out1 = reshape30_out1.permute([0, 2, 1]);
        let cast6_out1 = squeeze3_out1 as f32;
        let constant723_out1 = self.constant723.val();
        let mul40_out1 = cast6_out1 * constant723_out1;
        let constant724_out1 = self.constant724.val();
        let add49_out1 = mul40_out1.add(constant724_out1);
        let cast7_out1 = add49_out1.int().cast(burn::tensor::DType::I64);
        let reshape31_out1 = cast7_out1.into_scalar::<i64>();
        let unsqueeze13_out1 = [reshape31_out1 as i64];
        let constant727_out1: [i64; 1] = [32i64];
        let constant728_out1: [i64; 1] = [25i64];
        let constant729_out1: [i64; 1] = [25i64];
        let concat29_out1: [i64; 4usize] = [
            &unsqueeze13_out1[..],
            &constant727_out1[..],
            &constant728_out1[..],
            &constant729_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape32_out1 = transpose20_out1.reshape(concat29_out1);
        let slice17_out1 = add46_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze6_out1 = slice17_out1.squeeze_dims::<5>(&[3]);
        let transpose21_out1 = squeeze6_out1.permute([0, 2, 1, 3, 4]);
        let shape14_out1: [i64; 5] = {
            let axes = &transpose21_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice18_out1: [i64; 3] = shape14_out1[2..5].try_into().unwrap();
        let constant738_out1: [i64; 1] = [-1i64];
        let concat30_out1: [i64; 4usize] = [&constant738_out1[..], &slice18_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape33_out1 = transpose21_out1.reshape(concat30_out1);
        let gridsample3_out1 = reshape32_out1
            .grid_sample_2d(
                reshape33_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose22_out1 = reshape21_out1.permute([0, 2, 1, 3, 4]);
        let cast8_out1 = squeeze3_out1 as f32;
        let constant739_out1 = self.constant739.val();
        let mul41_out1 = cast8_out1 * constant739_out1;
        let constant740_out1 = self.constant740.val();
        let add50_out1 = mul41_out1.add(constant740_out1);
        let cast9_out1 = add50_out1.int().cast(burn::tensor::DType::I64);
        let reshape34_out1 = cast9_out1.into_scalar::<i64>();
        let unsqueeze14_out1 = [reshape34_out1 as i64];
        let constant743_out1: [i64; 1] = [1i64];
        let constant744_out1: [i64; 1] = [300i64];
        let constant745_out1: [i64; 1] = [12i64];
        let concat31_out1: [i64; 4usize] = [
            &unsqueeze14_out1[..],
            &constant743_out1[..],
            &constant744_out1[..],
            &constant745_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape35_out1 = transpose22_out1.reshape(concat31_out1);
        let unsqueeze15_out1: Tensor<5> = gridsample1_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze16_out1: Tensor<5> = gridsample2_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze17_out1: Tensor<5> = gridsample3_out1.unsqueeze_dims::<5>(&[3]);
        let concat32_out1 = burn::tensor::Tensor::cat(
            [unsqueeze15_out1, unsqueeze16_out1, unsqueeze17_out1].into(),
            3,
        );
        let shape15_out1: [i64; 5] = {
            let axes = &concat32_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice19_out1: [i64; 3] = shape15_out1[0..3].try_into().unwrap();
        let constant752_out1: [i64; 1] = [-1i64];
        let concat33_out1: [i64; 4usize] = [&slice19_out1[..], &constant752_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape36_out1 = concat32_out1.reshape(concat33_out1);
        let mul42_out1 = reshape36_out1.mul(reshape35_out1);
        let reducesum1_out1 = {
            mul42_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape37_out1 = squeeze3_out1;
        let unsqueeze18_out1 = [reshape37_out1 as i64];
        let constant756_out1: [i64; 1] = [256i64];
        let constant757_out1: [i64; 1] = [300i64];
        let concat34_out1: [i64; 3usize] = [
            &unsqueeze18_out1[..],
            &constant756_out1[..],
            &constant757_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape38_out1 = reducesum1_out1.reshape(concat34_out1);
        let transpose23_out1 = reshape38_out1.permute([0, 2, 1]);
        let linear21_out1 = self.linear21.forward(transpose23_out1);
        let add51_out1 = add40_out1.add(linear21_out1);
        add51_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule7 {
    constant760: burn::module::Param<Tensor<1>>,
    constant761: burn::module::Param<Tensor<1>>,
    constant762: burn::module::Param<Tensor<1>>,
    constant763: burn::module::Param<Tensor<1>>,
    linear22: Linear,
    linear23: Linear,
    constant768: burn::module::Param<Tensor<1>>,
    constant769: burn::module::Param<Tensor<1>>,
    constant770: burn::module::Param<Tensor<1>>,
    constant771: burn::module::Param<Tensor<1>>,
    linear24: Linear,
    linear25: Linear,
    linear26: Linear,
    constant782: burn::module::Param<Tensor<1>>,
    constant783: burn::module::Param<Tensor<1>>,
    linear27: Linear,
    linear28: Linear,
    linear29: Linear,
    linear30: Linear,
    linear31: Linear,
    constant796: burn::module::Param<Tensor<1>>,
    constant797: burn::module::Param<Tensor<1>>,
    linear32: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule7 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant760: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant761: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant762: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant763: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear22 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear23 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant768: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant769: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant770: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant771: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear24 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear25 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear26 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant782: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant783: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear27 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear28 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear29 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear30 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear31 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant796: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant797: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear32 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant760,
            constant761,
            constant762,
            constant763,
            linear22,
            linear23,
            constant768,
            constant769,
            constant770,
            constant771,
            linear24,
            linear25,
            linear26,
            constant782,
            constant783,
            linear27,
            linear28,
            linear29,
            linear30,
            linear31,
            constant796,
            constant797,
            linear32,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add51_out1: Tensor<3>,
        sigmoid25_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let reducemean9_out1 = { add51_out1.clone().mean_dim(2usize) };
        let sub5_out1 = add51_out1.sub(reducemean9_out1);
        let constant760_out1 = self.constant760.val();
        let pow5_out1 = sub5_out1
            .clone()
            .powf((constant760_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean10_out1 = { pow5_out1.mean_dim(2usize) };
        let constant761_out1 = self.constant761.val();
        let add52_out1 = reducemean10_out1
            .add((constant761_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt5_out1 = add52_out1.sqrt();
        let div6_out1 = sub5_out1.div(sqrt5_out1);
        let constant762_out1 = self.constant762.val();
        let mul43_out1 = div6_out1
            .mul((constant762_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant763_out1 = self.constant763.val();
        let add53_out1 = mul43_out1
            .add((constant763_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear22_out1 = self.linear22.forward(add53_out1.clone());
        let relu57_out1 = burn::tensor::activation::relu(linear22_out1);
        let linear23_out1 = self.linear23.forward(relu57_out1);
        let add54_out1 = add53_out1.add(linear23_out1);
        let reducemean11_out1 = { add54_out1.clone().mean_dim(2usize) };
        let sub6_out1 = add54_out1.sub(reducemean11_out1);
        let constant768_out1 = self.constant768.val();
        let pow6_out1 = sub6_out1
            .clone()
            .powf((constant768_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean12_out1 = { pow6_out1.mean_dim(2usize) };
        let constant769_out1 = self.constant769.val();
        let add55_out1 = reducemean12_out1
            .add((constant769_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt6_out1 = add55_out1.sqrt();
        let div7_out1 = sub6_out1.div(sqrt6_out1);
        let constant770_out1 = self.constant770.val();
        let mul44_out1 = div7_out1
            .mul((constant770_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant771_out1 = self.constant771.val();
        let add56_out1 = mul44_out1
            .add((constant771_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear24_out1 = self.linear24.forward(add56_out1.clone());
        let relu58_out1 = burn::tensor::activation::relu(linear24_out1);
        let linear25_out1 = self.linear25.forward(relu58_out1);
        let relu59_out1 = burn::tensor::activation::relu(linear25_out1);
        let linear26_out1 = self.linear26.forward(relu59_out1);
        let clip1_out1 = sigmoid25_out1.clamp(0f64, 1f64);
        let clip2_out1 = clip1_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant782_out1 = self.constant782.val();
        let mul45_out1 = clip1_out1
            .mul((constant782_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant783_out1 = self.constant783.val();
        let add57_out1 = mul45_out1
            .add((constant783_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip3_out1 = add57_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div8_out1 = clip2_out1.div(clip3_out1);
        let log1_out1 = div8_out1.log();
        let add58_out1 = linear26_out1.add(log1_out1);
        let sigmoid26_out1 = burn::tensor::activation::sigmoid(add58_out1);
        let unsqueeze19_out1: Tensor<4> = sigmoid26_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear27_out1 = self.linear27.forward(sigmoid26_out1.clone());
        let relu60_out1 = burn::tensor::activation::relu(linear27_out1);
        let linear28_out1 = self.linear28.forward(relu60_out1);
        let add59_out1 = add56_out1.clone().add(linear28_out1.clone());
        let linear29_out1 = self.linear29.forward(add59_out1.clone());
        let reshape39_out1 = linear29_out1.reshape([0, 0, 8, 32]);
        let transpose24_out1 = reshape39_out1.permute([0, 2, 1, 3]);
        let linear30_out1 = self.linear30.forward(add59_out1);
        let reshape40_out1 = linear30_out1.reshape([0, 0, 8, 32]);
        let transpose25_out1 = reshape40_out1.permute([0, 2, 1, 3]);
        let linear31_out1 = self.linear31.forward(add56_out1.clone());
        let reshape41_out1 = linear31_out1.reshape([0, 0, 8, 32]);
        let transpose26_out1 = reshape41_out1.permute([0, 2, 1, 3]);
        let transpose27_out1 = transpose25_out1.permute([0, 1, 3, 2]);
        let matmul36_out1 = transpose24_out1.matmul(transpose27_out1);
        let constant796_out1 = self.constant796.val();
        let mul46_out1 = matmul36_out1
            .mul((constant796_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant797_out1 = self.constant797.val();
        let add60_out1 = mul46_out1
            .add((constant797_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax4_out1 = burn::tensor::activation::softmax(add60_out1, 3);
        let matmul37_out1 = softmax4_out1.matmul(transpose26_out1);
        let transpose28_out1 = matmul37_out1.permute([0, 2, 1, 3]);
        let reshape42_out1 = transpose28_out1.reshape([0, 0, 256]);
        let linear32_out1 = self.linear32.forward(reshape42_out1);
        let add61_out1 = add56_out1.add(linear32_out1);
        (add61_out1, linear28_out1, unsqueeze19_out1, sigmoid26_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule8 {
    constant801: burn::module::Param<Tensor<1>>,
    constant802: burn::module::Param<Tensor<1>>,
    constant803: burn::module::Param<Tensor<1>>,
    constant804: burn::module::Param<Tensor<1>>,
    linear33: Linear,
    linear34: Linear,
    linear35: Linear,
    constant844: burn::module::Param<Tensor<1>>,
    constant845: burn::module::Param<Tensor<1>>,
    constant851: burn::module::Param<Tensor<1>>,
    constant852: burn::module::Param<Tensor<1>>,
    constant859: burn::module::Param<Tensor<1>>,
    constant860: burn::module::Param<Tensor<1>>,
    constant861: burn::module::Param<Tensor<1>>,
    constant862: burn::module::Param<Tensor<1>>,
    constant867: burn::module::Param<Tensor<1>>,
    constant868: burn::module::Param<Tensor<1>>,
    constant887: burn::module::Param<Tensor<1>>,
    constant888: burn::module::Param<Tensor<1>>,
    constant907: burn::module::Param<Tensor<1>>,
    constant908: burn::module::Param<Tensor<1>>,
    constant923: burn::module::Param<Tensor<1>>,
    constant924: burn::module::Param<Tensor<1>>,
    linear36: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule8 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant801: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant802: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant803: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant804: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear33 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear34 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear35 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant844: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant845: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant851: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant852: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant859: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant860: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant861: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant862: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant867: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant868: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant887: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant888: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant907: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant908: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant923: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant924: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear36 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant801,
            constant802,
            constant803,
            constant804,
            linear33,
            linear34,
            linear35,
            constant844,
            constant845,
            constant851,
            constant852,
            constant859,
            constant860,
            constant861,
            constant862,
            constant867,
            constant868,
            constant887,
            constant888,
            constant907,
            constant908,
            constant923,
            constant924,
            linear36,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add61_out1: Tensor<3>,
        linear28_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze19_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean13_out1 = { add61_out1.clone().mean_dim(2usize) };
        let sub7_out1 = add61_out1.sub(reducemean13_out1);
        let constant801_out1 = self.constant801.val();
        let pow7_out1 = sub7_out1
            .clone()
            .powf((constant801_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean14_out1 = { pow7_out1.mean_dim(2usize) };
        let constant802_out1 = self.constant802.val();
        let add62_out1 = reducemean14_out1
            .add((constant802_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt7_out1 = add62_out1.sqrt();
        let div9_out1 = sub7_out1.div(sqrt7_out1);
        let constant803_out1 = self.constant803.val();
        let mul47_out1 = div9_out1
            .mul((constant803_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant804_out1 = self.constant804.val();
        let add63_out1 = mul47_out1
            .add((constant804_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add64_out1 = add63_out1.clone().add(linear28_out1);
        let shape16_out1: [i64; 3] = {
            let axes = &add64_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice20_out1: [i64; 1] = shape16_out1[0..1].try_into().unwrap();
        let squeeze7_out1 = slice20_out1[0] as i64;
        let linear33_out1 = self.linear33.forward(concat16_out1);
        let reshape43_out1 = squeeze7_out1;
        let unsqueeze20_out1 = [reshape43_out1 as i64];
        let constant814_out1: [i64; 1] = [13125i64];
        let constant815_out1: [i64; 1] = [8i64];
        let constant816_out1: [i64; 1] = [32i64];
        let concat35_out1: [i64; 4usize] = [
            &unsqueeze20_out1[..],
            &constant814_out1[..],
            &constant815_out1[..],
            &constant816_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape44_out1 = linear33_out1.reshape(concat35_out1);
        let linear34_out1 = self.linear34.forward(add64_out1.clone());
        let reshape45_out1 = squeeze7_out1;
        let unsqueeze21_out1 = [reshape45_out1 as i64];
        let constant821_out1: [i64; 1] = [300i64];
        let constant822_out1: [i64; 1] = [8i64];
        let constant823_out1: [i64; 1] = [3i64];
        let constant824_out1: [i64; 1] = [4i64];
        let constant825_out1: [i64; 1] = [2i64];
        let concat36_out1: [i64; 6usize] = [
            &unsqueeze21_out1[..],
            &constant821_out1[..],
            &constant822_out1[..],
            &constant823_out1[..],
            &constant824_out1[..],
            &constant825_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape46_out1 = linear34_out1.reshape(concat36_out1);
        let linear35_out1 = self.linear35.forward(add64_out1);
        let reshape47_out1 = squeeze7_out1;
        let unsqueeze22_out1 = [reshape47_out1 as i64];
        let constant830_out1: [i64; 1] = [300i64];
        let constant831_out1: [i64; 1] = [8i64];
        let constant832_out1: [i64; 1] = [12i64];
        let concat37_out1: [i64; 4usize] = [
            &unsqueeze22_out1[..],
            &constant830_out1[..],
            &constant831_out1[..],
            &constant832_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape48_out1 = linear35_out1.reshape(concat37_out1);
        let softmax5_out1 = burn::tensor::activation::softmax(reshape48_out1, 3);
        let reshape49_out1 = squeeze7_out1;
        let unsqueeze23_out1 = [reshape49_out1 as i64];
        let constant835_out1: [i64; 1] = [300i64];
        let constant836_out1: [i64; 1] = [8i64];
        let constant837_out1: [i64; 1] = [3i64];
        let constant838_out1: [i64; 1] = [4i64];
        let concat38_out1: [i64; 5usize] = [
            &unsqueeze23_out1[..],
            &constant835_out1[..],
            &constant836_out1[..],
            &constant837_out1[..],
            &constant838_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape50_out1 = softmax5_out1.reshape(concat38_out1);
        let slice21_out1 = unsqueeze19_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze24_out1: Tensor<6> = slice21_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant844_out1 = self.constant844.val();
        let mul48_out1 = reshape46_out1
            .mul(
                (constant844_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant845_out1 = self.constant845.val();
        let add65_out1 = mul48_out1
            .add(
                (constant845_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice22_out1 = unsqueeze19_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze25_out1: Tensor<6> = slice22_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul49_out1 = add65_out1.mul(unsqueeze25_out1);
        let constant851_out1 = self.constant851.val();
        let mul50_out1 = mul49_out1
            .mul(
                (constant851_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant852_out1 = self.constant852.val();
        let add66_out1 = mul50_out1
            .add(
                (constant852_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add67_out1 = unsqueeze24_out1.add(add66_out1);
        let shape17_out1: [i64; 4] = {
            let axes = &reshape44_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice23_out1: [i64; 1] = shape17_out1[0..1].try_into().unwrap();
        let squeeze8_out1 = slice23_out1[0] as i64;
        let split_tensors = reshape44_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split2_out1, split2_out2, split2_out3] = split_tensors.try_into().unwrap();
        let constant859_out1 = self.constant859.val();
        let mul51_out1 = add67_out1
            .mul(
                (constant859_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant860_out1 = self.constant860.val();
        let add68_out1 = mul51_out1
            .add(
                (constant860_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant861_out1 = self.constant861.val();
        let mul52_out1 = add68_out1
            .mul(
                (constant861_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant862_out1 = self.constant862.val();
        let add69_out1 = mul52_out1
            .add(
                (constant862_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape18_out1: [i64; 4] = {
            let axes = &split2_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice24_out1: [i64; 2] = shape18_out1[0..2].try_into().unwrap();
        let constant866_out1: [i64; 1] = [-1i64];
        let concat39_out1: [i64; 3usize] = [&slice24_out1[..], &constant866_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape51_out1 = split2_out1.reshape(concat39_out1);
        let transpose29_out1 = reshape51_out1.permute([0, 2, 1]);
        let cast10_out1 = squeeze8_out1 as f32;
        let constant867_out1 = self.constant867.val();
        let mul53_out1 = cast10_out1 * constant867_out1;
        let constant868_out1 = self.constant868.val();
        let add70_out1 = mul53_out1.add(constant868_out1);
        let cast11_out1 = add70_out1.int().cast(burn::tensor::DType::I64);
        let reshape52_out1 = cast11_out1.into_scalar::<i64>();
        let unsqueeze26_out1 = [reshape52_out1 as i64];
        let constant871_out1: [i64; 1] = [32i64];
        let constant872_out1: [i64; 1] = [100i64];
        let constant873_out1: [i64; 1] = [100i64];
        let concat40_out1: [i64; 4usize] = [
            &unsqueeze26_out1[..],
            &constant871_out1[..],
            &constant872_out1[..],
            &constant873_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape53_out1 = transpose29_out1.reshape(concat40_out1);
        let slice25_out1 = add69_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze9_out1 = slice25_out1.squeeze_dims::<5>(&[3]);
        let transpose30_out1 = squeeze9_out1.permute([0, 2, 1, 3, 4]);
        let shape19_out1: [i64; 5] = {
            let axes = &transpose30_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice26_out1: [i64; 3] = shape19_out1[2..5].try_into().unwrap();
        let constant882_out1: [i64; 1] = [-1i64];
        let concat41_out1: [i64; 4usize] = [&constant882_out1[..], &slice26_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape54_out1 = transpose30_out1.reshape(concat41_out1);
        let gridsample4_out1 = reshape53_out1
            .grid_sample_2d(
                reshape54_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape20_out1: [i64; 4] = {
            let axes = &split2_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice27_out1: [i64; 2] = shape20_out1[0..2].try_into().unwrap();
        let constant886_out1: [i64; 1] = [-1i64];
        let concat42_out1: [i64; 3usize] = [&slice27_out1[..], &constant886_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape55_out1 = split2_out2.reshape(concat42_out1);
        let transpose31_out1 = reshape55_out1.permute([0, 2, 1]);
        let cast12_out1 = squeeze8_out1 as f32;
        let constant887_out1 = self.constant887.val();
        let mul54_out1 = cast12_out1 * constant887_out1;
        let constant888_out1 = self.constant888.val();
        let add71_out1 = mul54_out1.add(constant888_out1);
        let cast13_out1 = add71_out1.int().cast(burn::tensor::DType::I64);
        let reshape56_out1 = cast13_out1.into_scalar::<i64>();
        let unsqueeze27_out1 = [reshape56_out1 as i64];
        let constant891_out1: [i64; 1] = [32i64];
        let constant892_out1: [i64; 1] = [50i64];
        let constant893_out1: [i64; 1] = [50i64];
        let concat43_out1: [i64; 4usize] = [
            &unsqueeze27_out1[..],
            &constant891_out1[..],
            &constant892_out1[..],
            &constant893_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape57_out1 = transpose31_out1.reshape(concat43_out1);
        let slice28_out1 = add69_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze10_out1 = slice28_out1.squeeze_dims::<5>(&[3]);
        let transpose32_out1 = squeeze10_out1.permute([0, 2, 1, 3, 4]);
        let shape21_out1: [i64; 5] = {
            let axes = &transpose32_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice29_out1: [i64; 3] = shape21_out1[2..5].try_into().unwrap();
        let constant902_out1: [i64; 1] = [-1i64];
        let concat44_out1: [i64; 4usize] = [&constant902_out1[..], &slice29_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape58_out1 = transpose32_out1.reshape(concat44_out1);
        let gridsample5_out1 = reshape57_out1
            .grid_sample_2d(
                reshape58_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape22_out1: [i64; 4] = {
            let axes = &split2_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice30_out1: [i64; 2] = shape22_out1[0..2].try_into().unwrap();
        let constant906_out1: [i64; 1] = [-1i64];
        let concat45_out1: [i64; 3usize] = [&slice30_out1[..], &constant906_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape59_out1 = split2_out3.reshape(concat45_out1);
        let transpose33_out1 = reshape59_out1.permute([0, 2, 1]);
        let cast14_out1 = squeeze8_out1 as f32;
        let constant907_out1 = self.constant907.val();
        let mul55_out1 = cast14_out1 * constant907_out1;
        let constant908_out1 = self.constant908.val();
        let add72_out1 = mul55_out1.add(constant908_out1);
        let cast15_out1 = add72_out1.int().cast(burn::tensor::DType::I64);
        let reshape60_out1 = cast15_out1.into_scalar::<i64>();
        let unsqueeze28_out1 = [reshape60_out1 as i64];
        let constant911_out1: [i64; 1] = [32i64];
        let constant912_out1: [i64; 1] = [25i64];
        let constant913_out1: [i64; 1] = [25i64];
        let concat46_out1: [i64; 4usize] = [
            &unsqueeze28_out1[..],
            &constant911_out1[..],
            &constant912_out1[..],
            &constant913_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape61_out1 = transpose33_out1.reshape(concat46_out1);
        let slice31_out1 = add69_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze11_out1 = slice31_out1.squeeze_dims::<5>(&[3]);
        let transpose34_out1 = squeeze11_out1.permute([0, 2, 1, 3, 4]);
        let shape23_out1: [i64; 5] = {
            let axes = &transpose34_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice32_out1: [i64; 3] = shape23_out1[2..5].try_into().unwrap();
        let constant922_out1: [i64; 1] = [-1i64];
        let concat47_out1: [i64; 4usize] = [&constant922_out1[..], &slice32_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape62_out1 = transpose34_out1.reshape(concat47_out1);
        let gridsample6_out1 = reshape61_out1
            .grid_sample_2d(
                reshape62_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose35_out1 = reshape50_out1.permute([0, 2, 1, 3, 4]);
        let cast16_out1 = squeeze8_out1 as f32;
        let constant923_out1 = self.constant923.val();
        let mul56_out1 = cast16_out1 * constant923_out1;
        let constant924_out1 = self.constant924.val();
        let add73_out1 = mul56_out1.add(constant924_out1);
        let cast17_out1 = add73_out1.int().cast(burn::tensor::DType::I64);
        let reshape63_out1 = cast17_out1.into_scalar::<i64>();
        let unsqueeze29_out1 = [reshape63_out1 as i64];
        let constant927_out1: [i64; 1] = [1i64];
        let constant928_out1: [i64; 1] = [300i64];
        let constant929_out1: [i64; 1] = [12i64];
        let concat48_out1: [i64; 4usize] = [
            &unsqueeze29_out1[..],
            &constant927_out1[..],
            &constant928_out1[..],
            &constant929_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape64_out1 = transpose35_out1.reshape(concat48_out1);
        let unsqueeze30_out1: Tensor<5> = gridsample4_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze31_out1: Tensor<5> = gridsample5_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze32_out1: Tensor<5> = gridsample6_out1.unsqueeze_dims::<5>(&[3]);
        let concat49_out1 = burn::tensor::Tensor::cat(
            [unsqueeze30_out1, unsqueeze31_out1, unsqueeze32_out1].into(),
            3,
        );
        let shape24_out1: [i64; 5] = {
            let axes = &concat49_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice33_out1: [i64; 3] = shape24_out1[0..3].try_into().unwrap();
        let constant936_out1: [i64; 1] = [-1i64];
        let concat50_out1: [i64; 4usize] = [&slice33_out1[..], &constant936_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape65_out1 = concat49_out1.reshape(concat50_out1);
        let mul57_out1 = reshape65_out1.mul(reshape64_out1);
        let reducesum2_out1 = {
            mul57_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape66_out1 = squeeze8_out1;
        let unsqueeze33_out1 = [reshape66_out1 as i64];
        let constant940_out1: [i64; 1] = [256i64];
        let constant941_out1: [i64; 1] = [300i64];
        let concat51_out1: [i64; 3usize] = [
            &unsqueeze33_out1[..],
            &constant940_out1[..],
            &constant941_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape67_out1 = reducesum2_out1.reshape(concat51_out1);
        let transpose36_out1 = reshape67_out1.permute([0, 2, 1]);
        let linear36_out1 = self.linear36.forward(transpose36_out1);
        let add74_out1 = add63_out1.add(linear36_out1);
        add74_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule9 {
    constant944: burn::module::Param<Tensor<1>>,
    constant945: burn::module::Param<Tensor<1>>,
    constant946: burn::module::Param<Tensor<1>>,
    constant947: burn::module::Param<Tensor<1>>,
    linear37: Linear,
    linear38: Linear,
    constant952: burn::module::Param<Tensor<1>>,
    constant953: burn::module::Param<Tensor<1>>,
    constant954: burn::module::Param<Tensor<1>>,
    constant955: burn::module::Param<Tensor<1>>,
    linear39: Linear,
    linear40: Linear,
    linear41: Linear,
    constant966: burn::module::Param<Tensor<1>>,
    constant967: burn::module::Param<Tensor<1>>,
    linear42: Linear,
    linear43: Linear,
    linear44: Linear,
    linear45: Linear,
    linear46: Linear,
    constant980: burn::module::Param<Tensor<1>>,
    constant981: burn::module::Param<Tensor<1>>,
    linear47: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule9 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant944: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant945: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant946: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant947: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear37 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear38 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant952: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant953: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant954: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant955: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear39 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear40 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear41 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant966: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant967: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear42 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear43 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear44 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear45 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear46 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant980: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant981: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear47 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant944,
            constant945,
            constant946,
            constant947,
            linear37,
            linear38,
            constant952,
            constant953,
            constant954,
            constant955,
            linear39,
            linear40,
            linear41,
            constant966,
            constant967,
            linear42,
            linear43,
            linear44,
            linear45,
            linear46,
            constant980,
            constant981,
            linear47,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add74_out1: Tensor<3>,
        sigmoid26_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let reducemean15_out1 = { add74_out1.clone().mean_dim(2usize) };
        let sub8_out1 = add74_out1.sub(reducemean15_out1);
        let constant944_out1 = self.constant944.val();
        let pow8_out1 = sub8_out1
            .clone()
            .powf((constant944_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean16_out1 = { pow8_out1.mean_dim(2usize) };
        let constant945_out1 = self.constant945.val();
        let add75_out1 = reducemean16_out1
            .add((constant945_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt8_out1 = add75_out1.sqrt();
        let div10_out1 = sub8_out1.div(sqrt8_out1);
        let constant946_out1 = self.constant946.val();
        let mul58_out1 = div10_out1
            .mul((constant946_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant947_out1 = self.constant947.val();
        let add76_out1 = mul58_out1
            .add((constant947_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear37_out1 = self.linear37.forward(add76_out1.clone());
        let relu61_out1 = burn::tensor::activation::relu(linear37_out1);
        let linear38_out1 = self.linear38.forward(relu61_out1);
        let add77_out1 = add76_out1.add(linear38_out1);
        let reducemean17_out1 = { add77_out1.clone().mean_dim(2usize) };
        let sub9_out1 = add77_out1.sub(reducemean17_out1);
        let constant952_out1 = self.constant952.val();
        let pow9_out1 = sub9_out1
            .clone()
            .powf((constant952_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean18_out1 = { pow9_out1.mean_dim(2usize) };
        let constant953_out1 = self.constant953.val();
        let add78_out1 = reducemean18_out1
            .add((constant953_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt9_out1 = add78_out1.sqrt();
        let div11_out1 = sub9_out1.div(sqrt9_out1);
        let constant954_out1 = self.constant954.val();
        let mul59_out1 = div11_out1
            .mul((constant954_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant955_out1 = self.constant955.val();
        let add79_out1 = mul59_out1
            .add((constant955_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear39_out1 = self.linear39.forward(add79_out1.clone());
        let relu62_out1 = burn::tensor::activation::relu(linear39_out1);
        let linear40_out1 = self.linear40.forward(relu62_out1);
        let relu63_out1 = burn::tensor::activation::relu(linear40_out1);
        let linear41_out1 = self.linear41.forward(relu63_out1);
        let clip4_out1 = sigmoid26_out1.clamp(0f64, 1f64);
        let clip5_out1 = clip4_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant966_out1 = self.constant966.val();
        let mul60_out1 = clip4_out1
            .mul((constant966_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant967_out1 = self.constant967.val();
        let add80_out1 = mul60_out1
            .add((constant967_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip6_out1 = add80_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div12_out1 = clip5_out1.div(clip6_out1);
        let log2_out1 = div12_out1.log();
        let add81_out1 = linear41_out1.add(log2_out1);
        let sigmoid27_out1 = burn::tensor::activation::sigmoid(add81_out1);
        let unsqueeze34_out1: Tensor<4> = sigmoid27_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear42_out1 = self.linear42.forward(sigmoid27_out1.clone());
        let relu64_out1 = burn::tensor::activation::relu(linear42_out1);
        let linear43_out1 = self.linear43.forward(relu64_out1);
        let add82_out1 = add79_out1.clone().add(linear43_out1.clone());
        let linear44_out1 = self.linear44.forward(add82_out1.clone());
        let reshape68_out1 = linear44_out1.reshape([0, 0, 8, 32]);
        let transpose37_out1 = reshape68_out1.permute([0, 2, 1, 3]);
        let linear45_out1 = self.linear45.forward(add82_out1);
        let reshape69_out1 = linear45_out1.reshape([0, 0, 8, 32]);
        let transpose38_out1 = reshape69_out1.permute([0, 2, 1, 3]);
        let linear46_out1 = self.linear46.forward(add79_out1.clone());
        let reshape70_out1 = linear46_out1.reshape([0, 0, 8, 32]);
        let transpose39_out1 = reshape70_out1.permute([0, 2, 1, 3]);
        let transpose40_out1 = transpose38_out1.permute([0, 1, 3, 2]);
        let matmul53_out1 = transpose37_out1.matmul(transpose40_out1);
        let constant980_out1 = self.constant980.val();
        let mul61_out1 = matmul53_out1
            .mul((constant980_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant981_out1 = self.constant981.val();
        let add83_out1 = mul61_out1
            .add((constant981_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax6_out1 = burn::tensor::activation::softmax(add83_out1, 3);
        let matmul54_out1 = softmax6_out1.matmul(transpose39_out1);
        let transpose41_out1 = matmul54_out1.permute([0, 2, 1, 3]);
        let reshape71_out1 = transpose41_out1.reshape([0, 0, 256]);
        let linear47_out1 = self.linear47.forward(reshape71_out1);
        let add84_out1 = add79_out1.add(linear47_out1);
        (add84_out1, linear43_out1, unsqueeze34_out1, sigmoid27_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule10 {
    constant985: burn::module::Param<Tensor<1>>,
    constant986: burn::module::Param<Tensor<1>>,
    constant987: burn::module::Param<Tensor<1>>,
    constant988: burn::module::Param<Tensor<1>>,
    linear48: Linear,
    linear49: Linear,
    linear50: Linear,
    constant1028: burn::module::Param<Tensor<1>>,
    constant1029: burn::module::Param<Tensor<1>>,
    constant1035: burn::module::Param<Tensor<1>>,
    constant1036: burn::module::Param<Tensor<1>>,
    constant1043: burn::module::Param<Tensor<1>>,
    constant1044: burn::module::Param<Tensor<1>>,
    constant1045: burn::module::Param<Tensor<1>>,
    constant1046: burn::module::Param<Tensor<1>>,
    constant1051: burn::module::Param<Tensor<1>>,
    constant1052: burn::module::Param<Tensor<1>>,
    constant1071: burn::module::Param<Tensor<1>>,
    constant1072: burn::module::Param<Tensor<1>>,
    constant1091: burn::module::Param<Tensor<1>>,
    constant1092: burn::module::Param<Tensor<1>>,
    constant1107: burn::module::Param<Tensor<1>>,
    constant1108: burn::module::Param<Tensor<1>>,
    linear51: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule10 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant985: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant986: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant987: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant988: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear48 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear49 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear50 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant1028: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1029: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1035: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1036: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1043: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1044: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1045: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1046: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1051: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1052: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1071: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1072: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1091: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1092: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1107: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1108: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear51 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant985,
            constant986,
            constant987,
            constant988,
            linear48,
            linear49,
            linear50,
            constant1028,
            constant1029,
            constant1035,
            constant1036,
            constant1043,
            constant1044,
            constant1045,
            constant1046,
            constant1051,
            constant1052,
            constant1071,
            constant1072,
            constant1091,
            constant1092,
            constant1107,
            constant1108,
            linear51,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add84_out1: Tensor<3>,
        linear43_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze34_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean19_out1 = { add84_out1.clone().mean_dim(2usize) };
        let sub10_out1 = add84_out1.sub(reducemean19_out1);
        let constant985_out1 = self.constant985.val();
        let pow10_out1 = sub10_out1
            .clone()
            .powf((constant985_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean20_out1 = { pow10_out1.mean_dim(2usize) };
        let constant986_out1 = self.constant986.val();
        let add85_out1 = reducemean20_out1
            .add((constant986_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt10_out1 = add85_out1.sqrt();
        let div13_out1 = sub10_out1.div(sqrt10_out1);
        let constant987_out1 = self.constant987.val();
        let mul62_out1 = div13_out1
            .mul((constant987_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant988_out1 = self.constant988.val();
        let add86_out1 = mul62_out1
            .add((constant988_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add87_out1 = add86_out1.clone().add(linear43_out1);
        let shape25_out1: [i64; 3] = {
            let axes = &add87_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice34_out1: [i64; 1] = shape25_out1[0..1].try_into().unwrap();
        let squeeze12_out1 = slice34_out1[0] as i64;
        let linear48_out1 = self.linear48.forward(concat16_out1);
        let reshape72_out1 = squeeze12_out1;
        let unsqueeze35_out1 = [reshape72_out1 as i64];
        let constant998_out1: [i64; 1] = [13125i64];
        let constant999_out1: [i64; 1] = [8i64];
        let constant1000_out1: [i64; 1] = [32i64];
        let concat52_out1: [i64; 4usize] = [
            &unsqueeze35_out1[..],
            &constant998_out1[..],
            &constant999_out1[..],
            &constant1000_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape73_out1 = linear48_out1.reshape(concat52_out1);
        let linear49_out1 = self.linear49.forward(add87_out1.clone());
        let reshape74_out1 = squeeze12_out1;
        let unsqueeze36_out1 = [reshape74_out1 as i64];
        let constant1005_out1: [i64; 1] = [300i64];
        let constant1006_out1: [i64; 1] = [8i64];
        let constant1007_out1: [i64; 1] = [3i64];
        let constant1008_out1: [i64; 1] = [4i64];
        let constant1009_out1: [i64; 1] = [2i64];
        let concat53_out1: [i64; 6usize] = [
            &unsqueeze36_out1[..],
            &constant1005_out1[..],
            &constant1006_out1[..],
            &constant1007_out1[..],
            &constant1008_out1[..],
            &constant1009_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape75_out1 = linear49_out1.reshape(concat53_out1);
        let linear50_out1 = self.linear50.forward(add87_out1);
        let reshape76_out1 = squeeze12_out1;
        let unsqueeze37_out1 = [reshape76_out1 as i64];
        let constant1014_out1: [i64; 1] = [300i64];
        let constant1015_out1: [i64; 1] = [8i64];
        let constant1016_out1: [i64; 1] = [12i64];
        let concat54_out1: [i64; 4usize] = [
            &unsqueeze37_out1[..],
            &constant1014_out1[..],
            &constant1015_out1[..],
            &constant1016_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape77_out1 = linear50_out1.reshape(concat54_out1);
        let softmax7_out1 = burn::tensor::activation::softmax(reshape77_out1, 3);
        let reshape78_out1 = squeeze12_out1;
        let unsqueeze38_out1 = [reshape78_out1 as i64];
        let constant1019_out1: [i64; 1] = [300i64];
        let constant1020_out1: [i64; 1] = [8i64];
        let constant1021_out1: [i64; 1] = [3i64];
        let constant1022_out1: [i64; 1] = [4i64];
        let concat55_out1: [i64; 5usize] = [
            &unsqueeze38_out1[..],
            &constant1019_out1[..],
            &constant1020_out1[..],
            &constant1021_out1[..],
            &constant1022_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape79_out1 = softmax7_out1.reshape(concat55_out1);
        let slice35_out1 = unsqueeze34_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze39_out1: Tensor<6> = slice35_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant1028_out1 = self.constant1028.val();
        let mul63_out1 = reshape75_out1
            .mul(
                (constant1028_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1029_out1 = self.constant1029.val();
        let add88_out1 = mul63_out1
            .add(
                (constant1029_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice36_out1 = unsqueeze34_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze40_out1: Tensor<6> = slice36_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul64_out1 = add88_out1.mul(unsqueeze40_out1);
        let constant1035_out1 = self.constant1035.val();
        let mul65_out1 = mul64_out1
            .mul(
                (constant1035_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1036_out1 = self.constant1036.val();
        let add89_out1 = mul65_out1
            .add(
                (constant1036_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add90_out1 = unsqueeze39_out1.add(add89_out1);
        let shape26_out1: [i64; 4] = {
            let axes = &reshape73_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice37_out1: [i64; 1] = shape26_out1[0..1].try_into().unwrap();
        let squeeze13_out1 = slice37_out1[0] as i64;
        let split_tensors = reshape73_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split3_out1, split3_out2, split3_out3] = split_tensors.try_into().unwrap();
        let constant1043_out1 = self.constant1043.val();
        let mul66_out1 = add90_out1
            .mul(
                (constant1043_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1044_out1 = self.constant1044.val();
        let add91_out1 = mul66_out1
            .add(
                (constant1044_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1045_out1 = self.constant1045.val();
        let mul67_out1 = add91_out1
            .mul(
                (constant1045_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1046_out1 = self.constant1046.val();
        let add92_out1 = mul67_out1
            .add(
                (constant1046_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape27_out1: [i64; 4] = {
            let axes = &split3_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice38_out1: [i64; 2] = shape27_out1[0..2].try_into().unwrap();
        let constant1050_out1: [i64; 1] = [-1i64];
        let concat56_out1: [i64; 3usize] = [&slice38_out1[..], &constant1050_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape80_out1 = split3_out1.reshape(concat56_out1);
        let transpose42_out1 = reshape80_out1.permute([0, 2, 1]);
        let cast18_out1 = squeeze13_out1 as f32;
        let constant1051_out1 = self.constant1051.val();
        let mul68_out1 = cast18_out1 * constant1051_out1;
        let constant1052_out1 = self.constant1052.val();
        let add93_out1 = mul68_out1.add(constant1052_out1);
        let cast19_out1 = add93_out1.int().cast(burn::tensor::DType::I64);
        let reshape81_out1 = cast19_out1.into_scalar::<i64>();
        let unsqueeze41_out1 = [reshape81_out1 as i64];
        let constant1055_out1: [i64; 1] = [32i64];
        let constant1056_out1: [i64; 1] = [100i64];
        let constant1057_out1: [i64; 1] = [100i64];
        let concat57_out1: [i64; 4usize] = [
            &unsqueeze41_out1[..],
            &constant1055_out1[..],
            &constant1056_out1[..],
            &constant1057_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape82_out1 = transpose42_out1.reshape(concat57_out1);
        let slice39_out1 = add92_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze14_out1 = slice39_out1.squeeze_dims::<5>(&[3]);
        let transpose43_out1 = squeeze14_out1.permute([0, 2, 1, 3, 4]);
        let shape28_out1: [i64; 5] = {
            let axes = &transpose43_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice40_out1: [i64; 3] = shape28_out1[2..5].try_into().unwrap();
        let constant1066_out1: [i64; 1] = [-1i64];
        let concat58_out1: [i64; 4usize] = [&constant1066_out1[..], &slice40_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape83_out1 = transpose43_out1.reshape(concat58_out1);
        let gridsample7_out1 = reshape82_out1
            .grid_sample_2d(
                reshape83_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape29_out1: [i64; 4] = {
            let axes = &split3_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice41_out1: [i64; 2] = shape29_out1[0..2].try_into().unwrap();
        let constant1070_out1: [i64; 1] = [-1i64];
        let concat59_out1: [i64; 3usize] = [&slice41_out1[..], &constant1070_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape84_out1 = split3_out2.reshape(concat59_out1);
        let transpose44_out1 = reshape84_out1.permute([0, 2, 1]);
        let cast20_out1 = squeeze13_out1 as f32;
        let constant1071_out1 = self.constant1071.val();
        let mul69_out1 = cast20_out1 * constant1071_out1;
        let constant1072_out1 = self.constant1072.val();
        let add94_out1 = mul69_out1.add(constant1072_out1);
        let cast21_out1 = add94_out1.int().cast(burn::tensor::DType::I64);
        let reshape85_out1 = cast21_out1.into_scalar::<i64>();
        let unsqueeze42_out1 = [reshape85_out1 as i64];
        let constant1075_out1: [i64; 1] = [32i64];
        let constant1076_out1: [i64; 1] = [50i64];
        let constant1077_out1: [i64; 1] = [50i64];
        let concat60_out1: [i64; 4usize] = [
            &unsqueeze42_out1[..],
            &constant1075_out1[..],
            &constant1076_out1[..],
            &constant1077_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape86_out1 = transpose44_out1.reshape(concat60_out1);
        let slice42_out1 = add92_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze15_out1 = slice42_out1.squeeze_dims::<5>(&[3]);
        let transpose45_out1 = squeeze15_out1.permute([0, 2, 1, 3, 4]);
        let shape30_out1: [i64; 5] = {
            let axes = &transpose45_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice43_out1: [i64; 3] = shape30_out1[2..5].try_into().unwrap();
        let constant1086_out1: [i64; 1] = [-1i64];
        let concat61_out1: [i64; 4usize] = [&constant1086_out1[..], &slice43_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape87_out1 = transpose45_out1.reshape(concat61_out1);
        let gridsample8_out1 = reshape86_out1
            .grid_sample_2d(
                reshape87_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape31_out1: [i64; 4] = {
            let axes = &split3_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice44_out1: [i64; 2] = shape31_out1[0..2].try_into().unwrap();
        let constant1090_out1: [i64; 1] = [-1i64];
        let concat62_out1: [i64; 3usize] = [&slice44_out1[..], &constant1090_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape88_out1 = split3_out3.reshape(concat62_out1);
        let transpose46_out1 = reshape88_out1.permute([0, 2, 1]);
        let cast22_out1 = squeeze13_out1 as f32;
        let constant1091_out1 = self.constant1091.val();
        let mul70_out1 = cast22_out1 * constant1091_out1;
        let constant1092_out1 = self.constant1092.val();
        let add95_out1 = mul70_out1.add(constant1092_out1);
        let cast23_out1 = add95_out1.int().cast(burn::tensor::DType::I64);
        let reshape89_out1 = cast23_out1.into_scalar::<i64>();
        let unsqueeze43_out1 = [reshape89_out1 as i64];
        let constant1095_out1: [i64; 1] = [32i64];
        let constant1096_out1: [i64; 1] = [25i64];
        let constant1097_out1: [i64; 1] = [25i64];
        let concat63_out1: [i64; 4usize] = [
            &unsqueeze43_out1[..],
            &constant1095_out1[..],
            &constant1096_out1[..],
            &constant1097_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape90_out1 = transpose46_out1.reshape(concat63_out1);
        let slice45_out1 = add92_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze16_out1 = slice45_out1.squeeze_dims::<5>(&[3]);
        let transpose47_out1 = squeeze16_out1.permute([0, 2, 1, 3, 4]);
        let shape32_out1: [i64; 5] = {
            let axes = &transpose47_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice46_out1: [i64; 3] = shape32_out1[2..5].try_into().unwrap();
        let constant1106_out1: [i64; 1] = [-1i64];
        let concat64_out1: [i64; 4usize] = [&constant1106_out1[..], &slice46_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape91_out1 = transpose47_out1.reshape(concat64_out1);
        let gridsample9_out1 = reshape90_out1
            .grid_sample_2d(
                reshape91_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose48_out1 = reshape79_out1.permute([0, 2, 1, 3, 4]);
        let cast24_out1 = squeeze13_out1 as f32;
        let constant1107_out1 = self.constant1107.val();
        let mul71_out1 = cast24_out1 * constant1107_out1;
        let constant1108_out1 = self.constant1108.val();
        let add96_out1 = mul71_out1.add(constant1108_out1);
        let cast25_out1 = add96_out1.int().cast(burn::tensor::DType::I64);
        let reshape92_out1 = cast25_out1.into_scalar::<i64>();
        let unsqueeze44_out1 = [reshape92_out1 as i64];
        let constant1111_out1: [i64; 1] = [1i64];
        let constant1112_out1: [i64; 1] = [300i64];
        let constant1113_out1: [i64; 1] = [12i64];
        let concat65_out1: [i64; 4usize] = [
            &unsqueeze44_out1[..],
            &constant1111_out1[..],
            &constant1112_out1[..],
            &constant1113_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape93_out1 = transpose48_out1.reshape(concat65_out1);
        let unsqueeze45_out1: Tensor<5> = gridsample7_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze46_out1: Tensor<5> = gridsample8_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze47_out1: Tensor<5> = gridsample9_out1.unsqueeze_dims::<5>(&[3]);
        let concat66_out1 = burn::tensor::Tensor::cat(
            [unsqueeze45_out1, unsqueeze46_out1, unsqueeze47_out1].into(),
            3,
        );
        let shape33_out1: [i64; 5] = {
            let axes = &concat66_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice47_out1: [i64; 3] = shape33_out1[0..3].try_into().unwrap();
        let constant1120_out1: [i64; 1] = [-1i64];
        let concat67_out1: [i64; 4usize] = [&slice47_out1[..], &constant1120_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape94_out1 = concat66_out1.reshape(concat67_out1);
        let mul72_out1 = reshape94_out1.mul(reshape93_out1);
        let reducesum3_out1 = {
            mul72_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape95_out1 = squeeze13_out1;
        let unsqueeze48_out1 = [reshape95_out1 as i64];
        let constant1124_out1: [i64; 1] = [256i64];
        let constant1125_out1: [i64; 1] = [300i64];
        let concat68_out1: [i64; 3usize] = [
            &unsqueeze48_out1[..],
            &constant1124_out1[..],
            &constant1125_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape96_out1 = reducesum3_out1.reshape(concat68_out1);
        let transpose49_out1 = reshape96_out1.permute([0, 2, 1]);
        let linear51_out1 = self.linear51.forward(transpose49_out1);
        let add97_out1 = add86_out1.add(linear51_out1);
        add97_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule11 {
    constant1128: burn::module::Param<Tensor<1>>,
    constant1129: burn::module::Param<Tensor<1>>,
    constant1130: burn::module::Param<Tensor<1>>,
    constant1131: burn::module::Param<Tensor<1>>,
    linear52: Linear,
    linear53: Linear,
    constant1136: burn::module::Param<Tensor<1>>,
    constant1137: burn::module::Param<Tensor<1>>,
    constant1138: burn::module::Param<Tensor<1>>,
    constant1139: burn::module::Param<Tensor<1>>,
    linear54: Linear,
    linear55: Linear,
    linear56: Linear,
    constant1150: burn::module::Param<Tensor<1>>,
    constant1151: burn::module::Param<Tensor<1>>,
    linear57: Linear,
    linear58: Linear,
    linear59: Linear,
    linear60: Linear,
    linear61: Linear,
    constant1164: burn::module::Param<Tensor<1>>,
    constant1165: burn::module::Param<Tensor<1>>,
    linear62: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule11 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1128: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1129: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1130: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1131: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear52 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear53 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant1136: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1137: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1138: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1139: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear54 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear55 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear56 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant1150: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1151: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear57 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear58 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear59 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear60 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear61 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant1164: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1165: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear62 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1128,
            constant1129,
            constant1130,
            constant1131,
            linear52,
            linear53,
            constant1136,
            constant1137,
            constant1138,
            constant1139,
            linear54,
            linear55,
            linear56,
            constant1150,
            constant1151,
            linear57,
            linear58,
            linear59,
            linear60,
            linear61,
            constant1164,
            constant1165,
            linear62,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add97_out1: Tensor<3>,
        sigmoid27_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let reducemean21_out1 = { add97_out1.clone().mean_dim(2usize) };
        let sub11_out1 = add97_out1.sub(reducemean21_out1);
        let constant1128_out1 = self.constant1128.val();
        let pow11_out1 = sub11_out1
            .clone()
            .powf((constant1128_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean22_out1 = { pow11_out1.mean_dim(2usize) };
        let constant1129_out1 = self.constant1129.val();
        let add98_out1 = reducemean22_out1
            .add((constant1129_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt11_out1 = add98_out1.sqrt();
        let div14_out1 = sub11_out1.div(sqrt11_out1);
        let constant1130_out1 = self.constant1130.val();
        let mul73_out1 = div14_out1
            .mul((constant1130_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1131_out1 = self.constant1131.val();
        let add99_out1 = mul73_out1
            .add((constant1131_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear52_out1 = self.linear52.forward(add99_out1.clone());
        let relu65_out1 = burn::tensor::activation::relu(linear52_out1);
        let linear53_out1 = self.linear53.forward(relu65_out1);
        let add100_out1 = add99_out1.add(linear53_out1);
        let reducemean23_out1 = { add100_out1.clone().mean_dim(2usize) };
        let sub12_out1 = add100_out1.sub(reducemean23_out1);
        let constant1136_out1 = self.constant1136.val();
        let pow12_out1 = sub12_out1
            .clone()
            .powf((constant1136_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean24_out1 = { pow12_out1.mean_dim(2usize) };
        let constant1137_out1 = self.constant1137.val();
        let add101_out1 = reducemean24_out1
            .add((constant1137_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt12_out1 = add101_out1.sqrt();
        let div15_out1 = sub12_out1.div(sqrt12_out1);
        let constant1138_out1 = self.constant1138.val();
        let mul74_out1 = div15_out1
            .mul((constant1138_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1139_out1 = self.constant1139.val();
        let add102_out1 = mul74_out1
            .add((constant1139_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear54_out1 = self.linear54.forward(add102_out1.clone());
        let relu66_out1 = burn::tensor::activation::relu(linear54_out1);
        let linear55_out1 = self.linear55.forward(relu66_out1);
        let relu67_out1 = burn::tensor::activation::relu(linear55_out1);
        let linear56_out1 = self.linear56.forward(relu67_out1);
        let clip7_out1 = sigmoid27_out1.clamp(0f64, 1f64);
        let clip8_out1 = clip7_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant1150_out1 = self.constant1150.val();
        let mul75_out1 = clip7_out1
            .mul((constant1150_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1151_out1 = self.constant1151.val();
        let add103_out1 = mul75_out1
            .add((constant1151_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip9_out1 = add103_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div16_out1 = clip8_out1.div(clip9_out1);
        let log3_out1 = div16_out1.log();
        let add104_out1 = linear56_out1.add(log3_out1);
        let sigmoid28_out1 = burn::tensor::activation::sigmoid(add104_out1);
        let unsqueeze49_out1: Tensor<4> = sigmoid28_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear57_out1 = self.linear57.forward(sigmoid28_out1.clone());
        let relu68_out1 = burn::tensor::activation::relu(linear57_out1);
        let linear58_out1 = self.linear58.forward(relu68_out1);
        let add105_out1 = add102_out1.clone().add(linear58_out1.clone());
        let linear59_out1 = self.linear59.forward(add105_out1.clone());
        let reshape97_out1 = linear59_out1.reshape([0, 0, 8, 32]);
        let transpose50_out1 = reshape97_out1.permute([0, 2, 1, 3]);
        let linear60_out1 = self.linear60.forward(add105_out1);
        let reshape98_out1 = linear60_out1.reshape([0, 0, 8, 32]);
        let transpose51_out1 = reshape98_out1.permute([0, 2, 1, 3]);
        let linear61_out1 = self.linear61.forward(add102_out1.clone());
        let reshape99_out1 = linear61_out1.reshape([0, 0, 8, 32]);
        let transpose52_out1 = reshape99_out1.permute([0, 2, 1, 3]);
        let transpose53_out1 = transpose51_out1.permute([0, 1, 3, 2]);
        let matmul70_out1 = transpose50_out1.matmul(transpose53_out1);
        let constant1164_out1 = self.constant1164.val();
        let mul76_out1 = matmul70_out1
            .mul((constant1164_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant1165_out1 = self.constant1165.val();
        let add106_out1 = mul76_out1
            .add((constant1165_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax8_out1 = burn::tensor::activation::softmax(add106_out1, 3);
        let matmul71_out1 = softmax8_out1.matmul(transpose52_out1);
        let transpose54_out1 = matmul71_out1.permute([0, 2, 1, 3]);
        let reshape100_out1 = transpose54_out1.reshape([0, 0, 256]);
        let linear62_out1 = self.linear62.forward(reshape100_out1);
        let add107_out1 = add102_out1.add(linear62_out1);
        (add107_out1, linear58_out1, unsqueeze49_out1, sigmoid28_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule12 {
    constant1169: burn::module::Param<Tensor<1>>,
    constant1170: burn::module::Param<Tensor<1>>,
    constant1171: burn::module::Param<Tensor<1>>,
    constant1172: burn::module::Param<Tensor<1>>,
    linear63: Linear,
    linear64: Linear,
    linear65: Linear,
    constant1212: burn::module::Param<Tensor<1>>,
    constant1213: burn::module::Param<Tensor<1>>,
    constant1219: burn::module::Param<Tensor<1>>,
    constant1220: burn::module::Param<Tensor<1>>,
    constant1227: burn::module::Param<Tensor<1>>,
    constant1228: burn::module::Param<Tensor<1>>,
    constant1229: burn::module::Param<Tensor<1>>,
    constant1230: burn::module::Param<Tensor<1>>,
    constant1235: burn::module::Param<Tensor<1>>,
    constant1236: burn::module::Param<Tensor<1>>,
    constant1255: burn::module::Param<Tensor<1>>,
    constant1256: burn::module::Param<Tensor<1>>,
    constant1275: burn::module::Param<Tensor<1>>,
    constant1276: burn::module::Param<Tensor<1>>,
    constant1291: burn::module::Param<Tensor<1>>,
    constant1292: burn::module::Param<Tensor<1>>,
    linear66: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule12 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1169: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1170: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1171: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1172: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear63 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear64 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear65 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant1212: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1213: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1219: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1220: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1227: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1228: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1229: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1230: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1235: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1236: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1255: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1256: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1275: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1276: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1291: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1292: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear66 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1169,
            constant1170,
            constant1171,
            constant1172,
            linear63,
            linear64,
            linear65,
            constant1212,
            constant1213,
            constant1219,
            constant1220,
            constant1227,
            constant1228,
            constant1229,
            constant1230,
            constant1235,
            constant1236,
            constant1255,
            constant1256,
            constant1275,
            constant1276,
            constant1291,
            constant1292,
            linear66,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add107_out1: Tensor<3>,
        linear58_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze49_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean25_out1 = { add107_out1.clone().mean_dim(2usize) };
        let sub13_out1 = add107_out1.sub(reducemean25_out1);
        let constant1169_out1 = self.constant1169.val();
        let pow13_out1 = sub13_out1
            .clone()
            .powf((constant1169_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean26_out1 = { pow13_out1.mean_dim(2usize) };
        let constant1170_out1 = self.constant1170.val();
        let add108_out1 = reducemean26_out1
            .add((constant1170_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt13_out1 = add108_out1.sqrt();
        let div17_out1 = sub13_out1.div(sqrt13_out1);
        let constant1171_out1 = self.constant1171.val();
        let mul77_out1 = div17_out1
            .mul((constant1171_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1172_out1 = self.constant1172.val();
        let add109_out1 = mul77_out1
            .add((constant1172_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add110_out1 = add109_out1.clone().add(linear58_out1);
        let shape34_out1: [i64; 3] = {
            let axes = &add110_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice48_out1: [i64; 1] = shape34_out1[0..1].try_into().unwrap();
        let squeeze17_out1 = slice48_out1[0] as i64;
        let linear63_out1 = self.linear63.forward(concat16_out1);
        let reshape101_out1 = squeeze17_out1;
        let unsqueeze50_out1 = [reshape101_out1 as i64];
        let constant1182_out1: [i64; 1] = [13125i64];
        let constant1183_out1: [i64; 1] = [8i64];
        let constant1184_out1: [i64; 1] = [32i64];
        let concat69_out1: [i64; 4usize] = [
            &unsqueeze50_out1[..],
            &constant1182_out1[..],
            &constant1183_out1[..],
            &constant1184_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape102_out1 = linear63_out1.reshape(concat69_out1);
        let linear64_out1 = self.linear64.forward(add110_out1.clone());
        let reshape103_out1 = squeeze17_out1;
        let unsqueeze51_out1 = [reshape103_out1 as i64];
        let constant1189_out1: [i64; 1] = [300i64];
        let constant1190_out1: [i64; 1] = [8i64];
        let constant1191_out1: [i64; 1] = [3i64];
        let constant1192_out1: [i64; 1] = [4i64];
        let constant1193_out1: [i64; 1] = [2i64];
        let concat70_out1: [i64; 6usize] = [
            &unsqueeze51_out1[..],
            &constant1189_out1[..],
            &constant1190_out1[..],
            &constant1191_out1[..],
            &constant1192_out1[..],
            &constant1193_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape104_out1 = linear64_out1.reshape(concat70_out1);
        let linear65_out1 = self.linear65.forward(add110_out1);
        let reshape105_out1 = squeeze17_out1;
        let unsqueeze52_out1 = [reshape105_out1 as i64];
        let constant1198_out1: [i64; 1] = [300i64];
        let constant1199_out1: [i64; 1] = [8i64];
        let constant1200_out1: [i64; 1] = [12i64];
        let concat71_out1: [i64; 4usize] = [
            &unsqueeze52_out1[..],
            &constant1198_out1[..],
            &constant1199_out1[..],
            &constant1200_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape106_out1 = linear65_out1.reshape(concat71_out1);
        let softmax9_out1 = burn::tensor::activation::softmax(reshape106_out1, 3);
        let reshape107_out1 = squeeze17_out1;
        let unsqueeze53_out1 = [reshape107_out1 as i64];
        let constant1203_out1: [i64; 1] = [300i64];
        let constant1204_out1: [i64; 1] = [8i64];
        let constant1205_out1: [i64; 1] = [3i64];
        let constant1206_out1: [i64; 1] = [4i64];
        let concat72_out1: [i64; 5usize] = [
            &unsqueeze53_out1[..],
            &constant1203_out1[..],
            &constant1204_out1[..],
            &constant1205_out1[..],
            &constant1206_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape108_out1 = softmax9_out1.reshape(concat72_out1);
        let slice49_out1 = unsqueeze49_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze54_out1: Tensor<6> = slice49_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant1212_out1 = self.constant1212.val();
        let mul78_out1 = reshape104_out1
            .mul(
                (constant1212_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1213_out1 = self.constant1213.val();
        let add111_out1 = mul78_out1
            .add(
                (constant1213_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice50_out1 = unsqueeze49_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze55_out1: Tensor<6> = slice50_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul79_out1 = add111_out1.mul(unsqueeze55_out1);
        let constant1219_out1 = self.constant1219.val();
        let mul80_out1 = mul79_out1
            .mul(
                (constant1219_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1220_out1 = self.constant1220.val();
        let add112_out1 = mul80_out1
            .add(
                (constant1220_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add113_out1 = unsqueeze54_out1.add(add112_out1);
        let shape35_out1: [i64; 4] = {
            let axes = &reshape102_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice51_out1: [i64; 1] = shape35_out1[0..1].try_into().unwrap();
        let squeeze18_out1 = slice51_out1[0] as i64;
        let split_tensors = reshape102_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split4_out1, split4_out2, split4_out3] = split_tensors.try_into().unwrap();
        let constant1227_out1 = self.constant1227.val();
        let mul81_out1 = add113_out1
            .mul(
                (constant1227_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1228_out1 = self.constant1228.val();
        let add114_out1 = mul81_out1
            .add(
                (constant1228_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1229_out1 = self.constant1229.val();
        let mul82_out1 = add114_out1
            .mul(
                (constant1229_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1230_out1 = self.constant1230.val();
        let add115_out1 = mul82_out1
            .add(
                (constant1230_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape36_out1: [i64; 4] = {
            let axes = &split4_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice52_out1: [i64; 2] = shape36_out1[0..2].try_into().unwrap();
        let constant1234_out1: [i64; 1] = [-1i64];
        let concat73_out1: [i64; 3usize] = [&slice52_out1[..], &constant1234_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape109_out1 = split4_out1.reshape(concat73_out1);
        let transpose55_out1 = reshape109_out1.permute([0, 2, 1]);
        let cast26_out1 = squeeze18_out1 as f32;
        let constant1235_out1 = self.constant1235.val();
        let mul83_out1 = cast26_out1 * constant1235_out1;
        let constant1236_out1 = self.constant1236.val();
        let add116_out1 = mul83_out1.add(constant1236_out1);
        let cast27_out1 = add116_out1.int().cast(burn::tensor::DType::I64);
        let reshape110_out1 = cast27_out1.into_scalar::<i64>();
        let unsqueeze56_out1 = [reshape110_out1 as i64];
        let constant1239_out1: [i64; 1] = [32i64];
        let constant1240_out1: [i64; 1] = [100i64];
        let constant1241_out1: [i64; 1] = [100i64];
        let concat74_out1: [i64; 4usize] = [
            &unsqueeze56_out1[..],
            &constant1239_out1[..],
            &constant1240_out1[..],
            &constant1241_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape111_out1 = transpose55_out1.reshape(concat74_out1);
        let slice53_out1 = add115_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze19_out1 = slice53_out1.squeeze_dims::<5>(&[3]);
        let transpose56_out1 = squeeze19_out1.permute([0, 2, 1, 3, 4]);
        let shape37_out1: [i64; 5] = {
            let axes = &transpose56_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice54_out1: [i64; 3] = shape37_out1[2..5].try_into().unwrap();
        let constant1250_out1: [i64; 1] = [-1i64];
        let concat75_out1: [i64; 4usize] = [&constant1250_out1[..], &slice54_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape112_out1 = transpose56_out1.reshape(concat75_out1);
        let gridsample10_out1 = reshape111_out1
            .grid_sample_2d(
                reshape112_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape38_out1: [i64; 4] = {
            let axes = &split4_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice55_out1: [i64; 2] = shape38_out1[0..2].try_into().unwrap();
        let constant1254_out1: [i64; 1] = [-1i64];
        let concat76_out1: [i64; 3usize] = [&slice55_out1[..], &constant1254_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape113_out1 = split4_out2.reshape(concat76_out1);
        let transpose57_out1 = reshape113_out1.permute([0, 2, 1]);
        let cast28_out1 = squeeze18_out1 as f32;
        let constant1255_out1 = self.constant1255.val();
        let mul84_out1 = cast28_out1 * constant1255_out1;
        let constant1256_out1 = self.constant1256.val();
        let add117_out1 = mul84_out1.add(constant1256_out1);
        let cast29_out1 = add117_out1.int().cast(burn::tensor::DType::I64);
        let reshape114_out1 = cast29_out1.into_scalar::<i64>();
        let unsqueeze57_out1 = [reshape114_out1 as i64];
        let constant1259_out1: [i64; 1] = [32i64];
        let constant1260_out1: [i64; 1] = [50i64];
        let constant1261_out1: [i64; 1] = [50i64];
        let concat77_out1: [i64; 4usize] = [
            &unsqueeze57_out1[..],
            &constant1259_out1[..],
            &constant1260_out1[..],
            &constant1261_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape115_out1 = transpose57_out1.reshape(concat77_out1);
        let slice56_out1 = add115_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze20_out1 = slice56_out1.squeeze_dims::<5>(&[3]);
        let transpose58_out1 = squeeze20_out1.permute([0, 2, 1, 3, 4]);
        let shape39_out1: [i64; 5] = {
            let axes = &transpose58_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice57_out1: [i64; 3] = shape39_out1[2..5].try_into().unwrap();
        let constant1270_out1: [i64; 1] = [-1i64];
        let concat78_out1: [i64; 4usize] = [&constant1270_out1[..], &slice57_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape116_out1 = transpose58_out1.reshape(concat78_out1);
        let gridsample11_out1 = reshape115_out1
            .grid_sample_2d(
                reshape116_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape40_out1: [i64; 4] = {
            let axes = &split4_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice58_out1: [i64; 2] = shape40_out1[0..2].try_into().unwrap();
        let constant1274_out1: [i64; 1] = [-1i64];
        let concat79_out1: [i64; 3usize] = [&slice58_out1[..], &constant1274_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape117_out1 = split4_out3.reshape(concat79_out1);
        let transpose59_out1 = reshape117_out1.permute([0, 2, 1]);
        let cast30_out1 = squeeze18_out1 as f32;
        let constant1275_out1 = self.constant1275.val();
        let mul85_out1 = cast30_out1 * constant1275_out1;
        let constant1276_out1 = self.constant1276.val();
        let add118_out1 = mul85_out1.add(constant1276_out1);
        let cast31_out1 = add118_out1.int().cast(burn::tensor::DType::I64);
        let reshape118_out1 = cast31_out1.into_scalar::<i64>();
        let unsqueeze58_out1 = [reshape118_out1 as i64];
        let constant1279_out1: [i64; 1] = [32i64];
        let constant1280_out1: [i64; 1] = [25i64];
        let constant1281_out1: [i64; 1] = [25i64];
        let concat80_out1: [i64; 4usize] = [
            &unsqueeze58_out1[..],
            &constant1279_out1[..],
            &constant1280_out1[..],
            &constant1281_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape119_out1 = transpose59_out1.reshape(concat80_out1);
        let slice59_out1 = add115_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze21_out1 = slice59_out1.squeeze_dims::<5>(&[3]);
        let transpose60_out1 = squeeze21_out1.permute([0, 2, 1, 3, 4]);
        let shape41_out1: [i64; 5] = {
            let axes = &transpose60_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice60_out1: [i64; 3] = shape41_out1[2..5].try_into().unwrap();
        let constant1290_out1: [i64; 1] = [-1i64];
        let concat81_out1: [i64; 4usize] = [&constant1290_out1[..], &slice60_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape120_out1 = transpose60_out1.reshape(concat81_out1);
        let gridsample12_out1 = reshape119_out1
            .grid_sample_2d(
                reshape120_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose61_out1 = reshape108_out1.permute([0, 2, 1, 3, 4]);
        let cast32_out1 = squeeze18_out1 as f32;
        let constant1291_out1 = self.constant1291.val();
        let mul86_out1 = cast32_out1 * constant1291_out1;
        let constant1292_out1 = self.constant1292.val();
        let add119_out1 = mul86_out1.add(constant1292_out1);
        let cast33_out1 = add119_out1.int().cast(burn::tensor::DType::I64);
        let reshape121_out1 = cast33_out1.into_scalar::<i64>();
        let unsqueeze59_out1 = [reshape121_out1 as i64];
        let constant1295_out1: [i64; 1] = [1i64];
        let constant1296_out1: [i64; 1] = [300i64];
        let constant1297_out1: [i64; 1] = [12i64];
        let concat82_out1: [i64; 4usize] = [
            &unsqueeze59_out1[..],
            &constant1295_out1[..],
            &constant1296_out1[..],
            &constant1297_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape122_out1 = transpose61_out1.reshape(concat82_out1);
        let unsqueeze60_out1: Tensor<5> = gridsample10_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze61_out1: Tensor<5> = gridsample11_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze62_out1: Tensor<5> = gridsample12_out1.unsqueeze_dims::<5>(&[3]);
        let concat83_out1 = burn::tensor::Tensor::cat(
            [unsqueeze60_out1, unsqueeze61_out1, unsqueeze62_out1].into(),
            3,
        );
        let shape42_out1: [i64; 5] = {
            let axes = &concat83_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice61_out1: [i64; 3] = shape42_out1[0..3].try_into().unwrap();
        let constant1304_out1: [i64; 1] = [-1i64];
        let concat84_out1: [i64; 4usize] = [&slice61_out1[..], &constant1304_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape123_out1 = concat83_out1.reshape(concat84_out1);
        let mul87_out1 = reshape123_out1.mul(reshape122_out1);
        let reducesum4_out1 = {
            mul87_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape124_out1 = squeeze18_out1;
        let unsqueeze63_out1 = [reshape124_out1 as i64];
        let constant1308_out1: [i64; 1] = [256i64];
        let constant1309_out1: [i64; 1] = [300i64];
        let concat85_out1: [i64; 3usize] = [
            &unsqueeze63_out1[..],
            &constant1308_out1[..],
            &constant1309_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape125_out1 = reducesum4_out1.reshape(concat85_out1);
        let transpose62_out1 = reshape125_out1.permute([0, 2, 1]);
        let linear66_out1 = self.linear66.forward(transpose62_out1);
        let add120_out1 = add109_out1.add(linear66_out1);
        add120_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule13 {
    constant1312: burn::module::Param<Tensor<1>>,
    constant1313: burn::module::Param<Tensor<1>>,
    constant1314: burn::module::Param<Tensor<1>>,
    constant1315: burn::module::Param<Tensor<1>>,
    linear67: Linear,
    linear68: Linear,
    constant1320: burn::module::Param<Tensor<1>>,
    constant1321: burn::module::Param<Tensor<1>>,
    constant1322: burn::module::Param<Tensor<1>>,
    constant1323: burn::module::Param<Tensor<1>>,
    linear69: Linear,
    linear70: Linear,
    linear71: Linear,
    constant1334: burn::module::Param<Tensor<1>>,
    constant1335: burn::module::Param<Tensor<1>>,
    linear72: Linear,
    linear73: Linear,
    linear74: Linear,
    linear75: Linear,
    linear76: Linear,
    constant1348: burn::module::Param<Tensor<1>>,
    constant1349: burn::module::Param<Tensor<1>>,
    linear77: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule13 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1312: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1313: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1314: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1315: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear67 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear68 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant1320: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1321: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1322: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1323: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear69 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear70 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear71 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant1334: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1335: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear72 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear73 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear74 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear75 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear76 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant1348: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1349: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear77 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1312,
            constant1313,
            constant1314,
            constant1315,
            linear67,
            linear68,
            constant1320,
            constant1321,
            constant1322,
            constant1323,
            linear69,
            linear70,
            linear71,
            constant1334,
            constant1335,
            linear72,
            linear73,
            linear74,
            linear75,
            linear76,
            constant1348,
            constant1349,
            linear77,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add120_out1: Tensor<3>,
        sigmoid28_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let reducemean27_out1 = { add120_out1.clone().mean_dim(2usize) };
        let sub14_out1 = add120_out1.sub(reducemean27_out1);
        let constant1312_out1 = self.constant1312.val();
        let pow14_out1 = sub14_out1
            .clone()
            .powf((constant1312_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean28_out1 = { pow14_out1.mean_dim(2usize) };
        let constant1313_out1 = self.constant1313.val();
        let add121_out1 = reducemean28_out1
            .add((constant1313_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt14_out1 = add121_out1.sqrt();
        let div18_out1 = sub14_out1.div(sqrt14_out1);
        let constant1314_out1 = self.constant1314.val();
        let mul88_out1 = div18_out1
            .mul((constant1314_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1315_out1 = self.constant1315.val();
        let add122_out1 = mul88_out1
            .add((constant1315_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear67_out1 = self.linear67.forward(add122_out1.clone());
        let relu69_out1 = burn::tensor::activation::relu(linear67_out1);
        let linear68_out1 = self.linear68.forward(relu69_out1);
        let add123_out1 = add122_out1.add(linear68_out1);
        let reducemean29_out1 = { add123_out1.clone().mean_dim(2usize) };
        let sub15_out1 = add123_out1.sub(reducemean29_out1);
        let constant1320_out1 = self.constant1320.val();
        let pow15_out1 = sub15_out1
            .clone()
            .powf((constant1320_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean30_out1 = { pow15_out1.mean_dim(2usize) };
        let constant1321_out1 = self.constant1321.val();
        let add124_out1 = reducemean30_out1
            .add((constant1321_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt15_out1 = add124_out1.sqrt();
        let div19_out1 = sub15_out1.div(sqrt15_out1);
        let constant1322_out1 = self.constant1322.val();
        let mul89_out1 = div19_out1
            .mul((constant1322_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1323_out1 = self.constant1323.val();
        let add125_out1 = mul89_out1
            .add((constant1323_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear69_out1 = self.linear69.forward(add125_out1.clone());
        let relu70_out1 = burn::tensor::activation::relu(linear69_out1);
        let linear70_out1 = self.linear70.forward(relu70_out1);
        let relu71_out1 = burn::tensor::activation::relu(linear70_out1);
        let linear71_out1 = self.linear71.forward(relu71_out1);
        let clip10_out1 = sigmoid28_out1.clamp(0f64, 1f64);
        let clip11_out1 = clip10_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant1334_out1 = self.constant1334.val();
        let mul90_out1 = clip10_out1
            .mul((constant1334_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1335_out1 = self.constant1335.val();
        let add126_out1 = mul90_out1
            .add((constant1335_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip12_out1 = add126_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div20_out1 = clip11_out1.div(clip12_out1);
        let log4_out1 = div20_out1.log();
        let add127_out1 = linear71_out1.add(log4_out1);
        let sigmoid29_out1 = burn::tensor::activation::sigmoid(add127_out1);
        let unsqueeze64_out1: Tensor<4> = sigmoid29_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear72_out1 = self.linear72.forward(sigmoid29_out1.clone());
        let relu72_out1 = burn::tensor::activation::relu(linear72_out1);
        let linear73_out1 = self.linear73.forward(relu72_out1);
        let add128_out1 = add125_out1.clone().add(linear73_out1.clone());
        let linear74_out1 = self.linear74.forward(add128_out1.clone());
        let reshape126_out1 = linear74_out1.reshape([0, 0, 8, 32]);
        let transpose63_out1 = reshape126_out1.permute([0, 2, 1, 3]);
        let linear75_out1 = self.linear75.forward(add128_out1);
        let reshape127_out1 = linear75_out1.reshape([0, 0, 8, 32]);
        let transpose64_out1 = reshape127_out1.permute([0, 2, 1, 3]);
        let linear76_out1 = self.linear76.forward(add125_out1.clone());
        let reshape128_out1 = linear76_out1.reshape([0, 0, 8, 32]);
        let transpose65_out1 = reshape128_out1.permute([0, 2, 1, 3]);
        let transpose66_out1 = transpose64_out1.permute([0, 1, 3, 2]);
        let matmul87_out1 = transpose63_out1.matmul(transpose66_out1);
        let constant1348_out1 = self.constant1348.val();
        let mul91_out1 = matmul87_out1
            .mul((constant1348_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant1349_out1 = self.constant1349.val();
        let add129_out1 = mul91_out1
            .add((constant1349_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax10_out1 = burn::tensor::activation::softmax(add129_out1, 3);
        let matmul88_out1 = softmax10_out1.matmul(transpose65_out1);
        let transpose67_out1 = matmul88_out1.permute([0, 2, 1, 3]);
        let reshape129_out1 = transpose67_out1.reshape([0, 0, 256]);
        let linear77_out1 = self.linear77.forward(reshape129_out1);
        let add130_out1 = add125_out1.add(linear77_out1);
        (add130_out1, linear73_out1, unsqueeze64_out1, sigmoid29_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule14 {
    constant1353: burn::module::Param<Tensor<1>>,
    constant1354: burn::module::Param<Tensor<1>>,
    constant1355: burn::module::Param<Tensor<1>>,
    constant1356: burn::module::Param<Tensor<1>>,
    linear78: Linear,
    linear79: Linear,
    linear80: Linear,
    constant1396: burn::module::Param<Tensor<1>>,
    constant1397: burn::module::Param<Tensor<1>>,
    constant1403: burn::module::Param<Tensor<1>>,
    constant1404: burn::module::Param<Tensor<1>>,
    constant1411: burn::module::Param<Tensor<1>>,
    constant1412: burn::module::Param<Tensor<1>>,
    constant1413: burn::module::Param<Tensor<1>>,
    constant1414: burn::module::Param<Tensor<1>>,
    constant1419: burn::module::Param<Tensor<1>>,
    constant1420: burn::module::Param<Tensor<1>>,
    constant1439: burn::module::Param<Tensor<1>>,
    constant1440: burn::module::Param<Tensor<1>>,
    constant1459: burn::module::Param<Tensor<1>>,
    constant1460: burn::module::Param<Tensor<1>>,
    constant1475: burn::module::Param<Tensor<1>>,
    constant1476: burn::module::Param<Tensor<1>>,
    linear81: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule14 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1353: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1354: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1355: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1356: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear78 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear79 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear80 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant1396: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1397: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1403: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1404: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1411: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1412: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1413: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1414: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1419: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1420: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1439: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1440: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1459: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1460: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1475: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1476: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear81 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1353,
            constant1354,
            constant1355,
            constant1356,
            linear78,
            linear79,
            linear80,
            constant1396,
            constant1397,
            constant1403,
            constant1404,
            constant1411,
            constant1412,
            constant1413,
            constant1414,
            constant1419,
            constant1420,
            constant1439,
            constant1440,
            constant1459,
            constant1460,
            constant1475,
            constant1476,
            linear81,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add130_out1: Tensor<3>,
        linear73_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze64_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean31_out1 = { add130_out1.clone().mean_dim(2usize) };
        let sub16_out1 = add130_out1.sub(reducemean31_out1);
        let constant1353_out1 = self.constant1353.val();
        let pow16_out1 = sub16_out1
            .clone()
            .powf((constant1353_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean32_out1 = { pow16_out1.mean_dim(2usize) };
        let constant1354_out1 = self.constant1354.val();
        let add131_out1 = reducemean32_out1
            .add((constant1354_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt16_out1 = add131_out1.sqrt();
        let div21_out1 = sub16_out1.div(sqrt16_out1);
        let constant1355_out1 = self.constant1355.val();
        let mul92_out1 = div21_out1
            .mul((constant1355_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1356_out1 = self.constant1356.val();
        let add132_out1 = mul92_out1
            .add((constant1356_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add133_out1 = add132_out1.clone().add(linear73_out1);
        let shape43_out1: [i64; 3] = {
            let axes = &add133_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice62_out1: [i64; 1] = shape43_out1[0..1].try_into().unwrap();
        let squeeze22_out1 = slice62_out1[0] as i64;
        let linear78_out1 = self.linear78.forward(concat16_out1);
        let reshape130_out1 = squeeze22_out1;
        let unsqueeze65_out1 = [reshape130_out1 as i64];
        let constant1366_out1: [i64; 1] = [13125i64];
        let constant1367_out1: [i64; 1] = [8i64];
        let constant1368_out1: [i64; 1] = [32i64];
        let concat86_out1: [i64; 4usize] = [
            &unsqueeze65_out1[..],
            &constant1366_out1[..],
            &constant1367_out1[..],
            &constant1368_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape131_out1 = linear78_out1.reshape(concat86_out1);
        let linear79_out1 = self.linear79.forward(add133_out1.clone());
        let reshape132_out1 = squeeze22_out1;
        let unsqueeze66_out1 = [reshape132_out1 as i64];
        let constant1373_out1: [i64; 1] = [300i64];
        let constant1374_out1: [i64; 1] = [8i64];
        let constant1375_out1: [i64; 1] = [3i64];
        let constant1376_out1: [i64; 1] = [4i64];
        let constant1377_out1: [i64; 1] = [2i64];
        let concat87_out1: [i64; 6usize] = [
            &unsqueeze66_out1[..],
            &constant1373_out1[..],
            &constant1374_out1[..],
            &constant1375_out1[..],
            &constant1376_out1[..],
            &constant1377_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape133_out1 = linear79_out1.reshape(concat87_out1);
        let linear80_out1 = self.linear80.forward(add133_out1);
        let reshape134_out1 = squeeze22_out1;
        let unsqueeze67_out1 = [reshape134_out1 as i64];
        let constant1382_out1: [i64; 1] = [300i64];
        let constant1383_out1: [i64; 1] = [8i64];
        let constant1384_out1: [i64; 1] = [12i64];
        let concat88_out1: [i64; 4usize] = [
            &unsqueeze67_out1[..],
            &constant1382_out1[..],
            &constant1383_out1[..],
            &constant1384_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape135_out1 = linear80_out1.reshape(concat88_out1);
        let softmax11_out1 = burn::tensor::activation::softmax(reshape135_out1, 3);
        let reshape136_out1 = squeeze22_out1;
        let unsqueeze68_out1 = [reshape136_out1 as i64];
        let constant1387_out1: [i64; 1] = [300i64];
        let constant1388_out1: [i64; 1] = [8i64];
        let constant1389_out1: [i64; 1] = [3i64];
        let constant1390_out1: [i64; 1] = [4i64];
        let concat89_out1: [i64; 5usize] = [
            &unsqueeze68_out1[..],
            &constant1387_out1[..],
            &constant1388_out1[..],
            &constant1389_out1[..],
            &constant1390_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape137_out1 = softmax11_out1.reshape(concat89_out1);
        let slice63_out1 = unsqueeze64_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze69_out1: Tensor<6> = slice63_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant1396_out1 = self.constant1396.val();
        let mul93_out1 = reshape133_out1
            .mul(
                (constant1396_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1397_out1 = self.constant1397.val();
        let add134_out1 = mul93_out1
            .add(
                (constant1397_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice64_out1 = unsqueeze64_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze70_out1: Tensor<6> = slice64_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul94_out1 = add134_out1.mul(unsqueeze70_out1);
        let constant1403_out1 = self.constant1403.val();
        let mul95_out1 = mul94_out1
            .mul(
                (constant1403_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1404_out1 = self.constant1404.val();
        let add135_out1 = mul95_out1
            .add(
                (constant1404_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add136_out1 = unsqueeze69_out1.add(add135_out1);
        let shape44_out1: [i64; 4] = {
            let axes = &reshape131_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice65_out1: [i64; 1] = shape44_out1[0..1].try_into().unwrap();
        let squeeze23_out1 = slice65_out1[0] as i64;
        let split_tensors = reshape131_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split5_out1, split5_out2, split5_out3] = split_tensors.try_into().unwrap();
        let constant1411_out1 = self.constant1411.val();
        let mul96_out1 = add136_out1
            .mul(
                (constant1411_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1412_out1 = self.constant1412.val();
        let add137_out1 = mul96_out1
            .add(
                (constant1412_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1413_out1 = self.constant1413.val();
        let mul97_out1 = add137_out1
            .mul(
                (constant1413_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1414_out1 = self.constant1414.val();
        let add138_out1 = mul97_out1
            .add(
                (constant1414_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape45_out1: [i64; 4] = {
            let axes = &split5_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice66_out1: [i64; 2] = shape45_out1[0..2].try_into().unwrap();
        let constant1418_out1: [i64; 1] = [-1i64];
        let concat90_out1: [i64; 3usize] = [&slice66_out1[..], &constant1418_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape138_out1 = split5_out1.reshape(concat90_out1);
        let transpose68_out1 = reshape138_out1.permute([0, 2, 1]);
        let cast34_out1 = squeeze23_out1 as f32;
        let constant1419_out1 = self.constant1419.val();
        let mul98_out1 = cast34_out1 * constant1419_out1;
        let constant1420_out1 = self.constant1420.val();
        let add139_out1 = mul98_out1.add(constant1420_out1);
        let cast35_out1 = add139_out1.int().cast(burn::tensor::DType::I64);
        let reshape139_out1 = cast35_out1.into_scalar::<i64>();
        let unsqueeze71_out1 = [reshape139_out1 as i64];
        let constant1423_out1: [i64; 1] = [32i64];
        let constant1424_out1: [i64; 1] = [100i64];
        let constant1425_out1: [i64; 1] = [100i64];
        let concat91_out1: [i64; 4usize] = [
            &unsqueeze71_out1[..],
            &constant1423_out1[..],
            &constant1424_out1[..],
            &constant1425_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape140_out1 = transpose68_out1.reshape(concat91_out1);
        let slice67_out1 = add138_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze24_out1 = slice67_out1.squeeze_dims::<5>(&[3]);
        let transpose69_out1 = squeeze24_out1.permute([0, 2, 1, 3, 4]);
        let shape46_out1: [i64; 5] = {
            let axes = &transpose69_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice68_out1: [i64; 3] = shape46_out1[2..5].try_into().unwrap();
        let constant1434_out1: [i64; 1] = [-1i64];
        let concat92_out1: [i64; 4usize] = [&constant1434_out1[..], &slice68_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape141_out1 = transpose69_out1.reshape(concat92_out1);
        let gridsample13_out1 = reshape140_out1
            .grid_sample_2d(
                reshape141_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape47_out1: [i64; 4] = {
            let axes = &split5_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice69_out1: [i64; 2] = shape47_out1[0..2].try_into().unwrap();
        let constant1438_out1: [i64; 1] = [-1i64];
        let concat93_out1: [i64; 3usize] = [&slice69_out1[..], &constant1438_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape142_out1 = split5_out2.reshape(concat93_out1);
        let transpose70_out1 = reshape142_out1.permute([0, 2, 1]);
        let cast36_out1 = squeeze23_out1 as f32;
        let constant1439_out1 = self.constant1439.val();
        let mul99_out1 = cast36_out1 * constant1439_out1;
        let constant1440_out1 = self.constant1440.val();
        let add140_out1 = mul99_out1.add(constant1440_out1);
        let cast37_out1 = add140_out1.int().cast(burn::tensor::DType::I64);
        let reshape143_out1 = cast37_out1.into_scalar::<i64>();
        let unsqueeze72_out1 = [reshape143_out1 as i64];
        let constant1443_out1: [i64; 1] = [32i64];
        let constant1444_out1: [i64; 1] = [50i64];
        let constant1445_out1: [i64; 1] = [50i64];
        let concat94_out1: [i64; 4usize] = [
            &unsqueeze72_out1[..],
            &constant1443_out1[..],
            &constant1444_out1[..],
            &constant1445_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape144_out1 = transpose70_out1.reshape(concat94_out1);
        let slice70_out1 = add138_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze25_out1 = slice70_out1.squeeze_dims::<5>(&[3]);
        let transpose71_out1 = squeeze25_out1.permute([0, 2, 1, 3, 4]);
        let shape48_out1: [i64; 5] = {
            let axes = &transpose71_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice71_out1: [i64; 3] = shape48_out1[2..5].try_into().unwrap();
        let constant1454_out1: [i64; 1] = [-1i64];
        let concat95_out1: [i64; 4usize] = [&constant1454_out1[..], &slice71_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape145_out1 = transpose71_out1.reshape(concat95_out1);
        let gridsample14_out1 = reshape144_out1
            .grid_sample_2d(
                reshape145_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape49_out1: [i64; 4] = {
            let axes = &split5_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice72_out1: [i64; 2] = shape49_out1[0..2].try_into().unwrap();
        let constant1458_out1: [i64; 1] = [-1i64];
        let concat96_out1: [i64; 3usize] = [&slice72_out1[..], &constant1458_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape146_out1 = split5_out3.reshape(concat96_out1);
        let transpose72_out1 = reshape146_out1.permute([0, 2, 1]);
        let cast38_out1 = squeeze23_out1 as f32;
        let constant1459_out1 = self.constant1459.val();
        let mul100_out1 = cast38_out1 * constant1459_out1;
        let constant1460_out1 = self.constant1460.val();
        let add141_out1 = mul100_out1.add(constant1460_out1);
        let cast39_out1 = add141_out1.int().cast(burn::tensor::DType::I64);
        let reshape147_out1 = cast39_out1.into_scalar::<i64>();
        let unsqueeze73_out1 = [reshape147_out1 as i64];
        let constant1463_out1: [i64; 1] = [32i64];
        let constant1464_out1: [i64; 1] = [25i64];
        let constant1465_out1: [i64; 1] = [25i64];
        let concat97_out1: [i64; 4usize] = [
            &unsqueeze73_out1[..],
            &constant1463_out1[..],
            &constant1464_out1[..],
            &constant1465_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape148_out1 = transpose72_out1.reshape(concat97_out1);
        let slice73_out1 = add138_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze26_out1 = slice73_out1.squeeze_dims::<5>(&[3]);
        let transpose73_out1 = squeeze26_out1.permute([0, 2, 1, 3, 4]);
        let shape50_out1: [i64; 5] = {
            let axes = &transpose73_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice74_out1: [i64; 3] = shape50_out1[2..5].try_into().unwrap();
        let constant1474_out1: [i64; 1] = [-1i64];
        let concat98_out1: [i64; 4usize] = [&constant1474_out1[..], &slice74_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape149_out1 = transpose73_out1.reshape(concat98_out1);
        let gridsample15_out1 = reshape148_out1
            .grid_sample_2d(
                reshape149_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose74_out1 = reshape137_out1.permute([0, 2, 1, 3, 4]);
        let cast40_out1 = squeeze23_out1 as f32;
        let constant1475_out1 = self.constant1475.val();
        let mul101_out1 = cast40_out1 * constant1475_out1;
        let constant1476_out1 = self.constant1476.val();
        let add142_out1 = mul101_out1.add(constant1476_out1);
        let cast41_out1 = add142_out1.int().cast(burn::tensor::DType::I64);
        let reshape150_out1 = cast41_out1.into_scalar::<i64>();
        let unsqueeze74_out1 = [reshape150_out1 as i64];
        let constant1479_out1: [i64; 1] = [1i64];
        let constant1480_out1: [i64; 1] = [300i64];
        let constant1481_out1: [i64; 1] = [12i64];
        let concat99_out1: [i64; 4usize] = [
            &unsqueeze74_out1[..],
            &constant1479_out1[..],
            &constant1480_out1[..],
            &constant1481_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape151_out1 = transpose74_out1.reshape(concat99_out1);
        let unsqueeze75_out1: Tensor<5> = gridsample13_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze76_out1: Tensor<5> = gridsample14_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze77_out1: Tensor<5> = gridsample15_out1.unsqueeze_dims::<5>(&[3]);
        let concat100_out1 = burn::tensor::Tensor::cat(
            [unsqueeze75_out1, unsqueeze76_out1, unsqueeze77_out1].into(),
            3,
        );
        let shape51_out1: [i64; 5] = {
            let axes = &concat100_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice75_out1: [i64; 3] = shape51_out1[0..3].try_into().unwrap();
        let constant1488_out1: [i64; 1] = [-1i64];
        let concat101_out1: [i64; 4usize] = [&slice75_out1[..], &constant1488_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape152_out1 = concat100_out1.reshape(concat101_out1);
        let mul102_out1 = reshape152_out1.mul(reshape151_out1);
        let reducesum5_out1 = {
            mul102_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape153_out1 = squeeze23_out1;
        let unsqueeze78_out1 = [reshape153_out1 as i64];
        let constant1492_out1: [i64; 1] = [256i64];
        let constant1493_out1: [i64; 1] = [300i64];
        let concat102_out1: [i64; 3usize] = [
            &unsqueeze78_out1[..],
            &constant1492_out1[..],
            &constant1493_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape154_out1 = reducesum5_out1.reshape(concat102_out1);
        let transpose75_out1 = reshape154_out1.permute([0, 2, 1]);
        let linear81_out1 = self.linear81.forward(transpose75_out1);
        let add143_out1 = add132_out1.add(linear81_out1);
        add143_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule15 {
    constant1496: burn::module::Param<Tensor<1>>,
    constant1497: burn::module::Param<Tensor<1>>,
    constant1498: burn::module::Param<Tensor<1>>,
    constant1499: burn::module::Param<Tensor<1>>,
    linear82: Linear,
    linear83: Linear,
    constant1504: burn::module::Param<Tensor<1>>,
    constant1505: burn::module::Param<Tensor<1>>,
    constant1506: burn::module::Param<Tensor<1>>,
    constant1507: burn::module::Param<Tensor<1>>,
    linear84: Linear,
    linear85: Linear,
    linear86: Linear,
    constant1518: burn::module::Param<Tensor<1>>,
    constant1519: burn::module::Param<Tensor<1>>,
    linear87: Linear,
    linear88: Linear,
    linear89: Linear,
    linear90: Linear,
    linear91: Linear,
    constant1532: burn::module::Param<Tensor<1>>,
    constant1533: burn::module::Param<Tensor<1>>,
    linear92: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule15 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1496: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1497: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1498: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1499: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear82 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear83 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant1504: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1505: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1506: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1507: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear84 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear85 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear86 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant1518: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1519: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear87 = LinearConfig::new(4, 512).with_bias(true).init(device);
        let linear88 = LinearConfig::new(512, 256).with_bias(true).init(device);
        let linear89 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear90 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear91 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let constant1532: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1533: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear92 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1496,
            constant1497,
            constant1498,
            constant1499,
            linear82,
            linear83,
            constant1504,
            constant1505,
            constant1506,
            constant1507,
            linear84,
            linear85,
            linear86,
            constant1518,
            constant1519,
            linear87,
            linear88,
            linear89,
            linear90,
            linear91,
            constant1532,
            constant1533,
            linear92,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add143_out1: Tensor<3>,
        sigmoid29_out1: Tensor<3>,
    ) -> (Tensor<3>, Tensor<3>, Tensor<4>, Tensor<3>) {
        let reducemean33_out1 = { add143_out1.clone().mean_dim(2usize) };
        let sub17_out1 = add143_out1.sub(reducemean33_out1);
        let constant1496_out1 = self.constant1496.val();
        let pow17_out1 = sub17_out1
            .clone()
            .powf((constant1496_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean34_out1 = { pow17_out1.mean_dim(2usize) };
        let constant1497_out1 = self.constant1497.val();
        let add144_out1 = reducemean34_out1
            .add((constant1497_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt17_out1 = add144_out1.sqrt();
        let div22_out1 = sub17_out1.div(sqrt17_out1);
        let constant1498_out1 = self.constant1498.val();
        let mul103_out1 = div22_out1
            .mul((constant1498_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1499_out1 = self.constant1499.val();
        let add145_out1 = mul103_out1
            .add((constant1499_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear82_out1 = self.linear82.forward(add145_out1.clone());
        let relu73_out1 = burn::tensor::activation::relu(linear82_out1);
        let linear83_out1 = self.linear83.forward(relu73_out1);
        let add146_out1 = add145_out1.add(linear83_out1);
        let reducemean35_out1 = { add146_out1.clone().mean_dim(2usize) };
        let sub18_out1 = add146_out1.sub(reducemean35_out1);
        let constant1504_out1 = self.constant1504.val();
        let pow18_out1 = sub18_out1
            .clone()
            .powf((constant1504_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean36_out1 = { pow18_out1.mean_dim(2usize) };
        let constant1505_out1 = self.constant1505.val();
        let add147_out1 = reducemean36_out1
            .add((constant1505_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt18_out1 = add147_out1.sqrt();
        let div23_out1 = sub18_out1.div(sqrt18_out1);
        let constant1506_out1 = self.constant1506.val();
        let mul104_out1 = div23_out1
            .mul((constant1506_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1507_out1 = self.constant1507.val();
        let add148_out1 = mul104_out1
            .add((constant1507_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear84_out1 = self.linear84.forward(add148_out1.clone());
        let relu74_out1 = burn::tensor::activation::relu(linear84_out1);
        let linear85_out1 = self.linear85.forward(relu74_out1);
        let relu75_out1 = burn::tensor::activation::relu(linear85_out1);
        let linear86_out1 = self.linear86.forward(relu75_out1);
        let clip13_out1 = sigmoid29_out1.clamp(0f64, 1f64);
        let clip14_out1 = clip13_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant1518_out1 = self.constant1518.val();
        let mul105_out1 = clip13_out1
            .mul((constant1518_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1519_out1 = self.constant1519.val();
        let add149_out1 = mul105_out1
            .add((constant1519_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip15_out1 = add149_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div24_out1 = clip14_out1.div(clip15_out1);
        let log5_out1 = div24_out1.log();
        let add150_out1 = linear86_out1.add(log5_out1);
        let sigmoid30_out1 = burn::tensor::activation::sigmoid(add150_out1);
        let unsqueeze79_out1: Tensor<4> = sigmoid30_out1
            .clone()
            .unsqueeze_dims::<4>(&[2]);
        let linear87_out1 = self.linear87.forward(sigmoid30_out1.clone());
        let relu76_out1 = burn::tensor::activation::relu(linear87_out1);
        let linear88_out1 = self.linear88.forward(relu76_out1);
        let add151_out1 = add148_out1.clone().add(linear88_out1.clone());
        let linear89_out1 = self.linear89.forward(add151_out1.clone());
        let reshape155_out1 = linear89_out1.reshape([0, 0, 8, 32]);
        let transpose76_out1 = reshape155_out1.permute([0, 2, 1, 3]);
        let linear90_out1 = self.linear90.forward(add151_out1);
        let reshape156_out1 = linear90_out1.reshape([0, 0, 8, 32]);
        let transpose77_out1 = reshape156_out1.permute([0, 2, 1, 3]);
        let linear91_out1 = self.linear91.forward(add148_out1.clone());
        let reshape157_out1 = linear91_out1.reshape([0, 0, 8, 32]);
        let transpose78_out1 = reshape157_out1.permute([0, 2, 1, 3]);
        let transpose79_out1 = transpose77_out1.permute([0, 1, 3, 2]);
        let matmul104_out1 = transpose76_out1.matmul(transpose79_out1);
        let constant1532_out1 = self.constant1532.val();
        let mul106_out1 = matmul104_out1
            .mul((constant1532_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant1533_out1 = self.constant1533.val();
        let add152_out1 = mul106_out1
            .add((constant1533_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let softmax12_out1 = burn::tensor::activation::softmax(add152_out1, 3);
        let matmul105_out1 = softmax12_out1.matmul(transpose78_out1);
        let transpose80_out1 = matmul105_out1.permute([0, 2, 1, 3]);
        let reshape158_out1 = transpose80_out1.reshape([0, 0, 256]);
        let linear92_out1 = self.linear92.forward(reshape158_out1);
        let add153_out1 = add148_out1.add(linear92_out1);
        (add153_out1, linear88_out1, unsqueeze79_out1, sigmoid30_out1)
    }
}
#[derive(Module, Debug)]
pub struct Submodule16 {
    constant1537: burn::module::Param<Tensor<1>>,
    constant1538: burn::module::Param<Tensor<1>>,
    constant1539: burn::module::Param<Tensor<1>>,
    constant1540: burn::module::Param<Tensor<1>>,
    linear93: Linear,
    linear94: Linear,
    linear95: Linear,
    constant1580: burn::module::Param<Tensor<1>>,
    constant1581: burn::module::Param<Tensor<1>>,
    constant1587: burn::module::Param<Tensor<1>>,
    constant1588: burn::module::Param<Tensor<1>>,
    constant1595: burn::module::Param<Tensor<1>>,
    constant1596: burn::module::Param<Tensor<1>>,
    constant1597: burn::module::Param<Tensor<1>>,
    constant1598: burn::module::Param<Tensor<1>>,
    constant1603: burn::module::Param<Tensor<1>>,
    constant1604: burn::module::Param<Tensor<1>>,
    constant1623: burn::module::Param<Tensor<1>>,
    constant1624: burn::module::Param<Tensor<1>>,
    constant1643: burn::module::Param<Tensor<1>>,
    constant1644: burn::module::Param<Tensor<1>>,
    constant1659: burn::module::Param<Tensor<1>>,
    constant1660: burn::module::Param<Tensor<1>>,
    linear96: Linear,
    #[module(skip)]
    device: Device,
}
impl Submodule16 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1537: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1538: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1539: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1540: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear93 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear94 = LinearConfig::new(256, 192).with_bias(true).init(device);
        let linear95 = LinearConfig::new(256, 96).with_bias(true).init(device);
        let constant1580: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1581: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1587: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1588: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1595: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1596: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1597: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1598: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([-1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1603: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1604: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1623: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1624: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1643: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1644: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1659: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1660: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear96 = LinearConfig::new(256, 256).with_bias(true).init(device);
        Self {
            constant1537,
            constant1538,
            constant1539,
            constant1540,
            linear93,
            linear94,
            linear95,
            constant1580,
            constant1581,
            constant1587,
            constant1588,
            constant1595,
            constant1596,
            constant1597,
            constant1598,
            constant1603,
            constant1604,
            constant1623,
            constant1624,
            constant1643,
            constant1644,
            constant1659,
            constant1660,
            linear96,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add153_out1: Tensor<3>,
        linear88_out1: Tensor<3>,
        concat16_out1: Tensor<3>,
        unsqueeze79_out1: Tensor<4>,
    ) -> Tensor<3> {
        let reducemean37_out1 = { add153_out1.clone().mean_dim(2usize) };
        let sub19_out1 = add153_out1.sub(reducemean37_out1);
        let constant1537_out1 = self.constant1537.val();
        let pow19_out1 = sub19_out1
            .clone()
            .powf((constant1537_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean38_out1 = { pow19_out1.mean_dim(2usize) };
        let constant1538_out1 = self.constant1538.val();
        let add154_out1 = reducemean38_out1
            .add((constant1538_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt19_out1 = add154_out1.sqrt();
        let div25_out1 = sub19_out1.div(sqrt19_out1);
        let constant1539_out1 = self.constant1539.val();
        let mul107_out1 = div25_out1
            .mul((constant1539_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1540_out1 = self.constant1540.val();
        let add155_out1 = mul107_out1
            .add((constant1540_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add156_out1 = add155_out1.clone().add(linear88_out1);
        let shape52_out1: [i64; 3] = {
            let axes = &add156_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice76_out1: [i64; 1] = shape52_out1[0..1].try_into().unwrap();
        let squeeze27_out1 = slice76_out1[0] as i64;
        let linear93_out1 = self.linear93.forward(concat16_out1);
        let reshape159_out1 = squeeze27_out1;
        let unsqueeze80_out1 = [reshape159_out1 as i64];
        let constant1550_out1: [i64; 1] = [13125i64];
        let constant1551_out1: [i64; 1] = [8i64];
        let constant1552_out1: [i64; 1] = [32i64];
        let concat103_out1: [i64; 4usize] = [
            &unsqueeze80_out1[..],
            &constant1550_out1[..],
            &constant1551_out1[..],
            &constant1552_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape160_out1 = linear93_out1.reshape(concat103_out1);
        let linear94_out1 = self.linear94.forward(add156_out1.clone());
        let reshape161_out1 = squeeze27_out1;
        let unsqueeze81_out1 = [reshape161_out1 as i64];
        let constant1557_out1: [i64; 1] = [300i64];
        let constant1558_out1: [i64; 1] = [8i64];
        let constant1559_out1: [i64; 1] = [3i64];
        let constant1560_out1: [i64; 1] = [4i64];
        let constant1561_out1: [i64; 1] = [2i64];
        let concat104_out1: [i64; 6usize] = [
            &unsqueeze81_out1[..],
            &constant1557_out1[..],
            &constant1558_out1[..],
            &constant1559_out1[..],
            &constant1560_out1[..],
            &constant1561_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape162_out1 = linear94_out1.reshape(concat104_out1);
        let linear95_out1 = self.linear95.forward(add156_out1);
        let reshape163_out1 = squeeze27_out1;
        let unsqueeze82_out1 = [reshape163_out1 as i64];
        let constant1566_out1: [i64; 1] = [300i64];
        let constant1567_out1: [i64; 1] = [8i64];
        let constant1568_out1: [i64; 1] = [12i64];
        let concat105_out1: [i64; 4usize] = [
            &unsqueeze82_out1[..],
            &constant1566_out1[..],
            &constant1567_out1[..],
            &constant1568_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape164_out1 = linear95_out1.reshape(concat105_out1);
        let softmax13_out1 = burn::tensor::activation::softmax(reshape164_out1, 3);
        let reshape165_out1 = squeeze27_out1;
        let unsqueeze83_out1 = [reshape165_out1 as i64];
        let constant1571_out1: [i64; 1] = [300i64];
        let constant1572_out1: [i64; 1] = [8i64];
        let constant1573_out1: [i64; 1] = [3i64];
        let constant1574_out1: [i64; 1] = [4i64];
        let concat106_out1: [i64; 5usize] = [
            &unsqueeze83_out1[..],
            &constant1571_out1[..],
            &constant1572_out1[..],
            &constant1573_out1[..],
            &constant1574_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape166_out1 = softmax13_out1.reshape(concat106_out1);
        let slice77_out1 = unsqueeze79_out1.clone().slice(s![.., .., .., 0..2]);
        let unsqueeze84_out1: Tensor<6> = slice77_out1.unsqueeze_dims::<6>(&[2, 4]);
        let constant1580_out1 = self.constant1580.val();
        let mul108_out1 = reshape162_out1
            .mul(
                (constant1580_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1581_out1 = self.constant1581.val();
        let add157_out1 = mul108_out1
            .add(
                (constant1581_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let slice78_out1 = unsqueeze79_out1.slice(s![.., .., .., 2..2147483647]);
        let unsqueeze85_out1: Tensor<6> = slice78_out1.unsqueeze_dims::<6>(&[2, 4]);
        let mul109_out1 = add157_out1.mul(unsqueeze85_out1);
        let constant1587_out1 = self.constant1587.val();
        let mul110_out1 = mul109_out1
            .mul(
                (constant1587_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1588_out1 = self.constant1588.val();
        let add158_out1 = mul110_out1
            .add(
                (constant1588_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let add159_out1 = unsqueeze84_out1.add(add158_out1);
        let shape53_out1: [i64; 4] = {
            let axes = &reshape160_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice79_out1: [i64; 1] = shape53_out1[0..1].try_into().unwrap();
        let squeeze28_out1 = slice79_out1[0] as i64;
        let split_tensors = reshape160_out1
            .split_with_sizes([10000, 2500, 625].into(), 1);
        let [split6_out1, split6_out2, split6_out3] = split_tensors.try_into().unwrap();
        let constant1595_out1 = self.constant1595.val();
        let mul111_out1 = add159_out1
            .mul(
                (constant1595_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1596_out1 = self.constant1596.val();
        let add160_out1 = mul111_out1
            .add(
                (constant1596_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1597_out1 = self.constant1597.val();
        let mul112_out1 = add160_out1
            .mul(
                (constant1597_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let constant1598_out1 = self.constant1598.val();
        let add161_out1 = mul112_out1
            .add(
                (constant1598_out1)
                    .unsqueeze_dims(&[0isize, 1isize, 2isize, 3isize, 4isize]),
            );
        let shape54_out1: [i64; 4] = {
            let axes = &split6_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice80_out1: [i64; 2] = shape54_out1[0..2].try_into().unwrap();
        let constant1602_out1: [i64; 1] = [-1i64];
        let concat107_out1: [i64; 3usize] = [&slice80_out1[..], &constant1602_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape167_out1 = split6_out1.reshape(concat107_out1);
        let transpose81_out1 = reshape167_out1.permute([0, 2, 1]);
        let cast42_out1 = squeeze28_out1 as f32;
        let constant1603_out1 = self.constant1603.val();
        let mul113_out1 = cast42_out1 * constant1603_out1;
        let constant1604_out1 = self.constant1604.val();
        let add162_out1 = mul113_out1.add(constant1604_out1);
        let cast43_out1 = add162_out1.int().cast(burn::tensor::DType::I64);
        let reshape168_out1 = cast43_out1.into_scalar::<i64>();
        let unsqueeze86_out1 = [reshape168_out1 as i64];
        let constant1607_out1: [i64; 1] = [32i64];
        let constant1608_out1: [i64; 1] = [100i64];
        let constant1609_out1: [i64; 1] = [100i64];
        let concat108_out1: [i64; 4usize] = [
            &unsqueeze86_out1[..],
            &constant1607_out1[..],
            &constant1608_out1[..],
            &constant1609_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape169_out1 = transpose81_out1.reshape(concat108_out1);
        let slice81_out1 = add161_out1.clone().slice(s![.., .., .., 0..1, .., ..]);
        let squeeze29_out1 = slice81_out1.squeeze_dims::<5>(&[3]);
        let transpose82_out1 = squeeze29_out1.permute([0, 2, 1, 3, 4]);
        let shape55_out1: [i64; 5] = {
            let axes = &transpose82_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice82_out1: [i64; 3] = shape55_out1[2..5].try_into().unwrap();
        let constant1618_out1: [i64; 1] = [-1i64];
        let concat109_out1: [i64; 4usize] = [&constant1618_out1[..], &slice82_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape170_out1 = transpose82_out1.reshape(concat109_out1);
        let gridsample16_out1 = reshape169_out1
            .grid_sample_2d(
                reshape170_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape56_out1: [i64; 4] = {
            let axes = &split6_out2.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice83_out1: [i64; 2] = shape56_out1[0..2].try_into().unwrap();
        let constant1622_out1: [i64; 1] = [-1i64];
        let concat110_out1: [i64; 3usize] = [&slice83_out1[..], &constant1622_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape171_out1 = split6_out2.reshape(concat110_out1);
        let transpose83_out1 = reshape171_out1.permute([0, 2, 1]);
        let cast44_out1 = squeeze28_out1 as f32;
        let constant1623_out1 = self.constant1623.val();
        let mul114_out1 = cast44_out1 * constant1623_out1;
        let constant1624_out1 = self.constant1624.val();
        let add163_out1 = mul114_out1.add(constant1624_out1);
        let cast45_out1 = add163_out1.int().cast(burn::tensor::DType::I64);
        let reshape172_out1 = cast45_out1.into_scalar::<i64>();
        let unsqueeze87_out1 = [reshape172_out1 as i64];
        let constant1627_out1: [i64; 1] = [32i64];
        let constant1628_out1: [i64; 1] = [50i64];
        let constant1629_out1: [i64; 1] = [50i64];
        let concat111_out1: [i64; 4usize] = [
            &unsqueeze87_out1[..],
            &constant1627_out1[..],
            &constant1628_out1[..],
            &constant1629_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape173_out1 = transpose83_out1.reshape(concat111_out1);
        let slice84_out1 = add161_out1.clone().slice(s![.., .., .., 1..2, .., ..]);
        let squeeze30_out1 = slice84_out1.squeeze_dims::<5>(&[3]);
        let transpose84_out1 = squeeze30_out1.permute([0, 2, 1, 3, 4]);
        let shape57_out1: [i64; 5] = {
            let axes = &transpose84_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice85_out1: [i64; 3] = shape57_out1[2..5].try_into().unwrap();
        let constant1638_out1: [i64; 1] = [-1i64];
        let concat112_out1: [i64; 4usize] = [&constant1638_out1[..], &slice85_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape174_out1 = transpose84_out1.reshape(concat112_out1);
        let gridsample17_out1 = reshape173_out1
            .grid_sample_2d(
                reshape174_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let shape58_out1: [i64; 4] = {
            let axes = &split6_out3.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice86_out1: [i64; 2] = shape58_out1[0..2].try_into().unwrap();
        let constant1642_out1: [i64; 1] = [-1i64];
        let concat113_out1: [i64; 3usize] = [&slice86_out1[..], &constant1642_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape175_out1 = split6_out3.reshape(concat113_out1);
        let transpose85_out1 = reshape175_out1.permute([0, 2, 1]);
        let cast46_out1 = squeeze28_out1 as f32;
        let constant1643_out1 = self.constant1643.val();
        let mul115_out1 = cast46_out1 * constant1643_out1;
        let constant1644_out1 = self.constant1644.val();
        let add164_out1 = mul115_out1.add(constant1644_out1);
        let cast47_out1 = add164_out1.int().cast(burn::tensor::DType::I64);
        let reshape176_out1 = cast47_out1.into_scalar::<i64>();
        let unsqueeze88_out1 = [reshape176_out1 as i64];
        let constant1647_out1: [i64; 1] = [32i64];
        let constant1648_out1: [i64; 1] = [25i64];
        let constant1649_out1: [i64; 1] = [25i64];
        let concat114_out1: [i64; 4usize] = [
            &unsqueeze88_out1[..],
            &constant1647_out1[..],
            &constant1648_out1[..],
            &constant1649_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape177_out1 = transpose85_out1.reshape(concat114_out1);
        let slice87_out1 = add161_out1.slice(s![.., .., .., 2..3, .., ..]);
        let squeeze31_out1 = slice87_out1.squeeze_dims::<5>(&[3]);
        let transpose86_out1 = squeeze31_out1.permute([0, 2, 1, 3, 4]);
        let shape59_out1: [i64; 5] = {
            let axes = &transpose86_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice88_out1: [i64; 3] = shape59_out1[2..5].try_into().unwrap();
        let constant1658_out1: [i64; 1] = [-1i64];
        let concat115_out1: [i64; 4usize] = [&constant1658_out1[..], &slice88_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape178_out1 = transpose86_out1.reshape(concat115_out1);
        let gridsample18_out1 = reshape177_out1
            .grid_sample_2d(
                reshape178_out1,
                burn::tensor::ops::GridSampleOptions::new(
                        burn::tensor::ops::InterpolateMode::Bilinear,
                    )
                    .with_padding_mode(burn::tensor::ops::GridSamplePaddingMode::Zeros)
                    .with_align_corners(false),
            );
        let transpose87_out1 = reshape166_out1.permute([0, 2, 1, 3, 4]);
        let cast48_out1 = squeeze28_out1 as f32;
        let constant1659_out1 = self.constant1659.val();
        let mul116_out1 = cast48_out1 * constant1659_out1;
        let constant1660_out1 = self.constant1660.val();
        let add165_out1 = mul116_out1.add(constant1660_out1);
        let cast49_out1 = add165_out1.int().cast(burn::tensor::DType::I64);
        let reshape179_out1 = cast49_out1.into_scalar::<i64>();
        let unsqueeze89_out1 = [reshape179_out1 as i64];
        let constant1663_out1: [i64; 1] = [1i64];
        let constant1664_out1: [i64; 1] = [300i64];
        let constant1665_out1: [i64; 1] = [12i64];
        let concat116_out1: [i64; 4usize] = [
            &unsqueeze89_out1[..],
            &constant1663_out1[..],
            &constant1664_out1[..],
            &constant1665_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape180_out1 = transpose87_out1.reshape(concat116_out1);
        let unsqueeze90_out1: Tensor<5> = gridsample16_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze91_out1: Tensor<5> = gridsample17_out1.unsqueeze_dims::<5>(&[3]);
        let unsqueeze92_out1: Tensor<5> = gridsample18_out1.unsqueeze_dims::<5>(&[3]);
        let concat117_out1 = burn::tensor::Tensor::cat(
            [unsqueeze90_out1, unsqueeze91_out1, unsqueeze92_out1].into(),
            3,
        );
        let shape60_out1: [i64; 5] = {
            let axes = &concat117_out1.clone().dims()[0..5];
            let mut output = [0i64; 5];
            for i in 0..5 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice89_out1: [i64; 3] = shape60_out1[0..3].try_into().unwrap();
        let constant1672_out1: [i64; 1] = [-1i64];
        let concat118_out1: [i64; 4usize] = [&slice89_out1[..], &constant1672_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape181_out1 = concat117_out1.reshape(concat118_out1);
        let mul117_out1 = reshape181_out1.mul(reshape180_out1);
        let reducesum6_out1 = {
            mul117_out1.sum_dim(3usize).squeeze_dims::<3usize>(&[3])
        };
        let reshape182_out1 = squeeze28_out1;
        let unsqueeze93_out1 = [reshape182_out1 as i64];
        let constant1676_out1: [i64; 1] = [256i64];
        let constant1677_out1: [i64; 1] = [300i64];
        let concat119_out1: [i64; 3usize] = [
            &unsqueeze93_out1[..],
            &constant1676_out1[..],
            &constant1677_out1[..],
        ]
            .concat()
            .try_into()
            .unwrap();
        let reshape183_out1 = reducesum6_out1.reshape(concat119_out1);
        let transpose88_out1 = reshape183_out1.permute([0, 2, 1]);
        let linear96_out1 = self.linear96.forward(transpose88_out1);
        let add166_out1 = add155_out1.add(linear96_out1);
        add166_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule17 {
    constant1680: burn::module::Param<Tensor<1>>,
    constant1681: burn::module::Param<Tensor<1>>,
    constant1682: burn::module::Param<Tensor<1>>,
    constant1683: burn::module::Param<Tensor<1>>,
    linear97: Linear,
    linear98: Linear,
    constant1688: burn::module::Param<Tensor<1>>,
    constant1689: burn::module::Param<Tensor<1>>,
    constant1690: burn::module::Param<Tensor<1>>,
    constant1691: burn::module::Param<Tensor<1>>,
    linear99: Linear,
    linear100: Linear,
    linear101: Linear,
    constant1702: burn::module::Param<Tensor<1>>,
    constant1703: burn::module::Param<Tensor<1>>,
    linear102: Linear,
    constant1721: burn::module::Param<Tensor<1>>,
    constant1722: burn::module::Param<Tensor<1>>,
    constant1723: burn::module::Param<Tensor<1>>,
    constant1724: burn::module::Param<Tensor<1>>,
    constant1725: burn::module::Param<Tensor<1>>,
    constant1726: burn::module::Param<Tensor<1>>,
    constant1735: burn::module::Param<Tensor<1, Int>>,
    constant1736: burn::module::Param<Tensor<1>>,
    constant1757: burn::module::Param<Tensor<1, Int>>,
    #[module(skip)]
    device: Device,
}
impl Submodule17 {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let constant1680: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1681: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1682: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1683: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear97 = LinearConfig::new(256, 1024).with_bias(true).init(device);
        let linear98 = LinearConfig::new(1024, 256).with_bias(true).init(device);
        let constant1688: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([2f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1689: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.000009999999747378752f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1690: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let constant1691: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([256], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [256].into(),
        );
        let linear99 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear100 = LinearConfig::new(256, 256).with_bias(true).init(device);
        let linear101 = LinearConfig::new(256, 4).with_bias(true).init(device);
        let constant1702: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1703: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([1f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let linear102 = LinearConfig::new(256, 20).with_bias(true).init(device);
        let constant1721: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1722: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1723: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1724: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1725: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::zeros([1], (device, burn::tensor::DType::F32)),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1726: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([0.5f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1735: burn::module::Param<Tensor<1, Int>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
                Int,
            >::from_data(
                burn::tensor::TensorData::from([20i64]),
                (device, burn::tensor::DType::I64),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1736: burn::module::Param<Tensor<1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
            >::from_data(
                burn::tensor::TensorData::from([20f64]),
                (device, burn::tensor::DType::F32),
            ),
            device.clone(),
            false,
            [1].into(),
        );
        let constant1757: burn::module::Param<Tensor<1, Int>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<
                1,
                Int,
            >::zeros([1], (device, burn::tensor::DType::I32)),
            device.clone(),
            false,
            [1].into(),
        );
        Self {
            constant1680,
            constant1681,
            constant1682,
            constant1683,
            linear97,
            linear98,
            constant1688,
            constant1689,
            constant1690,
            constant1691,
            linear99,
            linear100,
            linear101,
            constant1702,
            constant1703,
            linear102,
            constant1721,
            constant1722,
            constant1723,
            constant1724,
            constant1725,
            constant1726,
            constant1735,
            constant1736,
            constant1757,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add166_out1: Tensor<3>,
        sigmoid30_out1: Tensor<3>,
        im_shape: Tensor<2>,
        scale_factor: Tensor<2>,
    ) -> (Tensor<2>, Tensor<1, Int>) {
        let reducemean39_out1 = { add166_out1.clone().mean_dim(2usize) };
        let sub20_out1 = add166_out1.sub(reducemean39_out1);
        let constant1680_out1 = self.constant1680.val();
        let pow20_out1 = sub20_out1
            .clone()
            .powf((constant1680_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean40_out1 = { pow20_out1.mean_dim(2usize) };
        let constant1681_out1 = self.constant1681.val();
        let add167_out1 = reducemean40_out1
            .add((constant1681_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt20_out1 = add167_out1.sqrt();
        let div26_out1 = sub20_out1.div(sqrt20_out1);
        let constant1682_out1 = self.constant1682.val();
        let mul118_out1 = div26_out1
            .mul((constant1682_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1683_out1 = self.constant1683.val();
        let add168_out1 = mul118_out1
            .add((constant1683_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear97_out1 = self.linear97.forward(add168_out1.clone());
        let relu77_out1 = burn::tensor::activation::relu(linear97_out1);
        let linear98_out1 = self.linear98.forward(relu77_out1);
        let add169_out1 = add168_out1.add(linear98_out1);
        let reducemean41_out1 = { add169_out1.clone().mean_dim(2usize) };
        let sub21_out1 = add169_out1.sub(reducemean41_out1);
        let constant1688_out1 = self.constant1688.val();
        let pow21_out1 = sub21_out1
            .clone()
            .powf((constant1688_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reducemean42_out1 = { pow21_out1.mean_dim(2usize) };
        let constant1689_out1 = self.constant1689.val();
        let add170_out1 = reducemean42_out1
            .add((constant1689_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sqrt21_out1 = add170_out1.sqrt();
        let div27_out1 = sub21_out1.div(sqrt21_out1);
        let constant1690_out1 = self.constant1690.val();
        let mul119_out1 = div27_out1
            .mul((constant1690_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1691_out1 = self.constant1691.val();
        let add171_out1 = mul119_out1
            .add((constant1691_out1).unsqueeze_dims(&[0isize, 1isize]));
        let linear99_out1 = self.linear99.forward(add171_out1.clone());
        let relu78_out1 = burn::tensor::activation::relu(linear99_out1);
        let linear100_out1 = self.linear100.forward(relu78_out1);
        let relu79_out1 = burn::tensor::activation::relu(linear100_out1);
        let linear101_out1 = self.linear101.forward(relu79_out1);
        let clip16_out1 = sigmoid30_out1.clamp(0f64, 1f64);
        let clip17_out1 = clip16_out1
            .clone()
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let constant1702_out1 = self.constant1702.val();
        let mul120_out1 = clip16_out1
            .mul((constant1702_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1703_out1 = self.constant1703.val();
        let add172_out1 = mul120_out1
            .add((constant1703_out1).unsqueeze_dims(&[0isize, 1isize]));
        let clip18_out1 = add172_out1
            .clamp(
                0.000009999999747378752f64,
                340282346638528860000000000000000000000f64,
            );
        let div28_out1 = clip17_out1.div(clip18_out1);
        let log6_out1 = div28_out1.log();
        let add173_out1 = linear101_out1.add(log6_out1);
        let sigmoid31_out1 = burn::tensor::activation::sigmoid(add173_out1);
        let linear102_out1 = self.linear102.forward(add171_out1);
        let unsqueeze94_out1: Tensor<4> = sigmoid31_out1.unsqueeze_dims::<4>(&[0]);
        let concat120_out1 = burn::tensor::Tensor::cat([unsqueeze94_out1].into(), 0);
        let unsqueeze95_out1: Tensor<4> = linear102_out1.unsqueeze_dims::<4>(&[0]);
        let concat121_out1 = burn::tensor::Tensor::cat([unsqueeze95_out1].into(), 0);
        let slice90_out1 = concat120_out1.slice(s![- 1..2147483647, .., .., ..]);
        let squeeze32_out1 = slice90_out1.squeeze_dims::<3>(&[0]);
        let slice91_out1 = concat121_out1.slice(s![- 1..2147483647, .., .., ..]);
        let squeeze33_out1 = slice91_out1.squeeze_dims::<3>(&[0]);
        let split_tensors = squeeze32_out1.split_with_sizes([2, 2].into(), 2);
        let [split7_out1, split7_out2] = split_tensors.try_into().unwrap();
        let constant1721_out1 = self.constant1721.val();
        let mul121_out1 = split7_out2
            .clone()
            .mul((constant1721_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1722_out1 = self.constant1722.val();
        let add174_out1 = mul121_out1
            .add((constant1722_out1).unsqueeze_dims(&[0isize, 1isize]));
        let sub22_out1 = split7_out1.clone().sub(add174_out1);
        let constant1723_out1 = self.constant1723.val();
        let mul122_out1 = split7_out2
            .mul((constant1723_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant1724_out1 = self.constant1724.val();
        let add175_out1 = mul122_out1
            .add((constant1724_out1).unsqueeze_dims(&[0isize, 1isize]));
        let add176_out1 = split7_out1.add(add175_out1);
        let concat122_out1 = burn::tensor::Tensor::cat(
            [sub22_out1, add176_out1].into(),
            2,
        );
        let div29_out1 = im_shape.div(scale_factor);
        let constant1725_out1 = self.constant1725.val();
        let mul123_out1 = div29_out1.mul((constant1725_out1).unsqueeze_dims(&[0isize]));
        let constant1726_out1 = self.constant1726.val();
        let add177_out1 = mul123_out1.add((constant1726_out1).unsqueeze_dims(&[0isize]));
        let floor1_out1 = add177_out1.floor();
        let split_tensors = floor1_out1.split_with_sizes([1, 1].into(), 1);
        let [split8_out1, split8_out2] = split_tensors.try_into().unwrap();
        let concat123_out1 = burn::tensor::Tensor::cat(
            [split8_out2, split8_out1].into(),
            1,
        );
        let tile2_out1 = concat123_out1.repeat(&[1, 2]);
        let unsqueeze96_out1: Tensor<3> = tile2_out1.unsqueeze_dims::<3>(&[1]);
        let mul124_out1 = concat122_out1.mul(unsqueeze96_out1);
        let sigmoid32_out1 = burn::tensor::activation::sigmoid(squeeze33_out1);
        let shape61_out1: [i64; 3] = {
            let axes = &sigmoid32_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice92_out1: [i64; 1] = shape61_out1[0..1].try_into().unwrap();
        let constant1733_out1: [i64; 1] = [-1i64];
        let concat124_out1: [i64; 2usize] = [&slice92_out1[..], &constant1733_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape184_out1 = sigmoid32_out1.reshape(concat124_out1);
        let (topk2_out1, topk2_out2) = {
            let (values, indices) = reshape184_out1.topk_with_indices(300, 1);
            (values, indices.cast(burn::tensor::DType::I64))
        };
        let constant1735_out1 = self.constant1735.val();
        let mod1_out1 = topk2_out2
            .clone()
            .remainder((constant1735_out1).unsqueeze_dims(&[0isize]));
        let cast50_out1 = topk2_out2.float().cast(burn::tensor::DType::F32);
        let constant1736_out1 = self.constant1736.val();
        let div30_out1 = cast50_out1.div((constant1736_out1).unsqueeze_dims(&[0isize]));
        let floor2_out1 = div30_out1.floor();
        let cast51_out1 = floor2_out1.int().cast(burn::tensor::DType::I64);
        let shape62_out1: [i64; 2] = {
            let axes = &topk2_out1.clone().dims()[0..2];
            let mut output = [0i64; 2];
            for i in 0..2 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice93_out1: [i64; 1] = shape62_out1[0..1].try_into().unwrap();
        let squeeze34_out1 = slice93_out1[0] as i64;
        let range2_out1 = {
            let start = 0i64;
            let delta = 1i64;
            assert!(delta != 0);
            let n = (((squeeze34_out1 as i128 - start as i128) as f64) / delta as f64)
                .ceil()
                .max(0.0) as i64;
            Tensor::arange(0..n, &self.device)
                .cast(burn::tensor::DType::I64)
                .mul_scalar(delta)
                .add_scalar(start)
        };
        let unsqueeze97_out1: Tensor<2, Int> = range2_out1.unsqueeze_dims::<2>(&[-1]);
        let tile3_out1 = unsqueeze97_out1.repeat(&[1, 300]);
        let unsqueeze98_out1: Tensor<3, Int> = tile3_out1.unsqueeze_dims::<3>(&[2]);
        let unsqueeze99_out1: Tensor<3, Int> = cast51_out1.unsqueeze_dims::<3>(&[2]);
        let concat125_out1 = burn::tensor::Tensor::cat(
            [unsqueeze98_out1, unsqueeze99_out1].into(),
            2,
        );
        let gathernd3_out1 = {
            let data_dims = mul124_out1.dims();
            let indices_i64 = concat125_out1.cast(burn::tensor::DType::I64);
            let idx_dims = indices_i64.dims();
            let k = idx_dims[3 - 1];
            let mut dim_sizes: alloc::vec::Vec<i64> = alloc::vec::Vec::with_capacity(k);
            for i in 0..k {
                dim_sizes.push(data_dims[0 + i] as i64);
            }
            let mut bcast_shape = [1usize; 3];
            bcast_shape[3 - 1] = k;
            let dims_tensor = Tensor::<
                1,
                Int,
            >::from_data(
                    burn::tensor::TensorData::from(dim_sizes.as_slice()),
                    (&self.device, burn::tensor::DType::I64),
                )
                .reshape(bcast_shape);
            let negative = indices_i64.clone().lower_elem(0i64);
            let corrected = indices_i64.clone() + dims_tensor;
            let indices_norm = indices_i64.mask_where(negative, corrected);
            let aug_indices = indices_norm;
            mul124_out1.gather_nd(aug_indices)
        };
        let unsqueeze100_out1: Tensor<3, Int> = mod1_out1.unsqueeze_dims::<3>(&[-1]);
        let cast52_out1 = unsqueeze100_out1.float().cast(burn::tensor::DType::F32);
        let unsqueeze101_out1: Tensor<3> = topk2_out1.unsqueeze_dims::<3>(&[-1]);
        let concat126_out1 = burn::tensor::Tensor::cat(
            [cast52_out1, unsqueeze101_out1, gathernd3_out1].into(),
            2,
        );
        let shape63_out1: [i64; 3] = {
            let axes = &concat126_out1.clone().dims()[0..3];
            let mut output = [0i64; 3];
            for i in 0..3 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice94_out1: [i64; 1] = shape63_out1[0..1].try_into().unwrap();
        let squeeze35_out1 = slice94_out1[0] as i64;
        let reshape185_out1 = squeeze35_out1;
        let unsqueeze102_out1 = [reshape185_out1 as i64];
        let concat127_out1: [i64; 1usize] = [&unsqueeze102_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let constant1757_out1 = self.constant1757.val();
        let tile4_out1 = {
            let repeats: alloc::vec::Vec<usize> = concat127_out1
                .iter()
                .map(|&v| v as usize)
                .collect();
            constant1757_out1.repeat(&repeats)
        };
        let reshape186_out1 = concat126_out1.reshape([-1, 6]);
        (reshape186_out1, tile4_out1)
    }
}

#[derive(Module, Debug)]
pub struct Model {
    submodule1: Submodule1,
    submodule2: Submodule2,
    submodule3: Submodule3,
    submodule4: Submodule4,
    submodule5: Submodule5,
    submodule6: Submodule6,
    submodule7: Submodule7,
    submodule8: Submodule8,
    submodule9: Submodule9,
    submodule10: Submodule10,
    submodule11: Submodule11,
    submodule12: Submodule12,
    submodule13: Submodule13,
    submodule14: Submodule14,
    submodule15: Submodule15,
    submodule16: Submodule16,
    submodule17: Submodule17,
    #[module(skip)]
    device: Device,
}


extern crate std;

impl Default for Model {
    fn default() -> Self {
        Self::from_file("models\\layout.bpk", &Default::default())
    }
}

impl Model {
    /// Load model weights from a burnpack file.
    pub fn from_file<P: AsRef<std::path::Path>>(file: P, device: &Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_file(&file);
        model
            .load_from(&mut store)
            .unwrap_or_else(|e| {
                panic!("Failed to load burnpack file {}: {e}", file.as_ref().display())
            });
        model
    }

    /// Load model weights from in-memory bytes.
    ///
    /// The bytes must be the contents of a `.bpk` file.
    pub fn from_bytes(bytes: Bytes, device: &Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_bytes(Some(bytes));
        model
            .load_from(&mut store)
            .unwrap_or_else(|e| panic!("Failed to load burnpack bytes: {e}"));
        model
    }
}

impl Model {
    #[allow(unused_variables)]
    pub fn new(device: &Device) -> Self {
        let submodule1 = Submodule1::new(device);
        let submodule2 = Submodule2::new(device);
        let submodule3 = Submodule3::new(device);
        let submodule4 = Submodule4::new(device);
        let submodule5 = Submodule5::new(device);
        let submodule6 = Submodule6::new(device);
        let submodule7 = Submodule7::new(device);
        let submodule8 = Submodule8::new(device);
        let submodule9 = Submodule9::new(device);
        let submodule10 = Submodule10::new(device);
        let submodule11 = Submodule11::new(device);
        let submodule12 = Submodule12::new(device);
        let submodule13 = Submodule13::new(device);
        let submodule14 = Submodule14::new(device);
        let submodule15 = Submodule15::new(device);
        let submodule16 = Submodule16::new(device);
        let submodule17 = Submodule17::new(device);
        Self {
            submodule1,
            submodule2,
            submodule3,
            submodule4,
            submodule5,
            submodule6,
            submodule7,
            submodule8,
            submodule9,
            submodule10,
            submodule11,
            submodule12,
            submodule13,
            submodule14,
            submodule15,
            submodule16,
            submodule17,
            device: device.clone(),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        im_shape: Tensor<2>,
        image: Tensor<4>,
        scale_factor: Tensor<2>,
    ) -> (Tensor<2>, Tensor<1, Int>) {
        let conv2d21_out1 = self.submodule1.forward(image);
        let (add1_out1, relu21_out1) = self.submodule2.forward(conv2d21_out1);
        let (concat7_out1, add2_out1) = self.submodule3.forward(add1_out1);
        let concat16_out1 = self
            .submodule4
            .forward(concat7_out1, relu21_out1, add2_out1);
        let (add38_out1, linear13_out1, unsqueeze4_out1, sigmoid25_out1) = self
            .submodule5
            .forward(concat16_out1.clone());
        let add51_out1 = self
            .submodule6
            .forward(add38_out1, linear13_out1, concat16_out1.clone(), unsqueeze4_out1);
        let (add61_out1, linear28_out1, unsqueeze19_out1, sigmoid26_out1) = self
            .submodule7
            .forward(add51_out1, sigmoid25_out1);
        let add74_out1 = self
            .submodule8
            .forward(add61_out1, linear28_out1, concat16_out1.clone(), unsqueeze19_out1);
        let (add84_out1, linear43_out1, unsqueeze34_out1, sigmoid27_out1) = self
            .submodule9
            .forward(add74_out1, sigmoid26_out1);
        let add97_out1 = self
            .submodule10
            .forward(add84_out1, linear43_out1, concat16_out1.clone(), unsqueeze34_out1);
        let (add107_out1, linear58_out1, unsqueeze49_out1, sigmoid28_out1) = self
            .submodule11
            .forward(add97_out1, sigmoid27_out1);
        let add120_out1 = self
            .submodule12
            .forward(
                add107_out1,
                linear58_out1,
                concat16_out1.clone(),
                unsqueeze49_out1,
            );
        let (add130_out1, linear73_out1, unsqueeze64_out1, sigmoid29_out1) = self
            .submodule13
            .forward(add120_out1, sigmoid28_out1);
        let add143_out1 = self
            .submodule14
            .forward(
                add130_out1,
                linear73_out1,
                concat16_out1.clone(),
                unsqueeze64_out1,
            );
        let (add153_out1, linear88_out1, unsqueeze79_out1, sigmoid30_out1) = self
            .submodule15
            .forward(add143_out1, sigmoid29_out1);
        let add166_out1 = self
            .submodule16
            .forward(add153_out1, linear88_out1, concat16_out1, unsqueeze79_out1);
        let (reshape186_out1, tile4_out1) = self
            .submodule17
            .forward(add166_out1, sigmoid30_out1, im_shape, scale_factor);
        (reshape186_out1, tile4_out1)
    }
}
