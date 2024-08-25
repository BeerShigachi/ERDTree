use std::env;

pub fn parse_args() -> Result<String, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [arg] => Ok(arg.clone()),

        _ => Err(
            "Error: Expected exactly one argument.\nUsage: erd-tree <your_directory>".to_string(),
        ),
    }
}
