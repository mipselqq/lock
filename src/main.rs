mod dir_walk;
mod files;
mod print;
mod stats;

use clap::Parser;
use print::print_stats;
use stats::gather_stats;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    print_stats(gather_stats(path), true);
}
