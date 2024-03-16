use crate::files::map_extension_to_file_type;
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
        } else if let Some((extension, file_type)) = map_extension_to_file_type(&path) {
            files.push(KnownFile {
                extension: extension.to_string(),
                file_type,
                path,
            });
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
