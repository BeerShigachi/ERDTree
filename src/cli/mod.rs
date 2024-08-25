mod parse;
mod validate;

use std::env;

use parse::parse_args;
use validate::validate_path;

pub fn process_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg = match parse_args(args) {
        Ok(arg) => arg,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match validate_path(&arg) {
        Ok(()) => arg,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
