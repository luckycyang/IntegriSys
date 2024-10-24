use burn::backend::{Autodiff, Wgpu};
use burn::{prelude::*, tensor::Tensor};
use image::{Rgb32FImage, RgbImage};
use model::test::Model;

mod model;

struct Rgb32FImageWrap(Rgb32FImage);
struct TensorD4Warp<B: Backend>(Tensor<B, 4, Float>, u32, u32);

impl Rgb32FImageWrap {
    fn new(data: Rgb32FImage) -> Self {
        Self(data)
    }

    fn to_tensor<B: Backend>(self, device: &B::Device) -> Tensor<B, 4> {
        let (width, height) = self.0.dimensions();
        Tensor::<B, 3>::from_data(
            TensorData::new(self.0.into_raw(), [width as usize, height as usize, 3]),
            device,
        )
        .permute([2, 0, 1])
        .unsqueeze::<4>()
    }

    fn from_path(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let image = image::ImageReader::open(path)?.decode()?;
        Ok(Rgb32FImageWrap(image.into_rgb32f()))
    }

    fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }

    fn save(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<u8> = self
            .0
            .clone()
            .into_raw()
            .iter()
            .map(|&x| (x * 255.0) as u8)
            .collect();
        RgbImage::from_raw(self.0.width(), self.0.height(), data)
            .ok_or("Faild to same image: {path}")?
            .save(path)?;

        Ok(())
    }
}

impl<B: Backend> TensorD4Warp<B> {
    fn new(data: Tensor<B, 4, Float>) -> Self {
        let [_, _, width, height] = data.shape().dims;
        Self(data, width as u32, height as u32)
    }
}

impl<B: Backend> Into<Rgb32FImageWrap> for TensorD4Warp<B> {
    fn into(self) -> Rgb32FImageWrap {
        let data = self
            .0
            .squeeze::<3>(0)
            .permute([1, 2, 0])
            .to_data()
            .to_vec::<f32>()
            .unwrap();
        Rgb32FImageWrap(Rgb32FImage::from_raw(self.1, self.2, data).unwrap())
    }
}

fn main() {
    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;
    let mymodel: model::test::Model<MyBackend> = Model::default();
    let device = burn::backend::wgpu::WgpuDevice::default();
    let input = std::env::args().nth(1).expect("no input given");
    let output_path = std::env::args().nth(2).expect("no  output given");

    let image = Rgb32FImageWrap::from_path(&std::path::Path::new(&input)).unwrap();
    let (width, height) = image.dimensions();
    println!("input image: {:?}", (width, height));
    let input_tensor = image.to_tensor(&device);
    let (out, _) = mymodel.forward(input_tensor);
    let out_tensor = TensorD4Warp::new(out);
    let out_img: Rgb32FImageWrap = out_tensor.into();
    out_img.save(std::path::Path::new(&output_path)).unwrap();
}
