use clap::{Arg, ArgAction, Command};

fn main() {
    let build_cli = Command::new("cli")
        .version("0.0.1")
        .about("testing cli app")
        .arg_required_else_help(true)
        .arg(
            Arg::new("input")
                .required(true)
                .index(1)
                .value_name("FILES"),
        )
        .arg(
            Arg::new("lineCount")
                .short('l')
                .long("linecount")
                .help("line count")
                .long_help("provides number of line in a input file."),
        )
        .arg(Arg::new("flag").short('f').action(ArgAction::SetTrue))
        .arg(
            Arg::new("env")
                .long("env")
                .env("INPUT_FILE")
                .action(ArgAction::Set),
        )
        .get_matches();

    let parsed_cli = build_cli
        .get_one::<String>("input")
        .expect("no input provided");

    let flag_value = build_cli.get_flag("flag");

    println!("the value of input flag is {:?}", parsed_cli);
    println!("the value of input flag is {:?}", flag_value);
}
