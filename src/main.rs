use std::env::args;
use std::process::exit;

mod install;
mod version;

fn main() {
    let arguments: Vec<String> = args().collect();

    match arguments[1].as_str() {
        "--version" => version::print_version(),
        "-v" => version::print_version(),
        "--install" => install::install_file(arguments[2].clone()),

        _ => {
            println!("{:?} is not an option", arguments[1]);
            exit(1);
        }
    }
}
