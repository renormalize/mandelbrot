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
        let color = pixel_color(c);
        let h = Hsl::new(color as f64 + 150.0, 100.0, 50.0, None);
        let xx = colorsys::Rgb::from(h);
        *pix = Rgb([xx.red() as u8, xx.green() as u8, xx.blue() as u8]);
    }
}

/// number_iter returns the number of iterations it takes for convergence
fn pixel_color(c: Complex64) -> u8 {
    let mut count_iterations: u32 = 0;
    let mut z = Complex::new(0.0, 0.0);

    while count_iterations < MAX_ITER && z.norm_sqr() <= BAILOUT_RADIUS_SQUARE as f64 {
        z = z * z + c;
        count_iterations += 1;
    }
    smooth_coloring(count_iterations, z)
}

fn smooth_coloring(count_iterations: u32, z: num_complex::Complex<f64>) -> u8 {
    let log2_mod_zn = f64::log2(z.norm_sqr()) / 2.0;
    let log2_log2_mod_zn = f64::log2(log2_mod_zn);
    let value_with_potential = if count_iterations < MAX_ITER {
        count_iterations as f64 + 1.0 - log2_log2_mod_zn
    } else {
        count_iterations as f64
    };
    (value_with_potential as u64 % u8::MAX as u64) as u8
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
