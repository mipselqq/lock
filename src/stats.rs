use crate::dir_walker::walk_dir;
use crate::file_analyzer::{count_loc, match_file_type};
use std::collections::HashMap;
use std::path::Path;

#[derive(Default, Debug)]
struct LanguageStat {
    extension: String,
    filetype: String,
    loc: u64,
    files_count: u64,
}

#[derive(Default, Debug)]
struct OverallStats {
    loc: u64,
    files_count: u64,
}

#[derive(Default, Debug)]
struct Stats {
    overall: OverallStats,
    languages: HashMap<String, LanguageStat>,
}

pub fn gather_stats(path: &Path) {
    let mut stats = Stats::default();

    walk_dir(path, &mut |dir_entry| {
        let path = dir_entry.path();
        let extension = path.extension().unwrap_or_default().to_str().unwrap();
        let loc = count_loc(&path).unwrap_or(0);
        let filetype = match_file_type(extension);

        update_stats(&mut stats, extension, filetype, loc);
    })
    .unwrap();

    dbg!(stats);
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
