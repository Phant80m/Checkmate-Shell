#[allow(unused_imports)]
mod command;
mod arguments;
fn main() {
    arguments::arguments();
    loop {
        command::shell();

    }

}
