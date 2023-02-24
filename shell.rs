#[allow(unused_imports)]
use colored::Colorize;
use std::{env};
use std::fs;
use std::io::{self, Write};
use hostname;
use username;
fn main() {
    
    arguments();
    // main {},
    loop {
        shell();
    }




}

// modules
// i am sad gpt gen this :C
fn directory() -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name().to_string_lossy().to_string();
                files.push(filename.clone());
                println!("{}", filename);
            }
        }
    }
    files
}


fn shell() {
    // shell input
    let user = username::get().unwrap_or_else(|_| "unknown".to_string().into());
    let user_string = user.to_string_lossy().to_string();
    // host
    let host = hostname::get().unwrap_or_else(|_| "unknown".to_string().into());
    let host_string = host.to_string_lossy().to_string();
    print!("{}{}{}{}", "@".blue().bold(), host_string.blue().bold(), user_string.blue().bold() ,"".blue().bold());
    io::stdout().flush().unwrap();
    let mut command = String::new();
    std::io::stdin().read_line(&mut command)
    .expect("Failed to read line");
    let command = command.trim();
    match command {
        "dir" | "ls" => {
            let output = directory();
            println!("{:?}", output);
        },

        "exit" | "quit" => std::process::exit(0),

        "clear" | "cls" => {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }

        command if command.starts_with("echo ") => {
            let echo_out = &command[5..];
            println!("{}", echo_out);
        },

        
        // wrong cmd | else
        _ => {
            println!("{}", "not a known command!".on_red());
        }
    };
}



// args
fn arguments() {
    // init args
    let args: Vec<String> = env::args_os()
    .skip(1)
    .filter_map(|arg| arg.into_string().ok())
    .collect();

    // help command | for command syntax;

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("{}", "=========================================".green());
        println!("{}", "             Untitled Program            ".blue());
        println!("{}", "=========================================".green())
    }
}