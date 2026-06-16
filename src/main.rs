use logguardian::parser::analyze_log;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;

fn validate_file(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("❌ File not found: {}", path));
    }

    if std::fs::metadata(path).is_err() {
        return Err(format!("❌ Cannot read file: {}", path));
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "logguardian")]
#[command(about = "A log analysis tool", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        file: String,
    },
    Top {
        file: String,
    },
    Export {
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { file } => {
            if let Err(msg) = validate_file(&file) {
                println!("{}", msg.red().bold());
                std::process::exit(1);
            }

            let report = analyze_log(&file);

            println!("{}", format!("⚪ Lines: {}", report.lines).bold());
            println!("{}", format!("🔴 Errors: {}", report.errors).red());
            println!("{}", format!("🟡 Warnings: {}", report.warnings).yellow());
            println!("{}", format!("🔵 Info: {}", report.infos).cyan());
        }

        Commands::Top { file } => {
            if let Err(msg) = validate_file(&file) {
                println!("{}", msg.red().bold());
                std::process::exit(1);
            }

            let report = analyze_log(&file);

            println!("{}", "🔥 Top Errors:".bold().red());
            for e in report.top_errors {
                println!("{}", format!("🔴 {} -> {}", e.message, e.count).red());
            }
        }

        Commands::Export { file } => {
            if let Err(msg) = validate_file(&file) {
                println!("{}", msg.red().bold());
                std::process::exit(1);
            }

            let report = analyze_log(&file);

            println!("{}", format!("⚪ Lines: {}", report.lines).bold());
            println!("{}", format!("🔴 Errors: {}", report.errors).red());
            println!("{}", format!("🟡 Warnings: {}", report.warnings).yellow());
            println!("{}", format!("🔵 Info: {}", report.infos).cyan());

            let json = serde_json::to_string_pretty(&report).unwrap();
            println!("📦 JSON Export:\n{}", json.bold());
        }
    }
}
