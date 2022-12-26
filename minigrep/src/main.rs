use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return usage(&args);
    }
    let pattern = match Regex::new(&args[1]) {
        Err(what) => panic!("{}", what),
        Ok(p) => p,
    };


    if args.len() < 3 || args[2] == "-" {
        let mut lines = io::stdin().lines();
        while let Some(Ok(line)) = lines.next() {
            match_and_print(&pattern, &line);
        }
    } else {
        for filename in &args[2..] {
            let mut lines = match File::open(filename) {
                Err(what) => panic!("Unabled to open {}: {}", filename, what),
                Ok(fh) => BufReader::new(fh).lines()
            };
            while let Some(Ok(line)) = lines.next() {
                match_and_print(&pattern, &line);
            }
        }
    }

}

fn match_and_print(pattern: &Regex, line: &String) {
    if pattern.is_match(line) {
        println!("{}", line);
    }

}

fn usage(args: &Vec<String>) {
    let name = &args[0];
    println!("Usage: {name} <pattern> [file...]");
}


