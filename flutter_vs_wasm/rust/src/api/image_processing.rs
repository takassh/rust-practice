use std::io::Cursor;

use image::{imageops::filter3x3, GenericImageView, ImageFormat, Luma};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn pic2edge(buffer: Vec<u8>) -> Vec<u8> {
    let image = image::load_from_memory_with_format(&buffer, ImageFormat::Jpeg);
    let Ok(image) = image else {
        panic!("Failed to load image from buffer");
    };

    let width = image.width();
    let height = image.height();

    let gray_image = image::ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = image.get_pixel(x, y);
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        Luma([(r as f32 * 299. / 1000. + g as f32 * 587. / 1000. + b as f32 * 114. / 1000.) as u8])
    });

    let gaussian_image = filter3x3(
        &gray_image,
        &[
            0.07511361, 0.1238414, 0.07511361, 0.1238414, 0.20417996, 0.1238414, 0.07511361,
            0.1238414, 0.07511361,
        ],
    );

    let gaussian_laplacian_image =
        filter3x3(&gaussian_image, &[1., 1., 1., 1., -8., 1., 1., 1., 1.]);

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    gaussian_laplacian_image
        .write_to(&mut writer, ImageFormat::Png)
        .unwrap();
    buffer
}
