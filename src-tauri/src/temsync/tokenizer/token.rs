use std::error::Error;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub start: usize,
    pub len: usize,
    pub value: String,
}

impl Token {
    pub fn new(start: usize, seq: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(Token { start, len: seq.len(), value: str::from_utf8(seq)?.to_owned() })
    }

    #[cfg(test)]
    pub fn new_with_values(start: usize, len: usize, value: String) -> Self {
        Token { start, len, value }
    }

    pub fn eq_value(&self, b: &Token) -> bool {
        self.value == b.value
    }
}
