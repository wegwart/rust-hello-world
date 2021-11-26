use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let filename  = match env::args().skip(1).next(){
        Some(filename) => filename,
        None => "./resources/dmesg_sample.log".into()
    };

    let file = match fs::read_to_string(filename) {
        Ok(file ) => file,
        Err(_) => { 
            panic!("File not found");
        }
    };
    for line in file.lines() {
        if let Some(first) = line.split("]").next() {
            println!("{}", &first[2..]);
        }    
    }
}

