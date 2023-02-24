mod shell;
mod arguments;
fn main() {
    // arguments main;
    arguments::arguments();
    
    loop {
        // shell main
        shell::shell();
    }
}
