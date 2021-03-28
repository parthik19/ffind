use anyhow::Result;
use colored::*;
use std::env;
use std::path;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLInput {
    query: String,

    #[structopt(parse(from_os_str))]
    starting_dir: Option<path::PathBuf>,

    #[structopt(short, long)]
    fuzzy: Option<u8>,
}

fn main() -> Result<()> {
    let input = CLInput::from_args();

    let start = Instant::now();
    let ffind_result = ffind::par_search(
        input.query,
        input
            .starting_dir
            .unwrap_or_else(|| env::current_dir().expect("Failed determining current directory")),
        input.fuzzy.map(|x| x as f64 / 100.0),
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
