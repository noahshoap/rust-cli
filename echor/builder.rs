use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Noah Shoap <noahshoap@gmail.com>")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Omit printing newline"),
        )
        .get_matches();

    let omit_newline = matches.get_flag("omit_newline");
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
