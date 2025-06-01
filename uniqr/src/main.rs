use anyhow::{Result, anyhow};
use clap::Parser;
use std::{fs::File, io::{self, BufRead, BufReader, BufWriter, Write},};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
  /// Input file
  #[arg(value_name="IN_FILE", default_value="-")]
  in_file: String,

  /// Output file
  #[arg(value_name="OUT_FILE")]
  out_file: Option<String>,

  /// Show counts
  #[arg(short, long)]
  count: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
      eprintln!("{e}");
      std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
  let mut file = open_input(&args.in_file)
    .map_err(|e| anyhow!("{}: {e}", args.in_file))?;

  let mut output = open_output(&args.out_file)
    .map_err(|e| anyhow!("{}: {e}", if let Some(x) = args.out_file { x } else { "stdout".to_string() }))?;

  let mut line = String::new();
  let mut prev_line = String::new();
  let mut count = 0;

  loop {
    let bytes = file.read_line(&mut line)?;

    if bytes == 0 {
      if prev_line.is_empty() == false {
        if let Err(e) = write_output(&mut output, &prev_line, if args.count { Some(count) } else { None }) {
          eprintln!("{e}");
        }
      }
      break;
    }

    match line.trim_end() == prev_line.trim_end() {
      true => {
        count += 1;
      },
      _ => {
        if prev_line.is_empty() == false {
          if let Err(e) = write_output(&mut output, &prev_line, if args.count { Some(count) } else { None }) {
            eprintln!("{e}");
          }
        }
        prev_line = line.clone();
        count = 1;
      },
    }

    line.clear();
  }
  
  Ok(())
}

fn write_output(mut output: impl Write, line: &String, count: Option<i32>) -> Result<()> {
  let line_count_output = match count {
    None => "".to_string(),
    Some(c) => format!("{c:>4} "),
  };

  let output_string = format!("{line_count_output}{line}");

  output.write(output_string.as_bytes())?;

  Ok(())
}

fn open_input(filename: &str) -> Result<Box<dyn BufRead>> {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

fn open_output(filename: &Option<String>) -> Result<Box<dyn Write>> {
  match filename {
      None => Ok(Box::new(BufWriter::new(io::stdout()))),
      Some(x) => Ok(Box::new(BufWriter::new(File::create(x)?))),
  }
}