use atty::Stream;
use rpn_test::rpn::{parse_rpn, execute_rpn};

use std::env;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let args = &args[1..];
        let thing = args.join(" ");
        do_thing(&thing);
    } else if atty::is(Stream::Stdin) {
        repl();
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if line.as_ref().unwrap().len() == 0 {
                continue;
            }
            do_thing(&line.unwrap());
        }
    }
}

fn print_repl_help() {
    println!("Welcome to the RPN calculator!");
    println!("Type 'quit' press Ctrl-D to exit.");
}

fn print_repl_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn repl() {
    print_repl_help();
    print_repl_prompt();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if line.as_ref().unwrap() == "quit" {
            break;
        }
        do_thing(&line.unwrap());
        print_repl_prompt();
    }
}

fn do_thing(thing: &str) {
    match parse_rpn(thing) {
        Ok(parsed) => match execute_rpn(&parsed) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error executing RPN: {}", e),
        },
        Err(e) => eprintln!("Error parsing RPN: {}", e),
    }
}
