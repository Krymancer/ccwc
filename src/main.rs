use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = env::args().skip(1);
    let mut flags: Vec<char> = vec![];
    let mut files_paths: Vec<String> = vec![];

    let avaliable_flags = vec!['c', 'm', 'l', 'L', 'w'];


    for arg in args {
        if arg.starts_with('-') {
           for flag in arg.chars().skip(1) {
                flags.push(flag);
            } 
        } else {
            files_paths.push(arg);
        }
    }

    let invalid_flag = flags.iter().find(|flag| !avaliable_flags.contains(flag));

    match invalid_flag {
        Some(flag) => panic!("ccwc: invalid option -- '{flag}'\nTry 'ccwc --help' for more information."),
        None => {},
    }

    let number_of_files = files_paths.len();

    if number_of_files < 1 {
        panic!("Reading from std in not yet implemented");
    }

    for path in files_paths {
        match File::open(&path) {
            Ok(file) => {
                if flags.len() < 1 {
                    // same as using -c -l -w 
                    // we have to print number of lines, words and bytes
                    let bytes = file.metadata().unwrap().len();
                    let reader = BufReader::new(file);
                    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                    let lines_count = lines.len();
                    let words = lines.iter().map(|line| line.split_whitespace()).flatten().count(); 
                    println!("{lines_count} {words} {bytes} {path}");
                }
            }, 
            Err(_) => {
                println!("ccwc: {path}: No such file or directory\n");
            }
        } 
    }

    if number_of_files > 1 {
        println!("We don't have rn but here should go the total of each file thingy");
    }
}
