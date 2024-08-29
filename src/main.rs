use log::{error, trace};

mod cli;
mod logging;

fn main() {
    match logging::init_logger() {
        Ok(()) => trace!("Initialized logger"),
        Err(e) => {
            error!("Failed to initialize logger: {}", e)
        }
    }
    cli::process_args();
}
