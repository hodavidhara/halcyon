use std::fs;
use std::fs::File;

pub fn init() {
    // create directory and default config
    if let Err(e) = fs::create_dir(".hlc") {
        panic!("Error initiating new halcyon repository: {}", e)
    }

    match File::create("./.hlc/config.toml") {
        Err(e) => panic!("Error initiating new halcyon repository: {}", e),
        Ok(_) => println!("Successfully instantiated new halcyon repo!")
    }
}

pub fn help() {
    // TODO
    println!("Help is on the way!");
}

pub fn get_command(name: &str) -> Box<Fn()> {
    Box::new(match name {
        "init" => init,
        _ => help,
    })
}