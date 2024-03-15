use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::{env, usize};

struct Stats {
    pub bytes: usize,
    pub chars: usize,
    pub lines: usize,
    pub max_line_lenght: usize,
    pub words: usize,
    pub path: String,
}

impl Stats {
    pub fn empty() -> Self {
        Self {
            bytes: 0,
            chars: 0,
            lines: 0,
            max_line_lenght: 0,
            words: 0,
            path: "".to_string(),
        }
    }
    pub fn new(chars: Vec<u8>, path: String) -> Self {
        let mut stats = Stats {
            bytes: chars.len(),
            chars: 0,
            lines: 0,
            max_line_lenght: 0,
            words: 0,
            path,
        };

        let mut in_word = false;
        let mut current_line = 0;

        for c in chars {
            let c = c as char;

            current_line += 1;

            if c != '\0' {
                stats.chars += 1;
            }

            if !c.is_whitespace() {
                in_word = true;
            } else if in_word {
                stats.words += 1;
                in_word = false;
            }

            if c == '\n' {
                if current_line > stats.max_line_lenght {
                    stats.max_line_lenght = current_line;
                }
                current_line = 0;
                stats.lines += 1;
            }
        }

        stats
    }
}

fn main() {
    let args = env::args().skip(1);
    let mut flags: Vec<char> = vec![];
    let mut files_paths: Vec<String> = vec![];
    let avaliable_flags = ['c', 'm', 'l', 'L', 'w'];

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

    if let Some(flag) = invalid_flag {
        panic!("ccwc: invalid option -- '{flag}'\nTry 'ccwc --help' for more information.")
    }

    if flags.is_empty() {
        flags.push('c');
        flags.push('l');
        flags.push('w');
    }

    let number_of_files = files_paths.len();
    let mut stats: Vec<Stats> = vec![];

    if number_of_files < 1 {
        let mut buffer = Vec::new();
        match io::stdin().read_to_end(&mut buffer) {
            Ok(_) => {
               stats.push(Stats::new(buffer, "".to_string())); 
            }, 
            Err(err) => { panic!("Error reading from stdin: {err}")}
        }
            
    }

    for path in files_paths {
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let bytes = reader.bytes().map(|byte| byte.unwrap()).collect();
                stats.push(Stats::new(bytes, path))
            }
            Err(_) => {
                println!("ccwc: {path}: No such file or directory\n");
            }
        }
    }

    let mut total_stats: Stats = Stats::empty();
    let number_of_stats = stats.len();

    for stat in stats {
        if flags.contains(&'l') {
            total_stats.lines += stat.lines;
            print!("{} ", stat.lines);
        }

        if flags.contains(&'w') {
            total_stats.words += stat.words;
            print!("{} ", stat.words);
        }

        if flags.contains(&'m') {
            total_stats.chars += stat.chars;
            print!("{} ", stat.chars);
        }

        if flags.contains(&'c') {
            total_stats.bytes += stat.chars;
            print!("{} ", stat.bytes);
        }

        if flags.contains(&'L') {
            if total_stats.max_line_lenght < stat.max_line_lenght {
                total_stats.max_line_lenght = stat.max_line_lenght;
            }
            print!("{} ", stat.max_line_lenght);
        }

        println!("{}", stat.path);
    }

    if number_of_stats > 1 {
        if flags.contains(&'l') {
            print!("{} ", total_stats.lines);
        }

        if flags.contains(&'w') {
            print!("{} ", total_stats.words);
        }

        if flags.contains(&'m') {
            print!("{} ", total_stats.chars);
        }

        if flags.contains(&'c') {
            print!("{} ", total_stats.bytes);
        }

        if flags.contains(&'L') {
            print!("{} ", total_stats.max_line_lenght);
        }

        println!("total");
    }
}
