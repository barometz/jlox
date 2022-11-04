use thiserror::Error;

#[derive(Debug)]
pub struct Token {}

#[derive(Error, Debug)]
#[error("{line}: {message}")]
pub struct ScannerError {
    line: usize,
    message: String,
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, ScannerError> {
    Err(ScannerError {
        line: 0,
        message: "Not implemented".into(),
    })
}
