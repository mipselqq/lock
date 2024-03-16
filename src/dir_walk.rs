use crate::files::match_file_type;
use glob::Pattern;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct KnownFile {
    pub extension: String,
    pub file_type: &'static str,
    pub path: PathBuf,
}

pub fn walk_dir(dir: &Path, exclusions: Option<Vec<&str>>) -> io::Result<Vec<KnownFile>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if let Some(ref exclusions) = exclusions {
            if check_if_path_must_be_excluded(&path, exclusions) {
                continue;
            }
        }

        if path.is_dir() {
            files.extend(walk_dir(&path, exclusions.clone())?);
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

    Ok(files)
}

fn check_if_path_must_be_excluded(path: &Path, exclusions: &[&str]) -> bool {
    exclusions.iter().any(|&exclusion| {
        Pattern::new(exclusion)
            .unwrap()
            .matches(path.to_str().unwrap())
    })
}
