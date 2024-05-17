use super::config::Config;
use super::constants::{BAILOUT_RADIUS_SQUARE, MAX_ITER, REAL_START, REAL_WIDTH};
use colorsys::Hsl;
use image::{Rgb, RgbImage};
use num_complex::{Complex, Complex64};

// type Scaling = fn(u32) -> u8;

/// generate_image generates the image from the configuration provided into the file_name
pub fn generate_image(config: &Config, file_name: &str) -> Result<(), &'static str> {
    let mut imgbuf = RgbImage::new(config.horizontal_resolution, config.vertical_resolution);
    fill_image(&mut imgbuf);
    imgbuf
        .save(file_name)
        .map_err(|_| "Not able to save the image")
}

/// fill_image fills each pixel with its corresponding value
fn fill_image(imgbuf: &mut RgbImage) {
    let (width, height) = imgbuf.dimensions();

    // the width per pixel must be the same as the height per pixel
    // TODO: @renormalize currently the scaling is only generated from the width, this could be improved
    let scale = REAL_WIDTH / width as f64;

    let imaginary_start = -scale * (height as f64 / 2.0);

    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        let c_real = scale * x as f64 + REAL_START;
        let c_imaginary = -(scale * y as f64 + imaginary_start);
        let c = num_complex::Complex::new(c_real, c_imaginary);
        *pix = pixel_color(c);
    }
}

// pixel_color returns the color that has been generated for the pixel
fn pixel_color(c: Complex64) -> Rgb<u8> {
    let mut count_iterations: u32 = 0;
    let mut z = Complex::new(0.0, 0.0);

    while count_iterations < MAX_ITER && z.norm_sqr() <= BAILOUT_RADIUS_SQUARE as f64 {
        z = z * z + c;
        count_iterations += 1;
    }

    let renormalized_iterations = renormalize_iterations(count_iterations, z);

    let colorsys_color =
        colorsys::Rgb::from(Hsl::new(renormalized_iterations + 150.0, 100.0, 50.0, None));
    Rgb([
        colorsys_color.red() as u8,
        colorsys_color.green() as u8,
        colorsys_color.blue() as u8,
    ])
}

// compensated_iterations returns the count iterations after using the potential function
// to compensate for the banding, as a f64.
fn renormalize_iterations(count_iterations: u32, z: num_complex::Complex<f64>) -> f64 {
    let ln_mod_zn = f64::ln(z.norm_sqr()) / 2.0;
    let ln_ln_mod_zn = f64::ln(ln_mod_zn);
    let log_factor = ln_ln_mod_zn / f64::ln(2.0);

    if count_iterations < MAX_ITER {
        count_iterations as f64 + 1.0 - log_factor
    } else {
        count_iterations as f64
    }
}

// fn pixel_value(func: Scaling, count_iterations: u32) -> u8 {
//     func(count_iterations)
// }

// // linear scaling for the pixel values
// fn _linear_scaling(count_iterations: u32) -> u8 {
//     ((count_iterations as f64 / MAX_ITER as f64) * u8::MAX as f64) as u8
// }

// // square root scaling for the pixel values
// fn root_scaling(count_iterations: u32) -> u8 {
//     (f64::powf(count_iterations as f64 / MAX_ITER as f64, 0.5) * u8::MAX as f64) as u8
// }

// // trucating to get the pixel value
// fn _truncating(count_iterations: u32) -> u8 {
//     if count_iterations > u8::MAX as u32 {
//         u8::MAX
//     } else {
//         count_iterations as u8
//     }
// }
