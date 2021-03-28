use anyhow::Result;
use colored::*;
use jwalk::WalkDir;
use std::path;

pub fn par_search(
    query: String,
    dir: path::PathBuf,
    use_fuzzy_search: bool,
) -> Result<Vec<path::PathBuf>> {
    let mut paths = vec![];

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                let file_name = entry
                    .file_name
                    .to_str()
                    .expect("Failed parsing path to string");

                // TODO make sure is_close does NOT get evaluated if use_fuzzy_search is false
                if file_name == query || (use_fuzzy_search && is_close(file_name, &query)) {
                    paths.push(entry.path())
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

fn is_close(a: &str, b: &str) -> bool {
    strsim::normalized_levenshtein(a, b) >= 0.75
}
