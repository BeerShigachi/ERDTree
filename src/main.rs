mod cli;

fn main() {
    cli::parse_args();
    cli::validate_arg();
}
