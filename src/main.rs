use std::process::{Command, Stdio};
use std::io::Read;

mod command_parser;

fn main() {
    let mut child = Command::new("ping")
                     .args(vec!["google.com", "-c 6"])
                     .stdout(Stdio::piped())
                     .spawn()
                     .expect("Failed to execute command");
    let mut child_out = child.stdout.take().unwrap();
    let mut buffer = [0; 100000];
    loop {
        let n = child_out.read(&mut buffer[..]).unwrap();
        if n == 0 {
            break;
        }
        println!("{}", buffer.iter().map(|&x| x as char).map(|x| x.to_string()).collect::<String>());
    }
    child.kill();
}
