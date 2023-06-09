extern crate ncomm;
extern crate tiny_cup_ref;
extern crate serde;
extern crate serde_json;

pub mod field;
pub mod dimensions;
pub mod robots;

use std::fs;
use std::process::exit;

use dimensions::Dimensions;
use robots::Robots;

fn main() {
    let filename = "robots.json";

    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            eprint!("Could not read file {}", filename);
            exit(1);
        }
    };

    let robots: Robots = match serde_json::from_str(contents.as_str()) {
        Ok(robots) => robots,
        Err(_) => {
            eprint!("Unable to load data from {}", filename);
            exit(1);
        }
    };

    let filename = "dimensions.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read file {}", filename);
            exit(1);
        }
    };

    let dimensions: Dimensions = match toml::from_str(contents.as_str()) {
        Ok(dimensions) => dimensions,
        Err(_) => {
            eprintln!("Unable to load data from {}", filename);
            exit(1);
        }
    };
}
