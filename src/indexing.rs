use std::error::Error;
use walkdir::WalkDir;

pub fn get_paths(directory: String) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            paths.push(entry.path().to_path_buf());
        }
    }
    Ok(paths)
}
