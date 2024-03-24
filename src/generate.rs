use super::config::Config;
use super::constants::{MAX_ITER, REAL_END, REAL_START};
use image::{Rgb, RgbImage};
use num_complex::{Complex, Complex64};

const REAL_WIDTH: f64 = REAL_END - REAL_START;

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
    // let scale_y = IMAGINARY_WIDTH / height as f64;

    let imaginary_start = -scale_x * (height as f64 / 2.0);

    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        let c_real = scale_x * x as f64 + REAL_START;
        let c_imaginary = -(scale_x * y as f64 + imaginary_start);
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
    pixel_value(count_iterations)
}

// linear scaling for the pixel values
#[cfg(feature = "linear")]
fn pixel_value(count_iterations: u32) -> u8 {
    ((count_iterations as f64 / MAX_ITER as f64) * u8::MAX as f64) as u8
}

// square root scaling for the pixel values
#[cfg(feature = "root")]
fn pixel_value(count_iterations: u32) -> u8 {
    (f64::powf(count_iterations as f64 / MAX_ITER as f64, 0.5) * u8::MAX as f64) as u8
}

// trucating to get the pixel value
#[cfg(not(any(feature = "linear", feature = "root")))]
fn pixel_value(count_iterations: u32) -> u8 {
    if count_iterations > u8::MAX as u32 {
        u8::MAX
    } else {
        count_iterations as u8
    }
}
