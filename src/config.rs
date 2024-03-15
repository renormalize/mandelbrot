#[derive(Debug, PartialEq)]
pub struct Config {
    horizontal_resolution: i32,
    vertical_resolution: i32,
}

impl Config {
    pub fn build(env_args: &[String]) -> Result<Config, &'static str> {
        // command_name horizontal_resolution vertical_resolution
        if env_args.len() < 3 {
            super::print_usage();
            return Err("Not enough arguments passed to the command");
        }
        let horizontal_resolution: i32 = match env_args[1].trim().parse() {
            Ok(horizontal_resolution) => horizontal_resolution,
            Err(_) => {
                super::print_usage();
                return Err("Error while converting the input to an integer");
            }
        };
        let vertical_resolution: i32 = match env_args[2].trim().parse() {
            Ok(horizontal_resolution) => horizontal_resolution,
            Err(_) => {
                super::print_usage();
                return Err("Error while converting the input to an integer");
            }
        };
        Ok(Config {
            horizontal_resolution,
            vertical_resolution,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn build_test() {
        // not enough arguments
        let a = [String::new(), String::new()];
        let config = super::Config::build(&a);
        assert_eq!(Err("Not enough arguments passed to the command"), config);
        // not numbers as arguments
        let a = [
            String::from("mandelbrot"),
            String::from("1920f"),
            String::from("1080"),
        ];
        let config = super::Config::build(&a);
        assert_eq!(
            Err("Error while converting the input to an integer"),
            config
        );
        let a = [
            String::from("mandelbrot"),
            String::from("1920"),
            String::from("1080f"),
        ];
        let config = super::Config::build(&a);
        assert_eq!(
            Err("Error while converting the input to an integer"),
            config
        );
        let a = [
            String::from("mandelbrot"),
            String::from("1920"),
            String::from("1080"),
        ];
        let config = super::Config::build(&a);
        assert_eq!(
            Ok(super::Config {
                horizontal_resolution: 1920,
                vertical_resolution: 1080
            }),
            config
        );
    }
}
