use logguardian::parser::analyze_log;

#[test]
fn test_basic_log_parsing() {
    let report = analyze_log("demo.log");

    assert!(report.lines > 0);
    assert!(report.errors >= 0);
    assert!(report.warnings >= 0);
    assert!(report.infos >= 0);
}

#[test]
fn test_top_errors_counting() {
    let report = analyze_log("tests/top_errors.log");

    assert_eq!(report.top_errors[0].message, "ERROR Disk failure");
    assert_eq!(report.top_errors[0].count, 3);

    assert_eq!(report.top_errors[1].message, "ERROR Network timeout");
    assert_eq!(report.top_errors[1].count, 2);
}

#[test]
fn test_top_errors_sorted_descending() {
    let report = analyze_log("tests/top_errors.log");

    assert!(report.top_errors[0].count >= report.top_errors[1].count);
}
