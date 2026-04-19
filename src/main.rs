use std::process::ExitCode;

use scimantic::{NAME, VERSION};

const HELP: &str = "\
scimantic — command-line companion for Scimantic

Usage: scimantic [OPTIONS]

Options:
  -V, --version    Print version
  -h, --help       Print help
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.as_slice() {
        [] => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        [arg] if arg == "--version" || arg == "-V" => {
            println!("{NAME} {VERSION}");
            ExitCode::SUCCESS
        }
        [arg] if arg == "--help" || arg == "-h" => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("error: unrecognized arguments: {}", args.join(" "));
            eprint!("\n{HELP}");
            ExitCode::from(2)
        }
    }
}
