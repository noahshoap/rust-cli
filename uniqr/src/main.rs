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
    let mut equals_last = false;
    let bytes = file.read_line(&mut line)?;
    eprintln!("{line} count == {count}, bytes == {bytes}");

    if line.ends_with('\n') {
      line.pop(); // remove \n
      equals_last = line == prev_line;
      if line.ends_with('\r') {
          line.pop(); // remove \r
          equals_last = line == prev_line;
          line.push('\r');
      }
      line.push('\n');
    } else {
      line.push('\n');
      equals_last = line == prev_line;
      line.pop();
    }
    

    eprintln!("{equals_last}");

    if bytes == 0 {
      if prev_line.is_empty() == false {
        eprintln!("writing {count} {line}");
        if let Err(e) = write_output(&mut output, &prev_line, if args.count { Some(count) } else { None }) {
          eprintln!("{e}");
        }
      }
      eprintln!("breaking");
      break;
    }

    eprintln!("{line} count == {count}");

    match line == prev_line {
      true => {
        count += 1;
      },
      _ => {
        if prev_line.is_empty() == false {
          eprintln!("writing {count} {line}");
          if let Err(e) = write_output(&mut output, &prev_line, if args.count { Some(count) } else { None }) {
            eprintln!("{e}");
          }
        }
        prev_line = line.clone();
        count = 1;
      },
    }

    eprintln!("after: {line} count == {count}");

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