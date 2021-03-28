use anyhow::Result;
use colored::*;
use jwalk::WalkDir;
use std::path;

pub fn par_search(
    query: String,
    dir: path::PathBuf,
    fuzzy_threshold: Option<f64>,
) -> Result<Vec<path::PathBuf>> {
    let mut paths = vec![];

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                let file_name = entry
                    .file_name
                    .to_str()
                    .expect("Failed parsing path to string");

                let is_match = if let Some(threshold) = fuzzy_threshold {
                    is_close(file_name, &query, threshold)
                } else {
                    file_name == query
                };

                if is_match {
                    paths.push(entry.path());
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

    Ok(paths)
}

fn is_close(a: &str, b: &str, threshold: f64) -> bool {
    strsim::normalized_levenshtein(a, b) >= threshold
}
