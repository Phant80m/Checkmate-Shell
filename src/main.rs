#[allow(unused_imports)]
use colored::Colorize;
use std::{env,};
mod command;
fn main() {
    arguments();
    loop {
        command::shell();

    }

}

// modules
// i am sad gpt gen this :C







// args
fn arguments() {
    // init args
    let args: Vec<String> = env::args_os()
    .skip(1)
    .filter_map(|arg| arg.into_string().ok())
    .collect();

    // help command | for command syntax;

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        //println!("{}", "=========================================".green().on_bright_white());
        //println!("{}", "            Checkmate Shellㅤㅤㅤㅤㅤㅤㅤ".blue().on_bright_white());
        //println!("{}", "=========================================".green().on_bright_white());
        println!("Commands:\n
            --help, shows the help prompt\n
            Run without args for shell!");
        std::process::exit(0);
    }
}