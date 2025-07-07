# 🐌 Slugomatic

[![Crates.io](https://img.shields.io/crates/v/slugomatic.svg)](https://crates.io/crates/slugomatic)

A simple and fast CLI tool to slugify and unslugify text, perfect for creating branch names, URLs, and file names.

## Features

- **Slugify text**: Convert text to URL-friendly slugs (default behavior)
- **Unslugify text**: Convert slugs back to readable text
- **Clipboard integration**: Automatically copies results to clipboard
- **Fast and lightweight**: Built with Rust for maximum performance
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Case conversion**: Output can be converted to all lowercase, all uppercase, or title case using `--lowercase`, `--uppercase`, or `--title`

## Installation

### From crates.io

```bash
cargo install slugomatic
```

### From source

```bash
git clone https://github.com/yoelrosenthal/slugomatic
cd slugomatic
cargo install --path .
```

## Usage

### Slugify text (default behavior)

```bash
# Convert text to slug
slug "Hello World"
# Output: Hello-World

# You may also echo text and pipe it (works if TEXT is omitted)
echo "My Feature Branch" | slug
# Output: My-Feature-Branch

# Most git branch names should be lowercase; see --lowercase below!
# For human-readable output you can use --title!
```

### Unslugify text

```bash
# Convert slug back to text
slug -u "hello-world"
# Output: hello world

slug --unslug "my-feature-branch"
# Output: my feature branch

echo "fix-user-authentication-bug" | slug -u
# Output: fix user authentication bug
```

### No clipboard copy

```bash
# Disable automatic clipboard copy
slug --no-clipboard "Just show this"
```

### Case conversion

```bash
# Default: preserves input capitalization
slug "Some Mixed Case Words"
# Output: Some-Mixed-Case-Words

# Force output to lowercase
slug "Some Text Here" --lowercase
# Output: some-text-here

slug -u "My-Feature-Branch" --lowercase
# Output: my feature branch

# Force output to uppercase
slug "Some Text Here" --uppercase
# Output: SOME-TEXT-HERE

slug -u "my-feature-branch" --uppercase
# Output: MY FEATURE BRANCH

# Title Case output:
slug "some mixed cAsE Words" --title
# Output: Some Mixed Case Words
```

### Examples

```bash
# Perfect for Git branch names (commonly lowercase):
slug "Fix user authentication bug" --lowercase
# Output: fix-user-authentication-bug

# Title case for readability
slug -u "fix-user-authentication-bug" --title
# Output: Fix User Authentication Bug

# Works with special characters
slug "Add new feature (v2.0)!"
# Output: Add-new-feature-v20
```

### Slugify via stdin

# These both work the same:
slug "A new feature branch"
echo "A new feature branch" | slug

### Prevent copying to clipboard

slug "Text" --no-clipboard
echo "Another example" | slug --no-clipboard
```

## Options

- `-s, --slug`: Explicitly slugify the input (default behavior)
- `-u, --unslug`: Unslugify the input (convert dashes to spaces)
- `--no-clipboard`: Do not copy the result to the clipboard (default is to copy)
- `TEXT`: The text to process; if omitted, reads from stdin
- `--lowercase`: Convert the result to all lowercase (mutually exclusive with `--uppercase` and `--title`)
- `--uppercase`: Convert the result to all uppercase (mutually exclusive with `--lowercase` and `--title`)
- `--title`: Convert the result to title case (each word starts uppercase, mutually exclusive with `--lowercase` and `--uppercase`)
- `-h, --help`: Show detailed help information, usage, and examples
- `-V, --version`: Show version information

## CLI/Usage Highlights

- **Text argument vs stdin**: If you provide a `TEXT`, that is processed. If `TEXT` is omitted, input will be read from standard input (stdin).
- **Clipboard**: Results are copied to clipboard by default if available; use `--no-clipboard` to control this.
- **Case conversion**: By default, slug output preserves case. Use `--lowercase`, `--uppercase`, or `--title` to convert. These options cannot be combined.
- **Help**: Try `slug --help` for full usage details, including all options and examples.

## How it works

- **Slugify**: Removes special characters and replaces spaces with hyphens. Output case is preserved unless `--lowercase`, `--uppercase`, or `--title` is used.
- **Unslugify**: Turns hyphens into spaces. Use `--lowercase`, `--uppercase`, or `--title` to standardize output if desired.
- **Clipboard**: Results are automatically copied to your clipboard (unless disabled)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
