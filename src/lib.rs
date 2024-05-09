use config::Config;
use std::time::Instant;

mod config;
mod constants;
mod generate;

pub fn run(mut arguments: impl Iterator<Item = String>) {
    // program name
    let _ = match arguments.next() {
        Some(program_name) => program_name,
        None => {
            eprintln!("Not enough arguments passed to the command");
            print_usage();
            return;
        }
    };
    // input parameters
    let config = match Config::build(arguments) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Running with the specified configuration failed because of {e}");
            return;
        }
    };
    let now = Instant::now();
    let _ = generate::generate_image(&config, "mandelbrot.bmp");
    let elapsed = now.elapsed();
    println!("Image generated successfully in {}", elapsed.as_secs_f64());
}

fn print_usage() {
    println!("Usage:\tmandelbrot horizontal_resolution vertical_resolution");
}
