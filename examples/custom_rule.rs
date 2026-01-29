use languagetool_rs::{LanguageTool, Rule, RuleMatch, Token};

/// A custom rule that detects passive voice (very simplified)
struct PassiveVoiceRule {
    id: String,
    description: String,
}

impl PassiveVoiceRule {
    fn new() -> Self {
        PassiveVoiceRule {
            id: "PASSIVE_VOICE".to_string(),
            description: "Detects simple passive voice constructions".to_string(),
        }
    }
}

impl Rule for PassiveVoiceRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn check(&self, tokens: &[Token], text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();
        
        // Simple heuristic: look for "was/were/is/are + past participle"
        // We'll just look for "was/were/is/are + word ending in 'ed'"
        for i in 0..tokens.len().saturating_sub(1) {
            let current = &tokens[i];
            let next = &tokens[i + 1];
            
            let current_lower = current.text.to_lowercase();
            if ["was", "were", "is", "are", "been"].contains(&current_lower.as_str()) {
                if next.text.ends_with("ed") || next.text.ends_with("en") {
                    let context_start = current.start.saturating_sub(20);
                    let context_end = (next.end + 20).min(text.len());
                    let context = text[context_start..context_end].to_string();
                    
                    matches.push(RuleMatch {
                        rule_id: self.id.clone(),
                        message: format!(
                            "Possible passive voice: '{} {}'",
                            current.text, next.text
                        ),
                        context,
                        start: current.start,
                        end: next.end,
                        line: current.line,
                        column: current.column,
                        suggestions: vec!["Consider using active voice".to_string()],
                    });
                }
            }
        }
        
        matches
    }
}

fn main() {
    println!("Custom Rule Example");
    println!("==================\n");
    
    // Create a LanguageTool instance
    let mut lt = LanguageTool::new("en");
    
    // Add our custom rule
    lt.add_rule(Box::new(PassiveVoiceRule::new()));
    
    // Test texts
    let test_cases = vec![
        "The ball was thrown by John.",
        "Mary ate the apple.",
        "The document was written yesterday.",
        "The cat is sleeping on the mat.",
        "The report has been completed.",
    ];
    
    for (i, text) in test_cases.iter().enumerate() {
        println!("Test case {}: \"{}\"", i + 1, text);
        let matches = lt.check(text);
        
        if matches.is_empty() {
            println!("  ✓ No issues detected\n");
        } else {
            println!("  Issues found:");
            for m in &matches {
                println!("  - [{}] {}", m.rule_id, m.message);
            }
            println!();
        }
    }
    
    println!("\nNote: This is a simplified passive voice detector.");
    println!("A production implementation would use more sophisticated NLP techniques.");
}
