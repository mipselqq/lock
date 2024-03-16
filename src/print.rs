use crate::stats::{LanguageStat, Stats};
use std::cmp::Reverse;

pub fn print_stats(stats: Stats, must_sort_languages_by_loc: bool) {
    let mut languages_stats_vec = stats.languages.into_values().collect::<Vec<LanguageStat>>();

    if must_sort_languages_by_loc {
        languages_stats_vec.sort_by_key(|stat| Reverse(stat.loc))
    }

    print_table_col("Type", "Extension", "Lines", "Files");
    print_horisontal_divisor();

    for stat in languages_stats_vec {
        print_table_col(
            &stat.filetype,
            &stat.extension,
            &stat.loc.to_string(),
            &stat.files_count.to_string(),
        );
    }

    print_horisontal_divisor();
    print_table_col(
        "Overall",
        "",
        &stats.overall.loc.to_string(),
        &stats.overall.files_count.to_string(),
    );
}

fn print_table_col(col1: &str, col2: &str, col3: &str, col4: &str) {
    println!("{:<15} {:<10} {:<10} {:<10}", col1, col2, col3, col4);
}

fn print_horisontal_divisor() {
    println!("{}", "-".repeat(15 + 10 + 10 + 10));
}
