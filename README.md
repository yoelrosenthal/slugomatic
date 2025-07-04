# 🐌 Slugomatic

[![Crates.io](https://img.shields.io/crates/v/slugomatic.svg)](https://crates.io/crates/slugomatic)

A simple and fast CLI tool to slugify and unslugify text, perfect for creating branch names, URLs, and file names.

## Features

- **Slugify text**: Convert text to URL-friendly slugs (default behavior)
- **Unslugify text**: Convert slugs back to readable text
- **Clipboard integration**: Automatically copies results to clipboard
- **Fast and lightweight**: Built with Rust for maximum performance
- **Cross-platform**: Works on Windows, macOS, and Linux

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
# Output: hello-world

# You may also echo text and pipe it (works if TEXT is omitted)
echo "My Feature Branch" | slug
# Output: my-feature-branch
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

### Examples

```bash
# Perfect for Git branch names
slug "Fix user authentication bug"
# Output: fix-user-authentication-bug

# Convert back to readable text
slug -u "fix-user-authentication-bug"
# Output: fix user authentication bug

# Works with special characters
slug "Add new feature (v2.0)!"
# Output: add-new-feature-v20

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
- `-h, --help`: Show detailed help information, usage, and examples
- `-V, --version`: Show version information

## CLI/Usage Highlights

- **Text argument vs stdin**: If you provide a `TEXT`, that is processed. If `TEXT` is omitted, input will be read from standard input (stdin).
- **Clipboard**: Results are copied to clipboard by default if available; use `--no-clipboard` to control this.
- **Help**: Try `slug --help` for full usage details, including all options and examples.

## How it works

- **Slugify**: Removes special characters, converts to lowercase, and replaces spaces with hyphens
- **Unslugify**: Simply replaces hyphens with spaces
- **Clipboard**: Results are automatically copied to your clipboard for easy pasting (unless disabled)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
