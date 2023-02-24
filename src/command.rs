#[allow(unused_imports)]
use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use hostname;

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
pub fn shell() {
    let cwd = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_string_lossy().to_string().replace("\\", "");
    let host = hostname::get().unwrap_or_else(|_| "unknown".to_string().into());
    let host_string = host.to_string_lossy().to_string();


    print!("{}{}{}{}{}", "".black().bold().on_bright_white(), host_string.black().bold().on_bright_white(), "".black().bright_white().bold().on_green(), cwd_string.black().to_string().on_green(), "".green().bold());
    io::stdout().flush().unwrap();


    let mut command = String::new();
    std::io::stdin().read_line(&mut command)
    .expect("Failed to read line");
    let command = command.trim();
    



    match command {
        // dir command
        "dir" | "ls" => {

            let output = directory();
            for file in output {
                println!("{}", file);
            }

        },

        // exit feature
        "exit" | "quit" | "q" => std::process::exit(0),

        // clear screem
        "clear" | "cls" => {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }

        // echo command (useless rn)
        command if command.starts_with("echo ") => {
            let echo_out = &command[5..];
            println!("{}", echo_out);
        },

        // change dir! works with shell prompt
        command if command.starts_with("cd ") => {
            let path = &command[3..];   
            if let Err(e) = std::env::set_current_dir(path) {
                eprintln!("Failed to find directory {}", e)
            }
        },
        command if command.starts_with("mkdir ") => {
            let path = &command[6..];
            match std::fs::create_dir(path) {
                Ok(_) => println!("Created directory {}", path),
                Err(e) => eprintln!("failed to find directory"),
            }

        }
        // wrong cmd | else
        _ => {
            eprintln!("{}", "not a known command!".on_red());
        }
    };
}