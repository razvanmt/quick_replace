use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

// struct for the cli arguments
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

// This function will print to the standard error a few useful information about the usage of this small tool
fn print_usage(){
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

// We collect the arguments into a Vector
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

// The replace function uses regex to replace text and returns a Result with an error type from regex
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}


fn main() {
    // the args variable holds the parsed arguments
    let args = parse_args();
    
    // We use std::fs to read from file and handle the variants of the Result enum for this particular thing
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprint!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };


    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprint!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };
    
    // Write the replaced_data to the output
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprint!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

}