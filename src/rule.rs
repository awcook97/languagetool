use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::token::Token;

/// Represents a match found by a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMatch {
    /// The ID of the rule that matched
    pub rule_id: String,
    /// The message describing the issue
    pub message: String,
    /// The context (surrounding text)
    pub context: String,
    /// Start position in the text
    pub start: usize,
    /// End position in the text
    pub end: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Suggested replacements
    pub suggestions: Vec<String>,
}

/// Trait for implementing grammar/style rules
pub trait Rule: Send + Sync {
    /// Returns the unique ID of the rule
    fn id(&self) -> &str;

    /// Returns a description of what this rule checks
    fn description(&self) -> &str;

    /// Check the given tokens and return any matches
    fn check(&self, tokens: &[Token], text: &str) -> Vec<RuleMatch>;
}

/// A simple pattern-based rule that uses regex
pub struct PatternRule {
    id: String,
    description: String,
    pattern: Regex,
    message: String,
    suggestions: Vec<String>,
}

impl PatternRule {
    pub fn new(
        id: String,
        description: String,
        pattern: &str,
        message: String,
        suggestions: Vec<String>,
    ) -> Result<Self, regex::Error> {
        Ok(PatternRule {
            id,
            description,
            pattern: Regex::new(pattern)?,
            message,
            suggestions,
        })
    }
}

impl Rule for PatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn check(&self, _tokens: &[Token], text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();

        for mat in self.pattern.find_iter(text) {
            // Calculate line and column (1-indexed)
            let prefix = &text[..mat.start()];
            let line = prefix.lines().count().max(1);
            let last_line_len = prefix.lines().last().map(|l| l.len()).unwrap_or(0);
            let column = if prefix.contains('\n') {
                last_line_len + 1
            } else {
                mat.start() + 1
            };

            // Get context (30 chars before and after)
            let context_start = mat.start().saturating_sub(30);
            let context_end = (mat.end() + 30).min(text.len());
            let context = text[context_start..context_end].to_string();

            matches.push(RuleMatch {
                rule_id: self.id.clone(),
                message: self.message.clone(),
                context,
                start: mat.start(),
                end: mat.end(),
                line,
                column,
                suggestions: self.suggestions.clone(),
            });
        }

        matches
    }
}

/// A rule that checks for repeated words
pub struct RepeatedWordRule {
    id: String,
    description: String,
}

impl RepeatedWordRule {
    pub fn new() -> Self {
        RepeatedWordRule {
            id: "REPEATED_WORD".to_string(),
            description: "Detects repeated words".to_string(),
        }
    }
}

impl Rule for RepeatedWordRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn check(&self, tokens: &[Token], text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();

        for i in 0..tokens.len().saturating_sub(1) {
            let current = &tokens[i];
            let next = &tokens[i + 1];

            // Check if both are words (not punctuation)
            if current.text.chars().all(|c| c.is_alphabetic())
                && next.text.chars().all(|c| c.is_alphabetic())
            {
                // Compare case-insensitively
                if current.text.to_lowercase() == next.text.to_lowercase() {
                    let context_start = current.start.saturating_sub(20);
                    let context_end = (next.end + 20).min(text.len());
                    let context = text[context_start..context_end].to_string();

                    matches.push(RuleMatch {
                        rule_id: self.id.clone(),
                        message: format!("Repeated word: '{}'", current.text),
                        context,
                        start: current.start,
                        end: next.end,
                        line: current.line,
                        column: current.column,
                        suggestions: vec![current.text.clone()],
                    });
                }
            }
        }

        matches
    }
}

/// A rule that checks for incorrect usage of "a" vs "an"
pub struct AvsAnRule {
    id: String,
    description: String,
}

impl AvsAnRule {
    pub fn new() -> Self {
        AvsAnRule {
            id: "A_VS_AN".to_string(),
            description: "Checks for incorrect usage of 'a' vs 'an'".to_string(),
        }
    }

    fn starts_with_vowel_sound(word: &str) -> bool {
        let word_lower = word.to_lowercase();
        // Simple heuristic: starts with vowel letter
        // (more sophisticated rules would handle 'hour', 'university', etc.)
        matches!(
            word_lower.chars().next(),
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u')
        )
    }
}

impl Rule for AvsAnRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn check(&self, tokens: &[Token], text: &str) -> Vec<RuleMatch> {
        let mut matches = Vec::new();

        for i in 0..tokens.len().saturating_sub(1) {
            let current = &tokens[i];
            let next = &tokens[i + 1];

            let current_lower = current.text.to_lowercase();

            if (current_lower == "a" || current_lower == "an")
                && next.text.chars().all(|c| c.is_alphabetic())
            {
                let starts_with_vowel = Self::starts_with_vowel_sound(&next.text);

                let should_be_an = starts_with_vowel;
                let is_an = current_lower == "an";

                if should_be_an != is_an {
                    let context_start = current.start.saturating_sub(20);
                    let context_end = (next.end + 20).min(text.len());
                    let context = text[context_start..context_end].to_string();

                    let correct_article = if should_be_an { "an" } else { "a" };
                    let suggestion = if current.text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        let mut chars = correct_article.chars();
                        match chars.next() {
                            None => correct_article.to_string(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    } else {
                        correct_article.to_string()
                    };

                    matches.push(RuleMatch {
                        rule_id: self.id.clone(),
                        message: format!(
                            "Use '{}' instead of '{}' before '{}'",
                            correct_article, current.text, next.text
                        ),
                        context,
                        start: current.start,
                        end: current.end,
                        line: current.line,
                        column: current.column,
                        suggestions: vec![suggestion],
                    });
                }
            }
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_word_rule() {
        let rule = RepeatedWordRule::new();
        let tokens = vec![
            Token::new("the".to_string(), 0, 3, 1, 1),
            Token::new("the".to_string(), 4, 7, 1, 5),
        ];
        let text = "the the";
        let matches = rule.check(&tokens, text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].rule_id, "REPEATED_WORD");
    }

    #[test]
    fn test_a_vs_an_rule() {
        let rule = AvsAnRule::new();
        let tokens = vec![
            Token::new("a".to_string(), 0, 1, 1, 1),
            Token::new("apple".to_string(), 2, 7, 1, 3),
        ];
        let text = "a apple";
        let matches = rule.check(&tokens, text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].rule_id, "A_VS_AN");
    }
}
