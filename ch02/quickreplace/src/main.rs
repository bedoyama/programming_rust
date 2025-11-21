use std::env;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    println!(
        "Replacing '{}' with '{}' in file '{}' and writing to '{}'",
        args.target, args.replacement, args.filename, args.output
    );
}

fn print_usage() {
    eprintln!(
        "{} - change all occurrences of a string in a file",
        "quickreplace".green()
    );
    eprintln!(
        "Usage: {} TARGET REPLACEMENT FILENAME OUTPUT",
        "quickreplace".green()
    );
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{}: expected 4 arguments, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
