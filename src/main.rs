mod dir_walk;
mod files;
mod print;
mod stats;

use clap::Parser;
use dir_walk::{walk_dir, KnownFile};
use print::print_stats;
use stats::gather_stats;
use std::path::Path;

use crate::files::map_extension_to_file_type;

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

    let known_files = if path.is_dir() {
        let exclusions = args
            .exclude_list
            .as_ref()
            .map(|s| s.split(',').collect::<Vec<&str>>());

        let Ok(known_files) = walk_dir(path, exclusions) else {
            return println!("Failed to read a directory when searching for files");
        };

        known_files
    } else {
        let Some((extension, file_type)) = map_extension_to_file_type(path) else {
            return println!("The file is unknown");
        };

        vec![KnownFile {
            extension: extension.to_string(),
            file_type,
            path: path.to_path_buf(),
        }]
    };

    print_stats(gather_stats(known_files), true);
}
