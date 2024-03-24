use std::env;

fn main() {
    println!("Mandelbrot!");
    let env_args = env::args();
    mandelbrot::run(env_args);
}
