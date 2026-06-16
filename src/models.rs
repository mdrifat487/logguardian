use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LogReport {
    pub lines: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub top_errors: Vec<ErrorCount>,
}

#[derive(Debug, Serialize)]
pub struct ErrorCount {
    pub message: String,
    pub count: usize,
}
