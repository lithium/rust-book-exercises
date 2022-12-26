use std::env;
use std::fmt::Display;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;





fn usage(args: &Vec<String>) {
    eprintln!("Usage: {0} <pattern> [file...]", args[0]);
}

struct Cli<'a> {
    pattern: &'a String,
    paths: Vec<&'a String>,
    inverted: bool
}

fn parse_args<'a>(args: &'a Vec<String>) -> Result<Cli<'a>, &'static str> {
    if args.len() < 2 {
        return Err("<pattern> is required")
    }

    let mut pattern: Option<&String> = None;
    let mut paths = vec![];
    let mut inverted = false;

    for a in args[1..].iter() {
        if "-v" == a.as_str() {
            inverted = true;
        }
        else if let None = pattern {
            pattern = Some(a);
        } else {
            paths.push(a)
        }

    }

    return match pattern { 
        None => Err("<pattern> is required"),
        Some(p) => Ok(Cli {
            pattern: p,
            paths: paths,
            inverted: inverted
        }),
    };

}

fn error(reason: &dyn Display) {
    eprintln!("{}", reason);
    std::process::exit(1);
}


/*
 * Entrypoint
 */
fn main() {
    // parse arguments
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) if ["--help", "-h", "-?"].contains(&s.as_str()) =>
            return usage(&args),
        Some(_) | None => ()
    }

    let cli = match parse_args(&args) {
        Err(_what) => return usage(&args),
        Ok(cli) => cli
    };

    dbg!(cli.pattern, &cli.paths, cli.inverted);


    // build pattern
    let pattern = match Regex::new(cli.pattern) {
        Err(what) => return error(&what),
        Ok(p) => p,
    };


    // process input
    if cli.paths.len() < 1 || cli.paths[0] == "-" {
        // match against stdio 
        let mut lines = io::stdin().lines();
        while let Some(Ok(line)) = lines.next() {
            match_and_print(&pattern, &line, &cli);
        }
    } else {
        // match against all paths
        for filename in &cli.paths {
            let mut lines = match File::open(filename) {
                Err(what) => return error(&format!("Unable to open '{}': {}", filename, what)),
                Ok(fh) => BufReader::new(fh).lines()
            };
            while let Some(Ok(line)) = lines.next() {
                match_and_print(&pattern, &line, &cli);
            }
        }
    }

}


fn match_and_print(pattern: &Regex, line: &String, cli: &Cli) {
    let matches = pattern.is_match(line);

    if (match cli.inverted { true => !matches, false => matches }) {
        println!("{}", line);
    }
}
