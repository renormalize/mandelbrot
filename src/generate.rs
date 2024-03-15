use std::cmp::max;

use super::config::Config;
use image::{Rgb, RgbImage};

pub fn create_image(config: &Config, file_name: String) -> Result<(), &'static str> {
    let mut imgbuf = RgbImage::new(config.horizontal_resolution, config.vertical_resolution);
    imgbuf = fill_image(imgbuf);
    imgbuf
        .save(file_name)
        .map_err(|_| "Not able to save the image")
}

fn fill_image(mut imgbuf: RgbImage) -> RgbImage {
    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        *pix = Rgb([x as u8, y as u8, max(x, y) as u8]);
    }
    imgbuf
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    #[allow(clippy::unit_cmp)]
    fn test_create_buffer() {
        let config = super::Config::build(&[
            "mandelbrot".to_string(),
            "100".to_string(),
            "100".to_string(),
        ])
        .expect("Valid arguments should not fail");
        assert_eq!(
            super::create_image(&config, "test_image.jpg".to_string()).unwrap(),
            ()
        );
        let _ = fs::remove_file("test_image.jpg");
    }
}
