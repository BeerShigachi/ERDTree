pub fn parse_args(args: Vec<String>) -> Result<String, String> {
    match args.as_slice() {
        [arg] => Ok(arg.clone()),

        _ => Err(
            "Error: Expected exactly one argument.\nUsage: erd-tree <your_directory>".to_string(),
        ),
    }
}
