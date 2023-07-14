use std::{fs, process::exit};

pub mod dimensions;
use dimensions::Dimensions;

pub mod robot_init;
use robot_init::RobotInit;

pub mod configuration;
use configuration::Configuration;

pub fn get_dimensions() -> Dimensions {
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

    return dimensions;
}

pub fn get_robots() -> RobotInit {
    let filename = "robots.json";
            
    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            eprint!("Could not read file {}", filename);
            exit(1);
        }
    };

    let robots: RobotInit = match serde_json::from_str(contents.as_str()) {
        Ok(robots) => robots,
        Err(_) => {
            eprint!("Unable to load data from {}", filename);
            exit(1);
        }
    };

    return robots;
}

pub fn get_configuration() -> Configuration {
    let filename = "Config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read file {}", filename);
            exit(1);
        }
    };

    let configuration: Configuration = match toml::from_str(contents.as_str()) {
        Ok(configuration) => configuration,
        Err(_) => {
            eprintln!("Unable to load data from {}", filename);
            exit(1);
        }
    };

    return configuration;
}