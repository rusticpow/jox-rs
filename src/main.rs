use std::{
    borrow::Cow,
    fs::File,
    io::{stdin, Read},
    ops::Deref,
};

fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        print!("Usage: rlox [script]. Add args")
    } else if args.len() == 2 {
        let arg = args.nth(0).expect("Usage: rlox [script]");
        run_file(&arg);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let mut file = File::open(path).expect("File cannot be read");
    let mut buf = vec![];
    file.read_to_end(&mut buf).expect("File read failed");
    let listing = String::from_utf8_lossy(&buf);
    run(&listing)
}

fn run_prompt() {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Not null");
        if buffer.trim().is_empty() {
            break;
        }

        run(&buffer)
    }
}

fn run(text: &str) {
    let scanner = Scanner {
        source: &text
    };
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }

    println!("{}", text);
}

#[derive(Debug)]
struct Scanner<'a> {
    source: &'a str,
}

impl Scanner<'_> {
    fn scan_tokens(&self) -> Vec<Token> {
        vec![Token{
            token: "/"
        }]
    }
}

#[derive(Debug)]
struct Token<'a> {
    token: &'a str,
}