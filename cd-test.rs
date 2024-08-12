use std::process::Command;

fn main() {
    println!("Hello, World!");
    Command::new("cd").arg("./src").spawn().expect("failed");
}
