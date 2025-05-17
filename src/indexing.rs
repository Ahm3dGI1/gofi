use std::collections::HashMap;
use std::{error::Error, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub full_path: PathBuf,
    pub file_name: String,
    pub parent_path: String,
}

pub fn get_paths(directory: String) -> Result<Vec<IndexedFile>, Box<dyn Error>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let curr_path = IndexedFile {
                full_path: entry.path().to_path_buf(),
                file_name: entry.file_name().to_string_lossy().to_string(),
                parent_path: entry
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

pub fn hash_files(directory: String) -> HashMap<String, Vec<IndexedFile>> {
    let files = match get_paths(directory) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to index directory: {}", e);
            Vec::new()
        }
    };

    let mut file_hashes = HashMap::new();
    for file in files {
        file_hashes
            .entry(file.file_name.clone())
            .or_insert_with(Vec::new)
            .push(file);
    }

    file_hashes
}
