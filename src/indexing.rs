use std::{error::Error, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct IndexedPath {
    full_path: PathBuf,
    full_name: String,
    parent_folder: String,
}

pub fn get_paths(directory: String) -> Result<Vec<IndexedPath>, Box<dyn Error>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let curr_path = IndexedPath {
                full_path: entry.path().to_path_buf(),
                full_name: entry.file_name().to_string_lossy().to_string(),
                parent_folder: entry
                    .path()
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string(),
            };

            paths.push(curr_path);
        }
    }
    Ok(paths)
}
