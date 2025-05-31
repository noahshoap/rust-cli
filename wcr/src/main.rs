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

  /// Show line count
  #[arg(short, long)]
  lines: bool,

  /// Show word count
  #[arg(short, long)]
  words: bool,

  /// Show byte count
  #[arg(short('c'), long)]
  bytes: bool,

  /// Show character count
  #[arg(short('m'), long, conflicts_with("bytes"))]
  chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
  num_lines: usize,
  num_words: usize,
  num_bytes: usize,
  num_chars: usize,
}

impl FileInfo {
  fn add(&mut self, i2: &FileInfo) {
    self.num_lines += i2.num_lines;
    self.num_words += i2.num_words;
    self.num_bytes += i2.num_bytes;
    self.num_chars += i2.num_chars;
  }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
  match filename {
      "-" => Ok(Box::new(BufReader::new(io::stdin()))),
      _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

fn check_args(args: &mut Args) {
  if [args.words, args.bytes, args.chars, args.lines]
    .iter()
    .all(|v| v == &false)
  {
    args.lines = true;
    args.words = true;
    args.bytes = true;
  }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
  let mut num_lines = 0;
  let mut num_words = 0;
  let mut num_bytes = 0;
  let mut num_chars = 0;
  let mut line = String::new();

  while let Ok(bytes_read) = file.read_line(&mut line) {
    if bytes_read == 0 {
        break; // EOF
    }

    num_lines += 1;
    num_bytes += bytes_read;
    num_words += line.split_whitespace().count();
    num_chars += line.chars().count();

    line.clear();
}

  Ok(FileInfo {
    num_lines,
    num_words,
    num_bytes,
    num_chars,
  })
}

fn print_file_info(file_info: &FileInfo, args: &Args) {
  let cols = [args.lines, args.words, args.bytes, args.chars];
  let info = [file_info.num_lines, file_info.num_words, file_info.num_bytes, file_info.num_chars];

  for (index, col) in cols.iter().enumerate() {
    if *col {
      print!("{:>8}", info[index]);
    }
  }
}

fn run(mut args: Args) -> Result<()> {
  check_args(&mut args);
  let mut total_info = FileInfo{num_lines: 0, num_words: 0, num_bytes: 0, num_chars: 0};

  for filename in &args.files {
    match open(filename) {
      Err(err) => eprintln!("{filename}: {err}"),
      Ok(file) => {
        match count(file) {
          Err(err) => eprintln!("{filename}: {err}"),
          Ok(file_info) => {
            print_file_info(&file_info, &args);

            total_info.add(&file_info);

            println!("{}", if filename != "-" { format!(" {filename}") } else { String::new() });
            // println!("{:>8}{:>8}{:>8} {filename}", file_info.num_lines, file_info.num_words, file_info.num_bytes);
          }
        }
      }
    }
  }

  if args.files.len() > 1 {
    print_file_info(&total_info, &args);
    println!(" total");
  }

  Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
      eprintln!("{e}");
      std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
  use super::{count, FileInfo};
  use std::io::Cursor;

  #[test]
  fn test_count() {
    let text = "I don't want the world.\nI just want your half.\r\n";
    let info = count(Cursor::new(text));

    assert!(info.is_ok());

    let expected = FileInfo {
      num_lines: 2,
      num_words: 10,
      num_bytes: 48,
      num_chars: 48,
    };
    
    assert_eq!(info.unwrap(), expected);
  }
}