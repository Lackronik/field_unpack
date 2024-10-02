use walkdir::WalkDir;
use std::path::PathBuf;

pub fn collect_files(dir: &str) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |s| s == "c" || s == "h") {
            files.push(path.to_path_buf());
        }
    }
    files
}
