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
        // mkdir feature
        command if command.starts_with("mkdir ") => {
            let path = &command[6..];
            match std::fs::create_dir(path) {
                Ok(_) => println!("Created directory {}", path),
                Err(_) => eprintln!("failed to find directory"),
            }

        }
        // remove directory
        command if command.starts_with("rmdir ") => {
            let path = &command[6..];
            match std::fs::remove_dir(path) {
                Ok(_) => println!("removed directory {}", path),
                Err(_) => eprintln!("cannot find directory specified"),
            }
        }

        // remove file (NOT REMOVE DIRECTORY)
        command if command.starts_with("rm ") => {
            let path = &command[3..];
            match std::fs::remove_file(path) {
                Ok(_) => println!("deleted {}", path),
                Err(_) => eprintln!("failed to delete file.")
            }
        }

        // create file, Different to makedir
        command if command.starts_with("mk ") => {
            let path = &command[3..];
            match std::fs::File::create(path) {
                Ok(_) => println!("Created {}", path),
                Err(_) => eprintln!("failed to create file.")
            }
        }

        // cat command (for reading files)
        command if command.starts_with("cat ") => {
            let path = &command[4..];
            match std::fs::read_to_string(path) {
                Ok(contents) => {
                    println!("{}", contents);
                },
                Err(err) => {
                    eprintln!("error {}", err);
                }
            }
        }
        // test command
        "aps" => {
            let output = std::process::Command::new("powershell")
                .arg("./aps.ps1")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        // wrong cmd | else
        _ => {
            eprintln!("{}", "not a known command!".on_red());
        }
    };
}