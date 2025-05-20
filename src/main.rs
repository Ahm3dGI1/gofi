use eframe::egui;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

mod hotkey;
mod indexing;

#[derive(Default)]
struct GofiApp {
    query: String,
    results: Vec<(i64, PathBuf)>,
    file_map: HashMap<String, Vec<indexing::IndexedFile>>,
}

impl eframe::App for GofiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gofi: Fuzzy File Finder");

            // Search input
            let changed = ui.text_edit_singleline(&mut self.query).changed();

            if changed {
                self.results = fuzzy_search(&self.query, &self.file_map);
            }

            // Results display
            for (score, path) in &self.results {
                ui.label(format!("({}) {}", score, path.display()));
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hashed_files = if std::path::Path::new("./cache/file_hashes.json").exists() {
        indexing::load_cache("./cache/file_hashes.json")?
    } else {
        let index = indexing::hash_files("D:/".to_string());
        index
    };

    let app = GofiApp {
        file_map: hashed_files,
        ..Default::default()
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native("Gofi", options, Box::new(|_cc| Box::new(app)));

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
