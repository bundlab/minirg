mod args;
mod search;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    if let Err(e) = search::run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
