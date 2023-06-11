use std::env;
use std::process;
use std::fs::File;
use std::io::{Read, Error};

mod lib;
use lib::{Token, Lexer};

const IGNORE_WS: bool = true;
const NO_IGNORE_WS: bool = false;

pub fn get_file_as_str(
    mut args: impl Iterator<Item = String>,
) -> Result<String, &'static str> {
    args.next(); // first val in env::args() is name of program
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    let mut file = File::open(file_path).expect("Failed to open input file");
    let mut file_as_str = String::new();
    file.read_to_string(&mut file_as_str).expect("Failed to read input file");
    Ok(file_as_str)
}

fn main() -> Result<(), Error> {

	let input_str = get_file_as_str(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});

    let mut lexer = Lexer::from(input_str, IGNORE_WS);
    let lexed_input = lexer.lex().unwrap_or_else(|err| {
        eprintln!("Problem lexing input: {err}");
        process::exit(1);
    });
    println!("{:#?}", lexed_input);
    Ok(())
}

