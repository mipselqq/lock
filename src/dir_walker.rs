use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub fn walk_dir(dir: &Path) -> io::Result<Vec<DirEntry>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk_dir(&path)?);
            } else {
                files.push(entry);
            }
        }
    }

    Ok(files)
}
