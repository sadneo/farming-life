use std::env;
use std::process;
use learning_rust::{self, Config};

#[cfg(test)]
pub mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let result = learning_rust::run(config);
    if let Err(e) = result {
        println!("Application error: {e}");
        process::exit(1);
    }
}

