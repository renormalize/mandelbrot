#[derive(Debug, PartialEq)]
pub struct Config {
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
}

impl Config {
    pub fn build(mut arguments: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // command_name horizontal_resolution vertical_resolution
        let horizontal_resolution: u32 = match arguments.next() {
            Some(horizontal_resolution) => match horizontal_resolution.trim().parse() {
                Ok(horizontal_resolution) => horizontal_resolution,
                Err(_) => {
                    super::print_usage();
                    return Err("Error while converting the input to an integer");
                }
            },
            None => {
                super::print_usage();
                return Err("Not enough arguments passed to the command");
            }
        };
        let vertical_resolution: u32 = match arguments.next() {
            Some(vertical_resolution) => match vertical_resolution.trim().parse() {
                Ok(vertical_resolution) => vertical_resolution,
                Err(_) => {
                    super::print_usage();
                    return Err("Error while converting the input to an integer");
                }
            },
            None => {
                super::print_usage();
                return Err("Not enough arguments passed to the command");
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
        let a = [String::new()].into_iter();
        let config = super::Config::build(a);
        assert_eq!(
            Err("Error while converting the input to an integer"),
            config
        );
        // not numbers as arguments
        let a = [String::from("1920f"), String::from("1080")].into_iter();
        let config = super::Config::build(a);
        assert_eq!(
            Err("Error while converting the input to an integer"),
            config
        );
        let a = [String::from("1920"), String::from("1080f")].into_iter();
        let config = super::Config::build(a);
        assert_eq!(
            Err("Error while converting the input to an integer"),
            config
        );
        let a = [String::from("1920"), String::from("1080")].into_iter();
        let config = super::Config::build(a);
        assert_eq!(
            Ok(super::Config {
                horizontal_resolution: 1920,
                vertical_resolution: 1080
            }),
            config
        );
    }
}
