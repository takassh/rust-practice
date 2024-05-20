use image::{imageops::filter3x3, ImageBuffer, Luma};

fn pic2gray(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let image = image::open("logo.jpg").unwrap();
    // image.grayscale().save("grayscale.jpg").unwrap();

    let width = image.width();
    let height = image.height();

    let image = image.as_rgb8();
    let Some(image) = image else {
        panic!("Failed to convert image to RGB8");
    };

    let mut l: Vec<f32> = vec![];
    for (x, y, pixel) in image.enumerate_pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        l.push(r as f32 * 299. / 1000. + g as f32 * 587. / 1000. + b as f32 * 114. / 1000.)
    }

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Luma([l[(x + y * height) as usize] as u8]);
        }
    }

    imgbuf
}
fn main() {
    let gray_image = pic2gray("logo.jpg");

    let gaussian_image = filter3x3(
        &gray_image,
        &[
            0.07511361, 0.1238414, 0.07511361, 0.1238414, 0.20417996, 0.1238414, 0.07511361,
            0.1238414, 0.07511361,
        ],
    );

    filter3x3(&gaussian_image, &[1., 1., 1., 1., -8., 1., 1., 1., 1.])
        .save("gaussian_laplacian.jpg")
        .unwrap();
}
