mod dir_walk;
mod files;
mod print;
mod stats;

use clap::Parser;
use dir_walk::walk_dir;
use print::print_stats;
use stats::gather_stats;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,
    #[arg(short, long, default_value = None)]
    exclude_list: Option<String>,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let exclusions = args
        .exclude_list
        .as_ref()
        .map(|s| s.split(',').collect::<Vec<&str>>());

    let known_files = walk_dir(path, exclusions).unwrap();
    print_stats(gather_stats(known_files), true);
}
