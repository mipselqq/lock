use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
