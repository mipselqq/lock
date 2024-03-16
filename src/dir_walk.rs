use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::files::match_file_type;

pub struct KnownFile {
    pub extension: String,
    pub file_type: &'static str,
    pub path: PathBuf,
}

pub fn walk_dir(dir: &Path) -> io::Result<Vec<KnownFile>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            if path.is_dir() {
                files.extend(walk_dir(&path)?);
            } else {
                let extension = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                if let Some(file_type) = match_file_type(&extension) {
                    files.push(KnownFile {
                        extension,
                        file_type,
                        path,
                    });
                }
            }
        }
    }

    Ok(files)
}
