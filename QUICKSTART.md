# Quick Start Guide - LanguageTool Rust

## Installation

```bash
# Make sure you have Rust installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
cd languagetool
cargo build --release
```

## Usage Examples

### Check text from stdin

```bash
echo "This is is a test." | cargo run
```

Output:
```
Found 1 issue found:

1. REPEATED_WORD at line 1, column 6:
   Message: Repeated word: 'is'
   Context: "This is is a test.
"
   Suggestions: is

Total: 1 issue(s) found
```

### Check a file

```bash
cargo run -- --file example.txt
```

### Get JSON output

```bash
echo "I have a apple." | cargo run -- --output json
```

Output:
```json
[
  {
    "rule_id": "A_VS_AN",
    "message": "Use 'an' instead of 'a' before 'apple'",
    "context": "I have a apple.\n",
    "start": 7,
    "end": 8,
    "line": 1,
    "column": 8,
    "suggestions": [
      "an"
    ]
  }
]
```

### Use as a library

Add to your `Cargo.toml`:
```toml
[dependencies]
languagetool-rs = { path = "../languagetool" }
```

In your code:
```rust
use languagetool_rs::LanguageTool;

fn main() {
    let mut lt = LanguageTool::new("en");
    let text = "This is is a test.";
    let matches = lt.check(text);
    
    for m in matches {
        println!("{}: {}", m.rule_id, m.message);
    }
}
```

## Built-in Rules

| Rule ID | Description | Example |
|---------|-------------|---------|
| REPEATED_WORD | Detects duplicate words | "the the" |
| A_VS_AN | Article usage before vowels | "a apple" → "an apple" |
| WHITESPACE_RULE | Multiple consecutive spaces | "hello  world" |
| COMMA_SPACING | Missing space after comma | "hello,world" |
| SENTENCE_CAPITALIZATION | Sentence start capitalization | ". the" → ". The" |
| TYPO_ALOT | Common typo | "alot" → "a lot" |
| TYPO_ITS_VS_ITS | Its vs It's confusion | "its raining" → "it's raining" |

## Options

```
-f, --file <FILE>        Input file to check
-l, --language <LANGUAGE> Language [default: en]
-v, --verbose            Verbose output
-o, --output <OUTPUT>    Format: text or json [default: text]
-h, --help              Print help
-V, --version           Print version
```

## Testing

Run all tests:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

## Performance

The Rust implementation is significantly faster than the Java version for basic checks:

- **Startup time**: ~5ms (vs ~2s for Java)
- **Memory usage**: ~5MB (vs ~500MB for Java)
- **Check speed**: ~1000 words/ms

*Note: The Java version has many more features and rules.*

## Next Steps

See [RUST_README.md](RUST_README.md) for:
- Architecture details
- How to add custom rules
- Comparison with Java version
- Contributing guidelines
