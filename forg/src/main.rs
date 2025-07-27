use clap::{ArgAction, Parser, Subcommand};

use std::{
    collections::{HashMap},
    fs,
    io::Error,
    path::{Path, PathBuf},
};

fn pre_validation(path: &String) -> bool {
    Path::new(path).is_dir()
}

fn get_files(path: &String) -> Result<Vec<PathBuf>, Error> {
    if !pre_validation(path) {
        return Ok(Vec::new());
    }

    let mut all_files: Vec<PathBuf> = Vec::new();

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            continue;
        }
        all_files.push(entry.path());
    }
    Ok(all_files)
}

fn scan_files(path: &String) -> Result<String, Error> {
    let all_files = get_files(path)?;

    if all_files.len() == 0 {
        println!("no files found in {} directory.", path);
    }
    let mut scan_result = HashMap::new();
    
    for file in all_files{
        
        if let Some(extension) = file.extension().and_then(|e| e.to_str()){
            *scan_result.entry(extension.to_string()).or_insert(0) += 1;
        }
    }

    println!("{:?}", scan_result);

    Ok("test".to_string())
}

#[derive(Debug, Parser)]
#[command(version, author, long_about = "file organization app")]
struct Forg {
    #[command(subcommand)]
    forg_option: ForgOptions,

    #[arg(long, action=ArgAction::SetTrue
        ,help="provides the preview of changes with making any changes."
    )]
    preview: bool,

    #[arg(long, short, help("should look through sub folder or not.")
    , action = ArgAction::SetTrue
    )]
    recursive: bool,
}

#[derive(Debug, Subcommand)]
enum ForgOptions {
    #[command(
        arg_required_else_help(true),
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

    match fo.forg_option {
        ForgOptions::Organize => {
            println!("received organize");
        }
        ForgOptions::Scan { file_path } => {
            // println!("Recived scan with file_path: {}", file_path);
            // println!("preview flag: {}", fo.preview);
            // println!("recursive flag: {}", fo.recursive);

            // get_files(&file_path).expect("can't process the file path");
            scan_files(&file_path).expect("can' t process file in directory");
        }
    }
}
