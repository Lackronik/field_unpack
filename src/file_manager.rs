use rayon::prelude::*;
use walkdir::WalkDir;
use std::path::PathBuf;

pub fn collect_files(dir: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .par_bridge() // Parallel iterator
        .filter_map(|entry| {
            let entry = entry.ok()?; // Handle potential errors
            let path = entry.path();
            if path.extension().map_or(false, |s| s == "c" || s == "h") {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .collect()
}
