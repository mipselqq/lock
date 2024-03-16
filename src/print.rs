use crate::stats::{LanguageStat, Stats};
use std::cmp::Reverse;

pub fn print_stats(stats: Stats, must_sort_languages_by_loc: bool) {
    let mut languages_stats = stats.languages.into_values().collect::<Vec<LanguageStat>>();

    if must_sort_languages_by_loc {
        languages_stats.sort_by_key(|stat| Reverse(stat.loc))
    }

    print_table_col("Type", "Extension", "Lines", "Files");
    print_horisontal_divisor();

    for stat in &languages_stats {
        print_table_col(
            &stat.file_type,
            &stat.extension,
            &stat.loc.to_string(),
            &stat.files_count.to_string(),
        );
    }

    if languages_stats.len() > 1 {
        print_horisontal_divisor();
        print_table_col(
            "Overall",
            "",
            &stats.overall.loc.to_string(),
            &stats.overall.files_count.to_string(),
        );
    }
}

// I hope nobody will use The WenYan Programming Language, right? XD
fn print_table_col(col1: &str, col2: &str, col3: &str, col4: &str) {
    println!("{:<23} {:<10} {:<10} {:<10}", col1, col2, col3, col4);
}

fn print_horisontal_divisor() {
    println!("{}", "-".repeat(23 + 10 + 10 + 10));
}
