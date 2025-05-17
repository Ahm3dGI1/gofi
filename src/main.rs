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
    let all_paths: Vec<String> = indexing::get_paths("D:/".to_string())?
        .iter()
        .filter_map(|p| p.to_str().map(|s| s.to_string()))
        .collect();

    let matcher = SkimMatcherV2::default();

    let match_results: Vec<(i64, String)> = all_paths
        .iter()
        .filter_map(|path| {
            matcher
                .fuzzy_match(path, query)
                .map(|score| (score, path.clone()))
        })
        .collect();

    print!("Search results:\n");
    for (score, path) in match_results.iter() {
        println!("Score: {}, Path: {}", score, path);
    }

    Ok(())
}
