use colored::*;
use jwalk::WalkDir;
use std::path;

pub fn par_search(query: String, dir: path::PathBuf, fuzzy_threshold: Option<f64>) -> Vec<String> {
    let mut paths = vec![];

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                if let Some(file_name) = entry.file_name.to_str() {
                    let is_match = if let Some(threshold) = fuzzy_threshold {
                        is_close(file_name, &query, threshold)
                    } else {
                        file_name == query
                    };

                    if is_match {
                        if let Some(file_path) = entry.path().to_str() {
                            paths.push(file_path.to_string());
                        } else {
                            eprintln!("{}", "Failed parsing file path".red());
                        }
                    }
                } else {
                    eprintln!("{}", "Failed parsing file name.".red());
                }
            }
            Err(e) => {
                eprintln!(
                    "{}: {}",
                    "Encountered error when reading directory".red(),
                    e.to_string()
                );
            }
        }
    }

    paths
}

fn is_close(a: &str, b: &str, threshold: f64) -> bool {
    strsim::normalized_levenshtein(a, b) >= threshold
}
