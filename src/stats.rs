use crate::dir_walk::KnownFile;
use crate::files::count_loc;
use rayon::prelude::*;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

#[derive(Default, Debug, Clone)]
pub struct LanguageStat {
    pub extension: String,
    pub file_type: String,
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

pub fn gather_stats(known_files: Vec<KnownFile>) -> Stats {
    let stats = Arc::new(Mutex::new(Stats::default()));

    known_files.into_par_iter().for_each(|known_file| {
        let mut stats = stats.lock().unwrap();

        consider_file_stats(&mut stats, known_file);
    });

    Arc::try_unwrap(stats).unwrap().into_inner().unwrap()
}

fn consider_file_stats(stats: &mut Stats, known_file: KnownFile) {
    let KnownFile {
        extension,
        file_type,
        path,
    } = known_file;

    let loc = count_loc(&path).unwrap_or(0);

    stats.overall.loc += loc;
    stats.overall.files_count += 1;

    let language_stat = stats
        .languages
        .entry(extension.to_string())
        .or_insert_with(|| LanguageStat {
            extension: extension.to_string(),
            file_type: file_type.to_string(),
            loc: 0,
            files_count: 0,
        });

    language_stat.loc += loc;
    language_stat.files_count += 1;
}
