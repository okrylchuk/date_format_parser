use date_format_parser::parse_date;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_help();
        return;
    }

    match args[1].as_str() {
        "--help" => display_help(),
        "--credits" => display_credits(),
        input => parse_and_display(input),
    }
}

fn display_help() {
    println!("Date-Time Parser CLI - Usage:");
    println!("  cargo run <date-time>       Parses the given date or date-time string and outputs the result in ISO formatted string.");
    println!("  cargo run -- --help            Displays this help message.");
    println!("  cargo run -- --credits         Shows project credits.");
}

fn display_credits() {
    println!("Date-Format Parser CLI");
    println!("Developed by Oleksiy Krylchuk");
    println!("Version 0.1.0");
    println!("Powered by Rust and Pest parsing library.");
}

fn parse_and_display(input: &str) {
    match parse_date(input) {
        Ok(date_parsed) => println!("Parsed Result: {}", date_parsed),
        Err(e) => eprintln!("Error: {}", e),
    }
}
