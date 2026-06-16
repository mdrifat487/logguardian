LogGuardian — Smart Log Analysis for Developers & DevOps
LogGuardian is a fast, reliable, and user‑friendly command‑line tool for analyzing log files.
It helps developers, system administrators, and students quickly extract insights from large logs without manually searching through thousands of lines.

 Features
Fast log parsing with optimized Rust code

Filter by level (INFO, WARN, ERROR, DEBUG)

Search by keyword

Count occurrences

Export filtered logs

Cross‑platform binaries

Linux (static MUSL)

Windows

macOS

 Download LogGuardian
Download the latest binaries from the GitHub Release page:

Linux x86_64 (MUSL)

Windows x86_64 (.exe)

macOS x86_64

 Installation
Linux / macOS
Code
chmod +x logguardian
./logguardian --help

Windows
Code
logguardian.exe --help

 Usage Examples

Filter by log level
Code
logguardian --level ERROR demo.log

Search for a keyword
Code
logguardian --search "database" demo.log

Count occurrences
Code
logguardian --count demo.log

Export filtered logs
Code
logguardian --level WARN --out warnings.log demo.log

 Project Structure
Code
src/
 ├── main.rs
 ├── parser.rs
 ├── models.rs
 └── lib.rs

tests/
 ├── parser_tests.rs
 └── integration_tests.rs

 Testing
Code
cargo test

 Source Code
Full project available on GitHub:
https://github.com/mdrifat487/logguardian (github.com in Bing)

 Author
Md Rifat
Rust CLI Development — Exam Project