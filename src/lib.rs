use config::Config;
use std::env;

mod config;
mod generate;

pub fn run() {
    let env_args: Vec<String> = env::args().collect();
    let config = match Config::build(&env_args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Running with the specified configuration failed because of {e}");
            return;
        }
    };
    let _ = generate::create_image(&config, "mandelbrot.jpg".to_string());
}

fn print_usage() {
    println!("Usage:\tmandelbrot horizontal_resolution vertical_resolution");
}
