use crate::dir_walker::walk_dir;
use crate::file_analyzer::{count_loc, match_file_type};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug, Clone)]
pub struct LanguageStat {
    pub extension: String,
    pub filetype: String,
    pub loc: u64,
    pub files_count: u64,
}

#[derive(Default, Debug, Clone)]
pub struct OverallStats {
    pub loc: u64,
    pub files_count: u64,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub overall: OverallStats,
    pub languages: HashMap<String, LanguageStat>,
}

pub fn gather_stats(path: &Path) -> Stats {
    let stats = Arc::new(Mutex::new(Stats::default()));

    let paths = walk_dir(path).unwrap();

    paths.par_iter().for_each(|dir_entry| {
        let path = dir_entry.path();
        let extension = path.extension().unwrap_or_default().to_str().unwrap();
        let loc = count_loc(&path).unwrap_or(0);
        let filetype = match_file_type(extension);

        let mut stats = stats.lock().unwrap();
        update_stats(&mut stats, extension, filetype, loc);
    });

    Arc::try_unwrap(stats).unwrap().into_inner().unwrap()
}

fn update_stats(stats: &mut Stats, extension: &str, filetype: Option<&str>, loc: u64) {
    stats.overall.loc += loc;
    stats.overall.files_count += 1;

    if let Some(filetype) = filetype {
        let language_stat = stats
            .languages
            .entry(extension.to_string())
            .or_insert_with(|| LanguageStat {
                extension: extension.to_string(),
                filetype: filetype.to_string(),
                loc: 0,
                files_count: 0,
            });

        language_stat.loc += loc;
        language_stat.files_count += 1;
    }
}
