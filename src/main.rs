use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[2];

    let file = File::open(file_path).expect("file not open");

    let length = file.metadata().unwrap().len();

    println!("{length} {file_path}")

}
