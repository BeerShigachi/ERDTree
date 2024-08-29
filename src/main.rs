use log::{error, info};

mod cli;
mod logging;

fn main() {
    match logging::init_logger() {
        Ok(()) => info!("Initialized logger"),
        Err(err) => {
            error!("Error: Failed to initialize logger: {}", err)
        }
    }

    let _path = match cli::get_path_from_args() {
        Ok(arg) => arg,
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    };
}
