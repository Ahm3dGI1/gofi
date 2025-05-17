mod hotkey;
mod indexing;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hashed_files = indexing::hash_files("D:/".to_string());

    loop {
        println!("Enter your search query:");
        // Read user input
        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

        let query = query.trim();

        let files_names: Vec<String> = hashed_files.keys().cloned().collect();

        let matcher = SkimMatcherV2::default();

        let mut matches: Vec<(i64, PathBuf)> = files_names
            .iter()
            .filter_map(|name| {
                let name_score = matcher.fuzzy_match(name, query)?;
                let parent_score = hashed_files.get(name).unwrap().parent_path.len() as i64;
                let total_score = name_score * 3 - parent_score; // Weighted
                Some((
                    total_score,
                    hashed_files.get(name).unwrap().full_path.clone(),
                ))
            })
            .collect();

        matches.sort_by(|a, b| b.0.cmp(&a.0)); // Best matches first
        matches.truncate(10); // Limit to top 10 matches

        print!("Search results:\n");
        for (score, file) in matches.iter() {
            println!("Score: {}, Path: {}", score, file.display());
        }
    }

    Ok(())
}
