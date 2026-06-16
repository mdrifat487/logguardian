# LogGuardian

LogGuardian is a Rust-based command-line tool for log analysis and system diagnostics.  
It provides a unified CLI for scanning log files, summarizing errors and warnings, extracting the most frequent issues, and exporting results in JSON format.  
The project includes cross‑compiled binaries, automated GitHub Actions builds, and a GitHub Pages website for distribution.

---

## 🩺 Motivation

Modern systems generate large volumes of logs.  
Manually inspecting these files is slow, error‑prone, and difficult to automate.

LogGuardian was developed to:

- Speed up troubleshooting  
- Provide structured summaries of log health  
- Extract repeated errors for root‑cause analysis  
- Support automation workflows via JSON output  
- Demonstrate real‑world Rust CLI development for the 24W‑GBAC exam  

---

## ✨ Features

- Fast log scanning using buffered streaming  
- Categorizes messages (Error, Warning, Info)  
- Extracts top repeated errors  
- JSON export mode for automation  
- Clean modular Rust architecture  
- Unit tests + integration tests  
- Cross‑compiled binaries (aarch64 + others)  
- GitHub Actions CI/CD pipeline  
- GitHub Pages website for distribution  

---

## 🚀 Command-Line Usage

### Scan a log file

```bash
logguardian scan demo.log

cargo test

git clone https://github.com/mdrifat487/logguardian
cd logguardian

cargo build --release

target/release/logguardian

.github/workflows/main.yml

https://mdrifat487.github.io/logguardian/

/docs/downloads/logguardian-aarch64

logguardian/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── parser.rs
│   ├── models.rs
├── tests/
│   ├── integration_tests.rs
│   └── parser_tests.rs
├── docs/
│   └── (GitHub Pages website files)
├── Cargo.toml
├── Cargo.lock
└── README.md

MIT License
Copyright (c) 2026
Permission granted for academic and non‑commercial use.

Author
Md Rifat Hossain
24W‑GBAC Exam Project
TH Deggendorf
