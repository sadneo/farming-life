use std::error::Error;
use std::fs;

pub struct Config {
    pub save: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Ok(Config { save: String::from("default") });
        }

        let save = args[1].clone();
        Ok(Config { save })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Hello world!");
    // call method to load the save, initialize the gui, and then run the game
    Ok(())
}

