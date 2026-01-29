use languagetool_rs::{LanguageTool, RuleMatch};

fn main() {
    // Create a new LanguageTool instance for English
    let mut lt = LanguageTool::new("en");
    
    println!("LanguageTool Rust - Library API Example");
    println!("========================================\n");
    
    // Example 1: Basic text checking
    println!("Example 1: Basic checking");
    let text1 = "This is is a simple test.";
    println!("Text: \"{}\"", text1);
    let matches = lt.check(text1);
    print_matches(&matches);
    
    // Example 2: Checking for a vs an
    println!("\nExample 2: Article checking (a vs an)");
    let text2 = "I have a apple and a orange.";
    println!("Text: \"{}\"", text2);
    let matches = lt.check(text2);
    print_matches(&matches);
    
    // Example 3: Multiple issues
    println!("\nExample 3: Multiple issues");
    let text3 = "This  has  multiple spaces and and repeated words. I saw a elephant.";
    println!("Text: \"{}\"", text3);
    let matches = lt.check(text3);
    print_matches(&matches);
    
    // Example 4: Clean text
    println!("\nExample 4: Clean text");
    let text4 = "This is a perfectly written sentence.";
    println!("Text: \"{}\"", text4);
    let matches = lt.check(text4);
    if matches.is_empty() {
        println!("✓ No issues found!");
    } else {
        print_matches(&matches);
    }
    
    // Example 5: Common typos
    println!("\nExample 5: Common typos");
    let text5 = "I like this alot, but its not perfect.";
    println!("Text: \"{}\"", text5);
    let matches = lt.check(text5);
    print_matches(&matches);
}

fn print_matches(matches: &[RuleMatch]) {
    if matches.is_empty() {
        println!("  No issues found!");
    } else {
        println!("  Found {} issue(s):", matches.len());
        for m in matches {
            println!("  - [{}] {}", m.rule_id, m.message);
            if !m.suggestions.is_empty() {
                println!("    Suggestions: {}", m.suggestions.join(", "));
            }
        }
    }
}
