use std::io::Read;
use std::process::{Command, Stdio};
pub struct CmdHandler;

impl CmdHandler {
    pub fn run_cmd(cmd: String, args: Vec<&str>) {
        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
        let mut child_out = child.stdout.take().unwrap();
        let mut buffer = [0; 10000];

        loop {
            let n = child_out.read(&mut buffer[..]).unwrap();
            if n == 0 {
                break;
            }
            println!(
                "{}",
                buffer
                    .iter()
                    .map(|&x| x as char)
                    .map(|x| x.to_string())
                    .collect::<String>()
            );
        }

        child.kill();
    }
}
