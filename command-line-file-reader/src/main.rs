use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file(path: &String) {
    let file = File::open(path);
    match file {
        Ok(file) => {
            println!("ready to read the file content");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("please check file name and path, can't find it: {}", error);
            }
            _ => {
                println!("can't open file to read {}", error)
            }
        },
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("path received: {}", path);
    if path.is_empty() {
        println!("please pass the file path.");
    }
    read_file(path);
    println!("program done.");
}
