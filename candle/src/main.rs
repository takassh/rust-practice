use candle_core::{Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::new_metal(0).unwrap();

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;
    println!("{a}");
    println!("{b}");

    let c = a.matmul(&b)?;
    println!("{c}");
    Ok(())
}