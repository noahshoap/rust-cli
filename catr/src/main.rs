use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value = "-")]
    files: Vec<String>,

    /// Number all lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    
    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let file = open(&filename);

        if let Err(err) = file {
            eprintln!("Failed to open file {filename}: {err}");
            continue;
        }

        let lines = file?.lines();

        let mut line_count = 0;
        for line in lines {
            let line = line?;
            let is_empty = line.trim().is_empty();

            if args.number_nonblank_lines && !is_empty || args.number_lines {
                line_count += 1;
                print!("{line_count:>6}\t");
            }

            println!("{line}");
        }

    }
    Ok(()) 
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
