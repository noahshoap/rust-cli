use clap::{Parser, builder::PossibleValue, ValueEnum, ArgAction};
use regex::Regex;
use anyhow::Result;
use walkdir::WalkDir;
use std::fs::FileType;

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
  for path in args.paths {
    for entry in WalkDir::new(path) {
      match entry {
        Err(e) => eprintln!("{e}"),
        Ok(entry) => {
          let types = &args.entry_types;
          let names = &args.names;
          let file_type = entry.file_type();
          let file_name = entry.file_name().to_string_lossy();

          if matches_name_filter(&names, file_name.to_string()) && matches_type_filter(&types, file_type) {
            println!("{}", entry.path().display());
          }
        }
      }
    }
  }
  Ok(())
}

// The author of the book wrote the following two methods as one large conditional in the above method.
// If this is considered good Rust, then I don't want to write good Rust.

fn matches_name_filter(names: &Vec<Regex>, file_name: String) -> bool {
  if names.is_empty() {
    return true;
  }

  return names.iter().any(|regex| {
    regex.is_match(&file_name)
  })
}

fn matches_type_filter(types: &Vec<EntryType>, file_type: FileType) -> bool {
  if types.is_empty() {
    return true;
  }

  return types.iter().any(|filterType| {
    match filterType {
      EntryType::Link => file_type.is_symlink(),
      EntryType::Dir => file_type.is_dir(),
      EntryType::File => file_type.is_file(),
    }
  });
}