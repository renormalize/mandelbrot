use super::config::Config;
use image::{Rgb, RgbImage};
use num_complex::{Complex, Complex64};

/// REAL_START and REAL_END constants are the endpoints of the interval in which the image is generated
/// The image is generated between x ∈ [REAL_START, REAL_END)
const REAL_END: f64 = 2.0;
const REAL_START: f64 = -2.0;

/// IMAGINARY_START and IMAGINARY_END constants are the endpoints of the interval in which the image is generated
/// The image is generated between y ∈ [IMAGINARY_START, IMAGINARY_END)
const IMAGINARY_END: f64 = 2.0;
const IMAGINARY_START: f64 = -2.0;

const REAL_WIDTH: f64 = REAL_END - REAL_START;
const IMAGINARY_WIDTH: f64 = IMAGINARY_END - IMAGINARY_START;

const MAX_ITER: u32 = 255;

/// generate_image generates the image from the configuration provided into the file_name
pub fn generate_image(config: &Config, file_name: &str) -> Result<(), &'static str> {
    let mut imgbuf = RgbImage::new(config.horizontal_resolution, config.vertical_resolution);
    imgbuf = fill_image(imgbuf);
    imgbuf
        .save(file_name)
        .map_err(|_| "Not able to save the image")
}

/// fill_image fills each pixel with its corresponding value
fn fill_image(mut imgbuf: RgbImage) -> RgbImage {
    let (width, height) = imgbuf.dimensions();
    // the width per pixel must be the same as the height per pixel
    let scale_x = REAL_WIDTH / width as f64;
    let scale_y = IMAGINARY_WIDTH / height as f64;

    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        let c_real = scale_x * x as f64 + REAL_START;
        let c_imaginary = -(scale_y * y as f64 + IMAGINARY_START);
        let c = num_complex::Complex::new(c_real, c_imaginary);
        let color = number_iter(c);
        *pix = Rgb([color, 0, 0]);
    }

    imgbuf
}

/// number_iter returns the number of iterations it takes for convergence
fn number_iter(c: Complex64) -> u8 {
    let mut count_iterations: u32 = 0;
    let mut z = Complex::new(0.0, 0.0);

    while count_iterations < MAX_ITER && z.norm() <= 2.0 {
        z = z * z + c;
        count_iterations += 1;
    }

    count_iterations as u8
}
