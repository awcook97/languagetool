#!/bin/bash
# Demo script showing LanguageTool Rust capabilities

set -e

echo "================================================"
echo "LanguageTool Rust - Demonstration"
echo "================================================"
echo ""

# Build the project
echo "1. Building the project..."
cargo build --release --quiet
echo "   ✓ Build complete"
echo ""

# Test 1: Basic usage
echo "2. Testing basic grammar checking..."
echo "   Input: 'This is is a test.'"
echo ""
echo "   Output:"
echo "This is is a test." | ./target/release/languagetool-rs
echo ""

# Test 2: Article checking
echo "3. Testing article correction (a vs an)..."
echo "   Input: 'I have a apple and a orange.'"
echo ""
echo "   Output:"
echo "I have a apple and a orange." | ./target/release/languagetool-rs
echo ""

# Test 3: File checking
echo "4. Testing file checking..."
echo "   Checking example.txt..."
echo ""
./target/release/languagetool-rs --file example.txt | head -20
echo "   ... (output truncated)"
echo ""

# Test 4: JSON output
echo "5. Testing JSON output format..."
echo "   Input: 'I need need to fix this.'"
echo ""
echo "   Output:"
echo "I need need to fix this." | ./target/release/languagetool-rs --output json | head -15
echo "   ... (output truncated)"
echo ""

# Test 5: Clean text
echo "6. Testing with clean text..."
echo "   Input: 'This is a perfect sentence.'"
echo ""
echo "   Output:"
echo "This is a perfect sentence." | ./target/release/languagetool-rs
echo ""

# Test 6: Library API example
echo "7. Running library API example..."
cargo run --example basic_usage --quiet 2>/dev/null | head -30
echo "   ... (output truncated)"
echo ""

# Test 7: Custom rule example
echo "8. Running custom rule example..."
cargo run --example custom_rule --quiet 2>/dev/null | head -20
echo "   ... (output truncated)"
echo ""

# Test 8: Running all tests
echo "9. Running test suite..."
cargo test --quiet 2>&1 | tail -5
echo ""

echo "================================================"
echo "Demonstration complete!"
echo "================================================"
echo ""
echo "Key Features:"
echo "  ✓ 7 built-in grammar rules"
echo "  ✓ Command-line interface"
echo "  ✓ Library API"
echo "  ✓ JSON and text output"
echo "  ✓ Custom rule support"
echo "  ✓ Fast performance"
echo "  ✓ Comprehensive tests"
echo ""
echo "For more information, see:"
echo "  - RUST_README.md"
echo "  - QUICKSTART.md"
echo "  - examples/"
