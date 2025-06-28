use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("echor")
        .version("0.0.1")
        .author("anil")
        .about("rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text to display")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("do not print newline at the end")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("Text is required")
        .cloned()
        .collect();

    let isnewline = matches.get_flag("omit_newline");

    println!(
        "{}{}",
        text.join(" "),
        if isnewline == true { "\n" } else { "" },
    );
}
