mod cli;
mod task;

fn main() {
    let matches = cli::build_cli().get_matches();
    cli::handle_matches(matches);
}
