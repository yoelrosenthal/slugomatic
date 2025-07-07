use arboard::Clipboard;
use clap::Parser;
use regex::Regex;
use std::io::{self, Read};
use std::process;

/// 🐌 Slugomatic — Simple and fast slug CLI

///

/// Examples:

///   slug "Hello World"

///   slug -u "my-feature-branch"

///   echo "Piped input" | slug --no-clipboard
///   slug "Some Text" --lowercase
///   slug "Some Text" --uppercase

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A simple and fast CLI tool to slugify and unslugify text (branch names, URLs, etc)\n\n\
              Examples:\n  slug \"Hello World\"\n  slug -u \"my-feature-branch\"\n  slug \"Text Here\" --lowercase\n  slug \"Text Here\" --uppercase\n  slug \"Text Here\" --title\n  echo \"from stdin\" | slug --no-clipboard",
    after_help = "If no TEXT is given, reads from stdin.\n\
                  By default, slugified/unslugified result is copied to clipboard unless --no-clipboard is set.\n\
                  \n\
                  Output case is preserved unless you specify --lowercase, --uppercase, or --title (all are mutually exclusive). Use these flags to control the result form commonly needed for git branches, URLs, or for readability.",
    next_line_help = true
)]

struct Args {
    /// Slugify the input (default)
    #[arg(short, long, group = "mode")]
    slug: bool,

    /// Unslugify the input
    #[arg(short, long, group = "mode")]
    unslug: bool,

    /// Do not copy result to clipboard
    #[arg(long, default_value_t = false)]
    no_clipboard: bool,

    /// Text to process. If not provided, reads from stdin.
    #[arg(value_name = "TEXT")]
    text: Option<String>,

    /// Convert output to lowercase
    #[arg(long, conflicts_with_all = ["uppercase", "title"])]
    lowercase: bool,

    /// Convert output to uppercase
    #[arg(long, conflicts_with_all = ["lowercase", "title"])]
    uppercase: bool,

    /// Convert output to Title Case
    #[arg(long, conflicts_with_all = ["lowercase", "uppercase"])]
    title: bool,
}

fn main() {
    let args = Args::parse();

    let input = match &args.text {
        Some(s) => s.clone(),
        None => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(n) if n > 0 => buffer.trim_end_matches(&['\n', '\r'][..]).to_string(),
                Ok(_) => {
                    eprintln!("Error: No input provided (neither TEXT argument nor stdin)");
                    process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: Failed to read from stdin: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    if input.trim().is_empty() {
        eprintln!("Error: No input provided (empty string)");
        process::exit(1);
    }

    let mut result = if args.unslug {
        unslugify(&input)
    } else {
        slugify(&input)
    };

    if args.lowercase {
        result = result.to_lowercase();
    } else if args.uppercase {
        result = result.to_uppercase();
    } else if args.title {
        result = title_case(&result);
    }

    fn title_case(input: &str) -> String {
        input
            .split_whitespace()
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    println!("{result}");

    if !args.no_clipboard {
        match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(result.clone()) {
                Ok(_) => eprintln!("✓ Copied to clipboard!"),
                Err(e) => eprintln!("Warning: Failed to copy to clipboard: {e}"),
            },
            Err(e) => {
                eprintln!("Warning: Could not access clipboard: {e}");
            }
        }
    }
}

fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^\w\s-]").unwrap();
    re.replace_all(input, "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

fn unslugify(input: &str) -> String {
    input.replace('-', " ")
}
