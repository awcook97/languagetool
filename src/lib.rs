//! # LanguageTool Rust
//!
//! A style and grammar checker library written in Rust.
//!
//! ## Example
//!
//! ```rust
//! use languagetool_rs::LanguageTool;
//!
//! let mut lt = LanguageTool::new("en");
//! let text = "This is is a test.";
//! let matches = lt.check(text);
//!
//! for m in matches {
//!     println!("{}: {}", m.rule_id, m.message);
//! }
//! ```

pub mod checker;
pub mod rule;
pub mod token;

pub use checker::LanguageTool;
pub use rule::{Rule, RuleMatch};
pub use token::{Token, tokenize};
