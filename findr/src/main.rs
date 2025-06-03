use clap::{Parser, builder::PossibleValue, ValueEnum, ArgAction};
use regex::Regex;
use anyhow::Result;
use walkdir::{WalkDir, DirEntry};

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
  Dir,
  File,
  Link
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
  /// Search path(s)
  #[arg(value_name="PATH", default_value=".")]
  paths: Vec<String>,

  /// Name
  #[arg(short, long("name"), value_name="NAME", value_parser(Regex::new), action(ArgAction::Append), num_args(0..))]
  names: Vec<Regex>,

  /// Entry type
  #[arg(short('t'), long("type"), value_name="TYPE", value_parser=clap::value_parser!(EntryType), action(ArgAction::Append), num_args(0..))]
  entry_types: Vec<EntryType>,
}

impl ValueEnum for EntryType {
  fn value_variants<'a>() -> &'a [Self] {
    &[EntryType::Dir, EntryType::File, EntryType::Link]
  }

  fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
    Some(match self {
      EntryType::Dir => PossibleValue::new("d"),
      EntryType::File => PossibleValue::new("f"),
      EntryType::Link => PossibleValue::new("l"),
    })
  }
}
fn main() {
    if let Err(e) = run(Args::parse()) {
      eprintln!("{e}");
      std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
  let type_filter = |entry: &DirEntry| {
    args.entry_types.is_empty() || 
    args.entry_types
      .iter()
      .any(|entry_type| match entry_type {
        EntryType::Link => entry.file_type().is_symlink(),
        EntryType::Dir => entry.file_type().is_dir(),
        EntryType::File => entry.file_type().is_file(),
      })
  };

  let name_filter = |entry: &DirEntry| {
    args.names.is_empty() ||
    args.names
      .iter()
      .any(|regex| regex.is_match(&entry.file_name().to_string_lossy()))
  };

  for path in args.paths {
    let entries = WalkDir::new(path)
      .into_iter()
      .filter_map(|e| match e {
        Err(e) => {
          eprintln!("{e}");
          None
        }
        Ok(entry) => Some(entry),
      })
      .filter(type_filter)
      .filter(name_filter)
      .map(|entry| entry.path().display().to_string())
      .collect::<Vec<_>>();

      println!("{}", entries.join("\n"));
  }
  Ok(())
}