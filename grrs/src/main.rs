use std::fs::File;
use std::io::{prelude:: *, BufReader};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    // The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    println!("Arguments: {:?}", args);

    let file = File::open(&args.path)
        .expect("could not open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        } else {
            println!("Error reading line: {:?}", line);
        }
    }
}
