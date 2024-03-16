use std::{fs, io, path::Path};

pub fn match_file_type(extension: &str) -> Option<&'static str> {
    match extension {
        "rs" => Some("Rust"),
        "ts" => Some("TypeScript"),
        "txt" => Some("Text file"),
        "js" => Some("JavaScript"),
        "jsx" => Some("JavaScript React"),
        "tsx" => Some("TypeScript React"),
        "html" => Some("HTML"),
        "css" => Some("CSS"),
        "scss" => Some("Sass"),
        "java" => Some("Java"),
        "kt" => Some("Kotlin"),
        "go" => Some("Go"),
        "py" => Some("Python"),
        "c" => Some("C"),
        "cpp" => Some("C++"),
        "h" => Some("C Header"),
        "hpp" => Some("C++ Header"),
        "cs" => Some("C#"),
        "m" => Some("Objective-C"),
        "mm" => Some("Objective-C++"),
        "swift" => Some("Swift"),
        "rb" => Some("Ruby"),
        "php" => Some("PHP"),
        "pl" => Some("Perl"),
        "lua" => Some("Lua"),
        "sh" => Some("Shell Script"),
        "bash" => Some("Bash Script"),
        "yml" => Some("YAML"),
        "yaml" => Some("YAML"),
        "json" => Some("JSON"),
        "xml" => Some("XML"),
        "sql" => Some("SQL"),
        "md" => Some("Markdown"),
        "toml" => Some("TOML"),
        "ini" => Some("INI"),
        "env" => Some("Environment Variables"),
        _ => None,
    }
}

pub fn count_loc(file_path: &Path) -> io::Result<u64> {
    Ok(fs::read_to_string(file_path)?.lines().count() as u64)
}
