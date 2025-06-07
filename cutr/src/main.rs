use clap::{Parser};
use anyhow::{Result, bail};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
  /// Input file(s)
  #[arg(value_name="FILES", default_value=".")]
  files: Vec<String>,

  /// Name
  #[arg(short, long, default_value="\t")]
  delimiter: String,

  /// Entry type
  #[command(flatten)]
  extract: ArgsExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
  /// Selected field
  #[arg(short, long, value_name = "FIELDS")]
  fields: Option<String>,

  /// Selected bytes
  #[arg(short, long, value_name = "BYTES")]
  bytes: Option<String>,

  /// Selected chars
  #[arg(short, long, value_name = "CHARS")]
  chars: Option<String>,
}

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
  Fields(PositionList),
  Bytes(PositionList),
  Chars(PositionList),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
      eprintln!("{e}");
      std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
  let delimiter = get_and_validate_delimiter(args.delimiter)?;

  println!("{delimiter}");

  Ok(())
}

fn get_and_validate_delimiter(delimiter: String) -> Result<u8> {
  let delim_bytes = delimiter.as_bytes();

  if delim_bytes.len() != 1 {
    bail!(r#"--delim "{}" must be a single byte"#, delimiter);
  }

  Ok(*delim_bytes.first().unwrap())
}