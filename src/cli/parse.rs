use std::env;

pub fn parse_args() -> Option<String> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [arg] => Some(arg.clone()),

        _ => {
            eprintln!("Error: Expected exactly one argument.");
            eprintln!("Usage: erd-tree <your_directory>");
            std::process::exit(1);
        }
    }
}
