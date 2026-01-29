use clap::Parser;
use colored::Colorize;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use languagetool_rs::LanguageTool;

#[derive(Parser, Debug)]
#[command(name = "languagetool-rs")]
#[command(author = "LanguageTool Contributors")]
#[command(version = "0.1.0")]
#[command(about = "A style and grammar checker written in Rust", long_about = None)]
struct Args {
    /// Input file to check (reads from stdin if not provided)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Language to use for checking
    #[arg(short, long, default_value = "en")]
    language: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input text
    let text = if let Some(file_path) = args.file {
        fs::read_to_string(file_path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Create language tool checker
    let mut lt = LanguageTool::new(&args.language);

    // Check the text
    let matches = lt.check(&text);

    // Output results
    if args.output == "json" {
        let json = serde_json::to_string_pretty(&matches).unwrap();
        println!("{}", json);
    } else {
        // Text output
        if matches.is_empty() {
            println!("{}", "No issues found!".green().bold());
        } else {
            let issue_count = if matches.len() == 1 {
                "1 issue".to_string()
            } else {
                format!("{} issues", matches.len())
            };
            
            println!(
                "{} {} found:\n",
                "Found".red().bold(),
                issue_count
            );

            for (i, m) in matches.iter().enumerate() {
                println!(
                    "{}. {} at line {}, column {}:",
                    i + 1,
                    m.rule_id.yellow().bold(),
                    m.line,
                    m.column
                );
                println!("   Message: {}", m.message);
                println!("   Context: \"{}\"", m.context);
                if !m.suggestions.is_empty() {
                    println!(
                        "   Suggestions: {}",
                        m.suggestions.join(", ").green()
                    );
                }
                println!();
            }

            println!(
                "{}",
                format!(
                    "Total: {} issue(s) found",
                    matches.len()
                )
                .red()
                .bold()
            );
        }
    }

    // Exit with appropriate code
    if matches.is_empty() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
