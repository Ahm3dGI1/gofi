use eframe::egui;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

mod hotkey;
mod indexing;

#[derive(Default)]
struct GofiApp {
    query: String,
    last_query: String,
    file_map: HashMap<String, Vec<indexing::IndexedFile>>,
    shared_results: Arc<Mutex<Vec<(i64, String, PathBuf)>>>,
}

impl eframe::App for GofiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gofi: Fuzzy File Finder");

            // Search input
            let changed = ui.text_edit_singleline(&mut self.query).changed();

            if changed && self.query != self.last_query {
                self.last_query = self.query.clone();

                let query = self.query.clone();
                let file_map = self.file_map.clone();
                let shared_results = Arc::clone(&self.shared_results);

                thread::spawn(move || {
                    let results = fuzzy_search(&query, &file_map);
                    if let Ok(mut lock) = shared_results.lock() {
                        *lock = results;
                    }
                });
            }

            // Results display
            if let Ok(results) = self.shared_results.lock() {
                for (_score, name, path) in results.iter().take(10) {
                    ui.label(format!("{} : {}", name, path.display()));
                }
            }
        });
        ctx.request_repaint(); // Ensures smooth updates
    }
}

fn main() -> Result<(), eframe::Error> {
    let hashed_files = if std::path::Path::new("./cache/file_hashes.json").exists() {
        indexing::load_cache("./cache/file_hashes.json").unwrap_or_else(|_| {
            eprintln!("Cache not found, re-indexing...");
            indexing::hash_files("D:/".to_string())
        })
    } else {
        let index = indexing::hash_files("D:/".to_string());
        index
    };

    let app = GofiApp {
        file_map: hashed_files,
        ..Default::default()
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native("Gofi", options, Box::new(|_cc| Ok(Box::new(app))));

    Ok(())
}

pub fn fuzzy_search(
    query: &str,
    files: &HashMap<String, Vec<indexing::IndexedFile>>,
) -> Vec<(i64, String, PathBuf)> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<(i64, String, PathBuf)> = Vec::new();

    for (name, file_list) in files.iter() {
        if let Some(name_score) = matcher.fuzzy_match(name, query) {
            for file in file_list {
                let depth_score =
                    file.parent_path.matches(std::path::MAIN_SEPARATOR).count() as i64;
                let total_score = name_score * 3 - depth_score;
                matches.push((total_score, file.file_name.clone(), file.full_path.clone()));
            }
        }
    }

    matches.sort_by(|a, b| b.0.cmp(&a.0)); // Best matches first
    matches.truncate(10); // Limit to top 10 results

    matches
}
