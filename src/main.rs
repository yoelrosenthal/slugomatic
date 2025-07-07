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
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A simple and fast CLI tool to slugify and unslugify text (branch names, URLs, etc)\n\n\
              Examples:\n  slug \"Hello World\"\n  slug -u \"my-feature-branch\"\n  echo \"from stdin\" | slug --no-clipboard",
    after_help = "If no TEXT is given, reads from stdin.\n\
                  By default, slugified/unslugified result is copied to clipboard unless --no-clipboard is set.",
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
}

fn main() {
    let args = Args::parse();

    // Prefer argument, then fall back to stdin
    let input = match &args.text {
        Some(s) => s.clone(),
        None => {
            // Read from stdin
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

    let result = if args.unslug {
        unslugify(&input)
    } else {
        slugify(&input)
    };

    println!("{result}");

    // Optionally copy to clipboard
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
