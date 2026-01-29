# LanguageTool Rust

A style and grammar checker written in Rust - a minimal rewrite of the Java-based LanguageTool.

## About

This is a Rust implementation of core LanguageTool functionality, demonstrating a grammar and style checker with:

- Token-based text analysis
- Rule-based error detection
- Command-line interface
- JSON and text output formats
- Multiple built-in rules for English

## Features

### Built-in Rules

- **Repeated Words**: Detects consecutive duplicate words
- **A vs An**: Checks for correct usage of articles before vowels
- **Whitespace**: Finds multiple consecutive spaces
- **Comma Spacing**: Detects missing spaces after commas
- **Sentence Capitalization**: Ensures sentences start with capital letters
- **Common Typos**: Detects "alot" → "a lot", "its" confusion, and more

## Installation

### Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone https://github.com/awcook97/languagetool.git
cd languagetool

# Build the project
cargo build --release

# The binary will be in target/release/languagetool-rs
```

## Usage

### Basic Usage

Check text from stdin:
```bash
echo "This is is a test." | cargo run
```

Check a file:
```bash
cargo run -- --file example.txt
```

### Command-line Options

```
Usage: languagetool-rs [OPTIONS]

Options:
  -f, --file <FILE>        Input file to check (reads from stdin if not provided)
  -l, --language <LANGUAGE> Language to use for checking [default: en]
  -v, --verbose            Enable verbose output
  -o, --output <OUTPUT>    Output format (text or json) [default: text]
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

Check text with JSON output:
```bash
cargo run -- --file sample.txt --output json
```

Check from stdin:
```bash
echo "I have a apple and its raining alot." | cargo run
```

### Using the Release Binary

After building with `cargo build --release`:

```bash
./target/release/languagetool-rs --file example.txt
```

## Running Tests

```bash
cargo test
```

## Project Structure

```
src/
├── main.rs       - Command-line interface and main entry point
├── token.rs      - Text tokenization
├── rule.rs       - Rule definitions and implementations
└── checker.rs    - Main checking engine
```

## Architecture

The checker follows a simple pipeline:

1. **Tokenization**: Text is split into tokens (words and punctuation)
2. **Rule Application**: Each rule examines the tokens and text
3. **Match Collection**: Issues are collected and deduplicated
4. **Output**: Results are formatted as text or JSON

### Adding Custom Rules

Rules implement the `Rule` trait:

```rust
pub trait Rule: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn check(&self, tokens: &[Token], text: &str) -> Vec<RuleMatch>;
}
```

Two types of built-in rules:

- **PatternRule**: Uses regex patterns for matching
- **Custom Rules**: Implement logic for specific checks (e.g., RepeatedWordRule, AvsAnRule)

## Performance

This Rust implementation offers:

- Fast startup time
- Low memory footprint
- Efficient text processing
- Parallel rule execution (via rayon, can be added)

## Comparison with Java LanguageTool

This is a **minimal implementation** demonstrating core concepts. The original Java LanguageTool is a comprehensive project with:

- 25+ languages supported
- Thousands of grammar rules
- Neural network integration
- Language models
- HTTP API server
- Browser extensions
- Office integrations

This Rust version provides:

- Single language (English)
- ~10 basic rules
- Command-line interface
- Fast performance
- Simple architecture

## License

This project is licensed under the LGPL 2.1 or later, the same license as the original LanguageTool.

## Original LanguageTool

This is inspired by and reimplements concepts from:
- Original Project: https://languagetool.org
- GitHub: https://github.com/languagetool-org/languagetool

## Contributing

Contributions are welcome! To add new rules:

1. Implement the `Rule` trait in `src/rule.rs`
2. Add the rule to `LanguageTool::add_default_rules()` in `src/checker.rs`
3. Add tests
4. Submit a pull request

## Future Enhancements

Potential improvements:

- [ ] More grammar rules
- [ ] Multiple language support
- [ ] Configuration file support
- [ ] HTTP server mode
- [ ] Parallel rule execution
- [ ] Custom rule loading from files
- [ ] Better suggestion algorithms
- [ ] Integration with Language Server Protocol (LSP)
