use std::fs::File;
use std::io::{prelude:: *, BufReader};
use structopt::StructOpt;
use anyhow::{Context, Result};
use log::{debug};

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

        find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    }
    Ok(())
}

fn find_matches(line: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    if line.contains(pattern) {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
