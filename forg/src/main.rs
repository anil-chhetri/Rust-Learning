use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, author, long_about = "file organization app")]
struct Forg {
    #[command(subcommand)]
    forg_option: ForgOptions,

    #[arg(long, action=ArgAction::SetTrue
        ,help="provides the preview of changes with making any changes."
    )]
    preview: bool,
}

#[derive(Debug, Subcommand)]
enum ForgOptions {
    #[command(
        long_about = "scan the given directory and provide information on each extension found."
    )]
    Scan {
        #[arg(long, short, help = "file path to check")]
        file_path: String,
    },
    Organize,
}

fn main() {
    let fo = Forg::parse();

    println!("{:?}", fo);
}
