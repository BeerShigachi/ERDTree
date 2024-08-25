use std::path::Path;

pub fn validate_path(arg: &str) -> Result<(), String> {
    let path = Path::new(arg);

    match (path.exists(), path.is_dir()) {
        (true, true) => Ok(()),
        (true, false) => Err(format!(
            "Error: The provided path '{}' exists but is not a directory.",
            arg
        )),
        (false, _) => Err(format!(
            "Error: The provided path '{}' does not exist.",
            arg
        )),
    }
}
