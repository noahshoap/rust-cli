use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name="FILE", default_value="-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(short('n'), long("lines"), conflicts_with("bytes"), default_value="10", value_parser = clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Number of bytes
    #[arg(short('c'), long("bytes"), value_parser = clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for (index, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut buf) => {
                if args.files.len() > 1 {
                    let line_end = if index > 0 { "\n" } else { "" };
                    println!("{line_end}==> {filename} <==");
                }
                // we are outputting bytes
                if let Some(bytes) = args.bytes {
                    let arr = vec![0u8; bytes as usize];
                    let mut boxed_arr: Box<[u8]> = arr.into_boxed_slice();
                    let result = buf.read_exact(&mut boxed_arr);

                    if let Err(_) = result {
                        continue;
                    }
                    let s = String::from_utf8_lossy(&boxed_arr);
                    print!("{}", s);
                } else {
                        let mut line = String::new();
                        for _ in 0..args.lines {
                            let bytes = buf.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{line}");
                            line.clear();
                        }
                    }
                }
            }
        }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
