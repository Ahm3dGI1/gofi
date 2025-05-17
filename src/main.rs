mod hotkey;
mod indexing;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your search query:");
    // Read user input
    let mut query = String::new();
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");

    let query = query.trim();

    // Get all paths as Strings
    let all_files: Vec<indexing::IndexedFile> = indexing::get_paths("D:/".to_string())?;

    let matcher = SkimMatcherV2::default();

    let mut matches: Vec<(i64, indexing::IndexedFile)> = all_files
        .iter()
        .filter_map(|file| {
            let name_score = matcher.fuzzy_match(&file.file_name, query)?;
            let parent_score = file.parent_path.len() as i64;
            let total_score = name_score * 3 - parent_score; // Weighted
            Some((total_score, file.clone()))
        })
        .collect();

    matches.sort_by(|a, b| b.0.cmp(&a.0)); // Best matches first
    matches.truncate(10); // Limit to top 10 matches

    print!("Search results:\n");
    for (score, file) in matches.iter() {
        println!("Score: {}, Path: {}", score, file.full_path.display());
    }

    Ok(())
}
