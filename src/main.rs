mod hotkey;
mod indexing;

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

    // Filter search results
    let search_result: Vec<String> = all_paths
        .iter()
        .filter(|path| path.to_lowercase().contains(&query.to_lowercase()))
        .cloned()
        .collect();

    print!(
        "{} results found for query '{}':\n",
        search_result.len(),
        query
    );

    Ok(())
}
