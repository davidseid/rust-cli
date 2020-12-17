use std::fs::File;
use std::io::{prelude:: *, BufReader};
use structopt::StructOpt;
use anyhow::{Context, Result};
use log::{info, debug, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    debug!("Arguments: {:?}", args);

    let file = File::open(&args.path)
        .expect("could not open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line
            .with_context(|| format!("could not read line"))?;

        find_matches(&content, &args.pattern);
    }
    Ok(())
}

fn find_matches(content: &str, pattern: &str) {
    if content.contains(pattern) {
        println!("{}", content);
    }
}
