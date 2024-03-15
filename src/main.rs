use mandelbrot::config::Config;
use std::env;

fn main() {
    println!("Mandelbrot!");
    let env_args: Vec<String> = env::args().collect();
    let config = Config::build(&env_args);
    println!("{:#?}", config);
}
