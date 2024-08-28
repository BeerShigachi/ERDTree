mod cli;
mod logging;

fn main() {
    logging::init_logger();
    cli::process_args();
}
