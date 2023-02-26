use std::io;
use std::io::Write;
#[allow(dead_code)]
pub struct Args {
    pub name: String,
    pub author: String,
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Flash error");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read from console. ");

    return s.trim().to_string();
}

#[allow(dead_code)]
pub fn get_args() -> Args {
    let n = input("Enter book name: ");
    let a = input("Enter author name: ");
    Args {
        name: n,
        author: a,
    }
}
