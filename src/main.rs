use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        command::get_command(&args[0])();
    } else {
        command::help();
    }
}

pub mod command;