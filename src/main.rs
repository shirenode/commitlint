use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "shirenode-commitlint", about = "Lint git commit messages against Conventional Commits")]
struct Cli {
    // Path to the commit message file (e.g. .git/COMMIT_EDITMSG)
    #[arg(short, long)]
    file: Option<String>,

    /// Commit message as a string
    #[arg(short, long)]
    message: Option<String>,
}

struct LintResult {
    rule: &'static str, 
    passed: bool,
    detail: String,
}

fn lint(msg: &str) -> Vec<LintResult> {
    let subject = msg.lines().next().unwrap_or("");
    let re = Regex::new(r"^(feat|fix|docs|style|refactor|perf|test|chore|ci)(\(.+\))?: .+").unwrap();

    vec![
        LintResult {
            rule: "format",
            passed: re.is_match(subject),
            detail: "Subject must match: <type>(scope): <description>".into(),
        },
        LintResult {
            rule: "subject-length",
            passed: subject.len() <= 72,
            detail: format!("Subject is {} chars (max 72)", subject.len()),
        },
        LintResult {
            rule: "no-trailing-period",
            passed: !subject.ends_with('.'),
            detail: "Subject must not end with a period".into(),
        },
        LintResult {
            rule: "body-separator",
            passed: msg.lines().count() == 1 || msg.lines().nth(1).map_or(true, |l| l.is_empty()),
            detail: "Blank line required between subject and body".into(),
        }
    ]
}

fn main() {
    let cli = Cli::parse();

    let msg = match (cli.message, cli.file) {
        (Some(m), _) => m,
        (_, Some(f)) => fs::read_to_string(&f).unwrap_or_else(|e| {
            eprintln!("{} Failed to read {}: {}", "𝘹".red(), f, e);
            process::exit(1);
        }),
        _ => {
            eprintln!("{} provide --message or --file", "x".red());
            process::exit(1);
        }
    };

    let results = lint(&msg);
    let mut failed = false;

    for r in &results {
        if r.passed {
            println!("  {} {}: {}", "✓".green(), r.rule, r.detail);
        } else {
            println!("  {} {}: {}", "𝘹".red(), r.rule, r.detail);
            failed = true;
        }
    }

    if failed {
        process::exit(1);
    } else {
        println!("\n{}", "Commit message looks good! ✓".green().bold());
    }
}