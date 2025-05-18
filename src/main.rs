mod hotkey;
mod indexing;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hashed_files = if std::path::Path::new("./cache/file_hashes.json").exists() {
        indexing::load_cache("./cache/file_hashes.json")?
    } else {
        let index = indexing::hash_files("D:/".to_string());
        index
    };

    loop {
        println!("Enter your search query:");
        // Read user input
        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

        let query = query.trim();

        if query.is_empty() {
            println!("Exiting search.");
            break;
        }

        let matches = fuzzy_search(query, &hashed_files);

        println!("Search results:\n");
        for (score, file) in matches.iter() {
            println!("Score: {}, Path: {}", score, file.display());
        }
    }

    Ok(())
}

pub fn fuzzy_search(
    query: &str,
    files: &HashMap<String, Vec<indexing::IndexedFile>>,
) -> Vec<(i64, PathBuf)> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<(i64, PathBuf)> = Vec::new();

    for (name, file_list) in files.iter() {
        if let Some(name_score) = matcher.fuzzy_match(name, query) {
            for file in file_list {
                let depth_score =
                    file.parent_path.matches(std::path::MAIN_SEPARATOR).count() as i64;
                let total_score = name_score * 3 - depth_score;
                matches.push((total_score, file.full_path.clone()));
            }
        }
    }

    matches.sort_by(|a, b| b.0.cmp(&a.0)); // Best matches first
    matches.truncate(10); // Limit to top 10 results

    matches
}
