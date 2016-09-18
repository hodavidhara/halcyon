use std::env;
use command::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        command::get_command(&args[0]).run();
    } else {
        command::Help.run();
    }
}

pub mod command;