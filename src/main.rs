use anyhow::Result;
use colored::*;
use std::env;
use std::path;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLInput {
    query: String,

    #[structopt(short, long)]
    fuzzy: bool, // defaults to false if this flag not present

    #[structopt(parse(from_os_str))]
    starting_dir: Option<path::PathBuf>,
}

fn main() -> Result<()> {
    let input = CLInput::from_args();

    let start = Instant::now();
    let ffind_result = ffind::par_search(
        input.query,
        input
            .starting_dir
            .unwrap_or_else(|| env::current_dir().expect("Failed determining current directory")),
        input.fuzzy,
    )?;
    let duration = start.elapsed().as_millis();

    if !ffind_result.is_empty() {
        for path in ffind_result {
            println!(
                "{}",
                path.to_str().expect("Failed parsing path").cyan().bold()
            );
        }
    } else {
        println!("{}", "No file matching that query found.".magenta());
    }

    println!("");

    println!("Took {} msecs.", duration);

    Ok(())
}
