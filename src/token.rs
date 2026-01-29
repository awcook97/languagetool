use serde::{Deserialize, Serialize};

/// Represents a token in the analyzed text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// The text of the token
    pub text: String,
    /// Position in the original text
    pub start: usize,
    /// End position in the original text
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Token {
    pub fn new(text: String, start: usize, end: usize, line: usize, column: usize) -> Self {
        Token {
            text,
            start,
            end,
            line,
            column,
        }
    }
}

/// Tokenizes text into a list of tokens
pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;
    let mut in_word = false;
    let mut word_start = 0;
    let mut word_start_column = 1;

    for (i, ch) in text.char_indices() {
        if ch == '\n' {
            if in_word {
                tokens.push(Token::new(
                    text[word_start..i].to_string(),
                    word_start,
                    i,
                    line,
                    word_start_column,
                ));
                in_word = false;
            }
            line += 1;
            column = 1;
        } else if ch.is_whitespace() || ch.is_ascii_punctuation() {
            if in_word {
                tokens.push(Token::new(
                    text[word_start..i].to_string(),
                    word_start,
                    i,
                    line,
                    word_start_column,
                ));
                in_word = false;
            }
            // Add punctuation as separate token
            if ch.is_ascii_punctuation() {
                tokens.push(Token::new(
                    ch.to_string(),
                    i,
                    i + ch.len_utf8(),
                    line,
                    column,
                ));
            }
            column += 1;
        } else {
            if !in_word {
                word_start = i;
                word_start_column = column;
                in_word = true;
            }
            column += 1;
        }
    }

    // Handle last word
    if in_word {
        tokens.push(Token::new(
            text[word_start..].to_string(),
            word_start,
            text.len(),
            line,
            word_start_column,
        ));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let text = "Hello world";
        let tokens = tokenize(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, "world");
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let text = "Hello, world!";
        let tokens = tokenize(text);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, ",");
        assert_eq!(tokens[2].text, "world");
        assert_eq!(tokens[3].text, "!");
    }

    #[test]
    fn test_tokenize_multiline() {
        let text = "Hello\nworld";
        let tokens = tokenize(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
    }
}
