use std::error::Error;
use walkdir::WalkDir;

fn search(directories: &[String]) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                println!("{}", entry.path().display());
            }
        }
    }
    Ok(())
}
