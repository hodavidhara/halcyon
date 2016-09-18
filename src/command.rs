use std::fs;
use std::fs::File;

pub trait Command {
    fn run(&self) -> ();
}

#[derive(Debug)]
pub struct Init;

impl Command for Init {
    fn run(&self) -> () {
        // create directory and default config
        if let Err(e) = fs::create_dir(".hlc") {
            panic!("Error initiating new halcyon repository: {}", e)
        }

        // TODO: figure out the name of the directory the command was executed in, and add that
        // as the name to the toml file. Also allow for a file name to be passed in.

        match File::create("./.hlc/config.toml") {
            Err(e) => panic!("Error initiating new halcyon repository: {}", e),
            Ok(_) => println!("Successfully instantiated new halcyon repo!")
        }
    }
}

#[derive(Debug)]
pub struct Help;

impl Command for Help {
    fn run(&self) {
        println!("Help is on the way!");
    }
}

#[derive(Debug)]
pub struct Version;

impl Command for Version {
    fn run(&self) {
        println!("halcyon version {}", env!("CARGO_PKG_VERSION"));
    }
}

pub fn get_command(name: &str) -> Box<Command> {
    match name {
        "init" => Box::new(Init),
        "version" => Box::new(Version),
        _ => Box::new(Help),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command() {
        // TODO: Figure out how to check the struct is the one we want.
        // assert_eq!(Help, get_command("fake"));
    }
}