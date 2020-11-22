mod command_handler;
mod command_parser;

use command_handler::CmdHandler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    let (first, remain) = args.split_at(2);
    
    CmdHandler::run_cmd(first[1].to_string(), remain.to_vec());
}
