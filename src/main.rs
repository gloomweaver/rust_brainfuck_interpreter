use clap::Parser;
use std::fs;
use std::process;

mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to program
    #[clap(value_parser)]
    program_path: String,

    /// Path to input file
    #[clap(short, long)]
    input_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let program = fs::read_to_string(args.program_path).unwrap_or_else(|err| {
        eprintln!("Error reading program: {}", err);
        process::exit(1);
    });

    let mut input = String::new();

    if let Some(input_path) = args.input_path {
        input = fs::read_to_string(input_path).unwrap_or_else(|err| {
            eprintln!("Error parsing input: {}", err);
            process::exit(1);
        });
    }

    let mut interpreter = lib::Interpreter::new(2048);
    interpreter.run(program, input);
    println!("{}", interpreter.get_output());
}
