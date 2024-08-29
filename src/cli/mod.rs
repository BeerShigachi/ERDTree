mod parse;
mod validate;

use parse::parse_args;
use std::env;
use validate::validate_path;

pub fn get_path_from_args() -> Result<String, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg = match parse_args(args) {
        Ok(arg) => arg,
        Err(err) => return Err(err),
    };

    match validate_path(&arg) {
        Ok(()) => Ok(arg),
        Err(err) => Err(err),
    }
}
