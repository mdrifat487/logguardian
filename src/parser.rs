use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::{LogReport, ErrorCount};

pub fn analyze_log(path: &str) -> LogReport {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut errors = 0;
    let mut warnings = 0;
    let mut infos = 0;

    let mut error_map: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            lines += 1;

            if line.contains("ERROR") {
                errors += 1;

                let msg = extract_message(&line);
                *error_map.entry(msg).or_insert(0) += 1;
            } else if line.contains("WARN") {
                warnings += 1;
            } else if line.contains("INFO") {
                infos += 1;
            }
        }
    }

    let mut top_errors: Vec<ErrorCount> = error_map
        .into_iter()
        .map(|(message, count)| ErrorCount { message, count })
        .collect();

    top_errors.sort_by(|a, b| b.count.cmp(&a.count));

    LogReport {
        lines,
        errors,
        warnings,
        infos,
        top_errors: top_errors.into_iter().take(5).collect(),
    }
}

fn extract_message(line: &str) -> String {
    line.to_string()
}

