mod dir_walker;
mod file_analyzer;
mod stats;

use clap::Parser;
use stats::gather_stats;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    gather_stats(path);
}
