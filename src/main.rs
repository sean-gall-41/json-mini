use std::env;
use std::process;
use std::fs::File;
use std::io::{Read, Error};

mod lib;
use lib::{minify_json, prettify_json};

pub fn get_file_as_string(
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
	let input_json = get_file_as_string(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});

    let min_json = minify_json(input_json).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
    });

    let pretty_json = prettify_json(min_json).unwrap_or_else(|err| {
        eprintln!("Problem prettifying json {err}");
		process::exit(1);
    });
    println!("{}", pretty_json);

    //let input = String::from(r#"{"field_1": {"inner_field_1": "inner_value_1"}, "field_2": [true,false,true], "field_3":true}"#);
    //let pretty = prettify_json(input).unwrap_or_else(|err| {
    //    eprintln!("something went wrong while prettifying");
    //    process::exit(1);
    //});
    //println!("{pretty}");
    Ok(())
}

