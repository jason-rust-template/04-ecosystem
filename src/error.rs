use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error {0}")]
    IO(#[from] std::io::Error),
    #[error("Parse error {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Custom error {0}")]
    Custom(String),
    #[error("Big error {0:?}")]
    BigError(Box<BigError>),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}
