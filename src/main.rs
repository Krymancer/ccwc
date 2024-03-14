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

    if flags.len() < 1 {
        flags.push('c');
        flags.push('l');
        flags.push('w');
    }

    for path in files_paths {
        match File::open(&path) {
            Ok(file) => {
                let bytes = file.metadata().unwrap().len();
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                
                if flags.contains(&'l') {
                    let lines_count = lines.len();
                    print!("{lines_count} ");
                }

                if flags.contains(&'w') {
                    let words = lines.iter().map(|line| line.split_whitespace()).flatten().count(); 
                    print!("{words} ");
                }

                if flags.contains(&'m') {
                    let chars = lines.iter().map(|line| line.chars()).flatten().count() + lines.len(); 
                    print!("{chars} ");
                }

                if flags.contains(&'c') {
                    print!("{bytes} ");
                }

                if flags.contains(&'L') {
                    let max_line_lenght = lines.iter().map(|line| line.len()).max().unwrap();
                    print!("{max_line_lenght} ");
                }

                println!("{path}");
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
