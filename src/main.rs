mod command_handler;
mod command_parser;

use clap::{App, Arg};
use command_handler::CmdHandler;
use command_parser::Parser;

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

        String::from("--help"),
    ];

    let args: Vec<String> = std::env::args().collect();

    if !allowd_cmd.contains(&args[1]) {
        return false;
    }

    let matches = App::new("My Super Program")
        .version("0.1")
        .author("Gaurav Tyagi")
        .about("Does awesome things")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .value_name("TEXT")
                .multiple(true)
                .help("Set/Update a key -> value mapping."),
        )
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .value_name("TEXT")
                .help("Remove a key -> value mapping."),
        )
        .get_matches();

    if let Some(ref mut data) = matches.values_of("add") {

        parser.alias_manager.set_alias(
            data.next().unwrap().to_string(),
            data.next().unwrap().to_string(),
        );
        return true;
    }

    if let Some(data) = matches.value_of("remove") {
        println!("{:?}", data);
        parser.alias_manager.drop_alias(data);
        return true;
    }

    return false;
}
