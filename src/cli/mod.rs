mod parse;
mod validate;

use parse::parse_args;
use validate::validate_path;

pub fn process_args() -> String {
    let arg = match parse_args() {
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
