#[allow(unused_imports)]
use colored::Colorize;
use hostname;
use std::io::{self, Write};

fn directory() -> String {
    let mut files = String::new();
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let filename = entry.file_name().to_string_lossy().to_string();
            let colored_filename = if entry.path().is_dir() {
                filename.blue().bold().to_string()
            } else {
                filename.normal().to_string()
            };
            files.push_str(&format!("{}  ", colored_filename));
        }
    }

    files
}

pub fn shell() {
    // shell headers | prompt
    let cwd = std::env::current_dir().unwrap();
    let cwd_string = cwd.to_string_lossy().to_string().replace(['\\', '/'], "");
    let host = hostname::get().unwrap_or_else(|_| "unknown".to_string().into());
    let _host_string = host.to_string_lossy().to_string();
    let checkmate_demo = format!("{} @", env!("USER"));

    print!(
        "{}{}{}{}{}",
        "".black().bold().on_bright_white(),
        checkmate_demo.black().bold().on_bright_white(),
        "".black().bright_white().bold().on_green(),
        cwd_string.black().to_string().on_green(),
        " ".green().bold()
    );
    io::stdout().flush().unwrap();

    let mut command = String::new();
    std::io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    let command = command.trim();

    match command {
        // dir command
                // no cmd
        "" => {}
        _ => {
            use std::io::{BufRead, BufReader};
            use std::process::Stdio;
            let mut splitted = command.split_whitespace();
            let mut pre_cmd = std::process::Command::new(splitted.next().unwrap());
            for arg in splitted {
                pre_cmd.arg(arg);
            }
            let mut cmd = match pre_cmd.stdout(Stdio::piped()).spawn() {
                Ok(c) => c,
                Err(err) => {
                    if err.kind() != io::ErrorKind::NotFound {
                        let err_msg =
                            "Problem running the command: ".to_string() + &err.to_string();
                        eprintln!("{}", err_msg.on_red());
                        return;
                    }
                    eprintln!("{}", "not a known command!".on_red());
                    return;
                }
            };
            let stdout = cmd.stdout.take().unwrap();

            let mut bufread = BufReader::new(stdout);
            let mut buf = String::new();
            let mut fbuf = String::new();
            while let Ok(num_read) = bufread.read_line(&mut buf) {
                if num_read > 0 {
                    print!("{}", buf);
                    fbuf += &buf;
                    buf.clear();
                } else {
                    break;
                }
            }
            if !fbuf.ends_with('\n') {
                println!("{}", "%".on_white().black());
            }
        }
    };
}

