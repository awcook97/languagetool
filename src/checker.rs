use crate::rule::{AvsAnRule, PatternRule, RepeatedWordRule, Rule, RuleMatch};
use crate::token::tokenize;

/// Main LanguageTool checker
pub struct LanguageTool {
    language: String,
    rules: Vec<Box<dyn Rule>>,
}

impl LanguageTool {
    /// Create a new LanguageTool instance for the specified language
    pub fn new(language: &str) -> Self {
        let mut lt = LanguageTool {
            language: language.to_string(),
            rules: Vec::new(),
        };

        // Add built-in rules
        lt.add_default_rules();

        lt
    }

    /// Add default rules for English
    fn add_default_rules(&mut self) {
        // Repeated word rule
        self.rules.push(Box::new(RepeatedWordRule::new()));

        // A vs An rule
        self.rules.push(Box::new(AvsAnRule::new()));

        // Whitespace rules
        if let Ok(rule) = PatternRule::new(
            "WHITESPACE_RULE".to_string(),
            "Checks for multiple consecutive spaces".to_string(),
            r"  +",
            "Multiple consecutive spaces found".to_string(),
            vec![" ".to_string()],
        ) {
            self.rules.push(Box::new(rule));
        }

        // Punctuation spacing rules
        if let Ok(rule) = PatternRule::new(
            "COMMA_SPACING".to_string(),
            "Checks for missing space after comma".to_string(),
            r",[^ \n]",
            "Missing space after comma".to_string(),
            vec![],
        ) {
            self.rules.push(Box::new(rule));
        }

        // Sentence capitalization
        if let Ok(rule) = PatternRule::new(
            "SENTENCE_CAPITALIZATION".to_string(),
            "Checks that sentences start with capital letter".to_string(),
            r"(?:^|[.!?]\s+)([a-z])",
            "Sentence should start with a capital letter".to_string(),
            vec![],
        ) {
            self.rules.push(Box::new(rule));
        }

        // Common typos
        if let Ok(rule) = PatternRule::new(
            "TYPO_ALOT".to_string(),
            "Detects 'alot' which should be 'a lot'".to_string(),
            r"\balot\b",
            "Did you mean 'a lot'?".to_string(),
            vec!["a lot".to_string()],
        ) {
            self.rules.push(Box::new(rule));
        }

        if let Ok(rule) = PatternRule::new(
            "TYPO_ITS_VS_ITS".to_string(),
            "Detects potential confusion between it's and its".to_string(),
            r"\bits\s+[a-z]+ing\b",
            "Did you mean \"it's\" (it is)?".to_string(),
            vec!["it's".to_string()],
        ) {
            self.rules.push(Box::new(rule));
        }
    }

    /// Add a custom rule
    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    /// Check the given text and return all matches
    pub fn check(&mut self, text: &str) -> Vec<RuleMatch> {
        let tokens = tokenize(text);
        let mut all_matches = Vec::new();

        for rule in &self.rules {
            let matches = rule.check(&tokens, text);
            all_matches.extend(matches);
        }

        // Sort matches by position
        all_matches.sort_by_key(|m| m.start);

        // Remove duplicates (same position)
        all_matches.dedup_by_key(|m| (m.start, m.end));

        all_matches
    }

    /// Get the language being used
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Get the number of active rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_languagetool_basic() {
        let mut lt = LanguageTool::new("en");
        let text = "This is a test.";
        let matches = lt.check(text);
        // Should not find issues in correct text
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_languagetool_repeated_word() {
        let mut lt = LanguageTool::new("en");
        let text = "This is is a test.";
        let matches = lt.check(text);
        assert!(matches.len() > 0);
        assert!(matches.iter().any(|m| m.rule_id == "REPEATED_WORD"));
    }

    #[test]
    fn test_languagetool_a_vs_an() {
        let mut lt = LanguageTool::new("en");
        let text = "I have a apple.";
        let matches = lt.check(text);
        assert!(matches.len() > 0);
        assert!(matches.iter().any(|m| m.rule_id == "A_VS_AN"));
    }

    #[test]
    fn test_languagetool_multiple_spaces() {
        let mut lt = LanguageTool::new("en");
        let text = "This  has   multiple spaces.";
        let matches = lt.check(text);
        assert!(matches.len() > 0);
        assert!(matches
            .iter()
            .any(|m| m.rule_id == "WHITESPACE_RULE"));
    }

    #[test]
    fn test_languagetool_typo() {
        let mut lt = LanguageTool::new("en");
        let text = "I like this alot.";
        let matches = lt.check(text);
        assert!(matches.len() > 0);
        assert!(matches.iter().any(|m| m.rule_id == "TYPO_ALOT"));
    }
}
