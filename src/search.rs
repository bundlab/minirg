use crate::args::Args;
use ignore::WalkBuilder;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let regex = Regex::new(&args.pattern)?;

    let walker = WalkBuilder::new(&args.path).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Err(e) = search_file(entry.path(), &regex) {
                        eprintln!("Skipping {}: {}", entry.path().display(), e);
                    }
                }
            }
            Err(err) => eprintln!("Walk error: {}", err),
        }
    }

    Ok(())
}

fn search_file(path: &Path, regex: &Regex) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (line_idx, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if regex.is_match(&line) {
            println!("{}:{}:{}", path.display(), line_idx + 1, line);
        }
    }

    Ok(())
}