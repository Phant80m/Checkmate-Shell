# Checkmate
## A simple shell written in Rust.
Features

    ls/dir: List files in the current directory
    cd: Change the current directory
    mkdir: Create a new directory
    rmdir: Remove a directory
    mk: Create a new file
    rm: Remove a file
    cat: Read the contents of a file
    echo: Print a message to the console (useless feature)
    clear/cls: Clear the console screen
    exit/quit/q: Exit the shell
    aps: Test command to execute a PowerShell script

Installation

Clone the repository and build it:
```powershell
cargo build --release
```
and run it with
```poweshell
cargo run --release
```
You can alternatively use makefile, `make run` or `make build`

Once the shell is running, you can enter commands at the prompt. Use the help command to see a list of available commands.

<img src="https://raw.githubusercontent.com/Phant80m/Checkmate-Shell/main/icons/square.jpg" width="200" height="200" />
