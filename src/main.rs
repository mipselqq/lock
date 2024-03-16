mod dir_walk;
mod files;
mod print;
mod stats;

use clap::Parser;
use dir_walk::{walk_dir, KnownFile};
use print::print_stats;
use stats::gather_stats;
use std::path::Path;

use crate::files::match_file_type;

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

    if !path.exists() {
        return println!("The directory doesn't exist");
    }

    let exclusions = args
        .exclude_list
        .as_ref()
        .map(|s| s.split(',').collect::<Vec<&str>>());

    let known_files = if path.is_dir() {
        walk_dir(path, exclusions).unwrap()
    } else {
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let file_type = match_file_type(&extension);

        let Some(file_type) = file_type else {
            return print!("The file is unknown");
        };

        vec![KnownFile {
            extension,
            file_type,
            path: path.to_path_buf(),
        }]
    };

    print_stats(gather_stats(known_files), true);
}
