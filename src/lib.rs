use anyhow::Result;
use colored::*;
use jwalk::WalkDir;
use std::path;

pub fn par_search(query: String, dir: path::PathBuf) -> Result<Vec<path::PathBuf>> {
    let mut paths = vec![];

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                let file_name = entry
                    .file_name
                    .to_str()
                    .expect("Failed parsing path to string");

                // TODO introduce fuzzy search
                if file_name == query {
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
