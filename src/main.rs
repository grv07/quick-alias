mod command_handler;
mod parser_handler;

use clap::{App, Arg};
use command_handler::CmdHandler;
use parser_handler::Parser;
use std::io::{self, Write};

fn main() {
    let ref parser = Parser::new();
    let is_found = cli_handler(parser);

    if !is_found {
        let args: Vec<String> = std::env::args().collect();
        let ref raw_string = parser.parse(args.join(" ")).unwrap();
        let args: Vec<&str> = raw_string.split(" ").collect();
        let (first, remain) = args.split_at(2);
        CmdHandler::run_cmd(first[1].to_string(), remain.to_vec());
    }
}

fn cli_handler(parser: &Parser) -> bool {
    let allowd_cmd: Vec<String> = vec![
        String::from("add"),
        String::from("-a"),
        String::from("remove"),
        String::from("-r"),
        String::from("explain"),
        String::from("-e"),
        String::from("list"),
        String::from("-l"),
        String::from("--help"),
    ];

    let args: Vec<String> = std::env::args().collect();

    if !allowd_cmd.contains(&args[1]) {
        return false;
    }

    let matches = App::new("Quick alias Program")
        .version("0.1")
        .author("Gaurav Tyagi")
        .about("Add/Drop aliases fast")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .value_name("TEXT")
                .multiple(true)
                .help("Set/Update a key -> value mapping."),
        )
        .arg(
            Arg::with_name("explain")
                .short("e")
                .long("explain")
                .value_name("TEXT")
                .multiple(true)
                .help("Show underneath command."),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .value_name("TEXT")
                .multiple(true)
                .help("Show all saved aliases."),
        )
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .value_name("TEXT")
                .help("Remove a key -> value mapping."),
        )
        .get_matches();

    if let Some(mut data) = matches.values_of("explain") {
        let raw_string = {
            let data = data.next().unwrap();
            parser.parse(data.to_string()).unwrap()
        };
        let stdio = io::stdout();
        let mut handle = stdio.lock();
        handle
            .write_all(raw_string.as_bytes())
            .expect("Not able to write :(");
        handle.write(b"\n").unwrap();
        return true;
    }

    if let Some(ref mut data) = matches.values_of("add") {
        parser.alias_manager.set_alias(
            data.next().unwrap().to_string(),
            data.next().unwrap().to_string(),
        );
        return true;
    }

    if matches.is_present("list") {
        let data = parser.alias_manager.get_all_aliases_mapping();
        let stdio = io::stdout();
        let mut handle = stdio.lock();
        let data = data.join(" \n");
        handle
            .write_all(data.as_bytes())
            .expect("Not able to write :(");
        return true;
    }

    if let Some(data) = matches.value_of("remove") {
        parser.alias_manager.drop_alias(data);
        return true;
    }

    return false;
}
