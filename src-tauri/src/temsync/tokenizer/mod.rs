mod token;

use std::{error::Error, fs::File, io::Read, path::PathBuf};

pub use token::Token;

const ZERO: u8 = 48; // 0 ascii code
const NINE: u8 = 57; // 9 ascii code
const CAP_A: u8 = 65; // Capital A ascii code
const CAP_Z: u8 = 90; // Capital Z ascii code
const LOW_A: u8 = 97; // Lowercase A ascii code
const LOW_Z: u8 = 122; // Lowercase Z ascii code

/// Parses the given file, returning a vector of Tokens
pub fn parse_file_tokens(filename: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    // Open the file
    let mut f = File::open(PathBuf::from(filename))?;

    // Read entire file into a buffer
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Iterate through the buffer
    let mut tokens = Vec::<Token>::new();
    let mut start: usize = 0;

    for i in 0..buffer.len() {
        // Extract current character
        let curr = buffer
            .get(i)
            .ok_or_else(|| "buffer should not return null")?;

        // If alphabetical, keep going (appending to current token)
        if (ZERO <= *curr && NINE >= *curr)
            || (CAP_A <= *curr && CAP_Z >= *curr)
            || (LOW_A <= *curr && LOW_Z >= *curr)
        {
            continue;
        }

        // If we are at the end of a string (start != i), push that onto the stack
        if start != i {
            tokens.push(Token::new(start, &buffer[start..i])?);
        }

        // Then push current character onto stack
        tokens.push(Token::new(i, &buffer[i..(i + 1)])?);

        // Set start to next character
        start = i + 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        // Instantiate tokenizer
        let tokens = parse_file_tokens("./src/temsync/test-files/basic-tokens".into())
            .expect("tokenizer parse_file failed");

        // Expected tokens
        let expected_tokens = vec![
            Token::new_with_values(0, 1, "A".into()),     // "A"
            Token::new_with_values(1, 1, " ".into()),     // " "
            Token::new_with_values(2, 2, "AB".into()),    // "AB"
            Token::new_with_values(4, 1, " ".into()),     // " "
            Token::new_with_values(5, 4, "AB12".into()),  // "AB12"
            Token::new_with_values(9, 1, "\n".into()),    // "\n"
            Token::new_with_values(10, 1, "a".into()),    // "a"
            Token::new_with_values(11, 1, ".".into()),    // "."
            Token::new_with_values(12, 1, "a".into()),    // "a"
            Token::new_with_values(13, 1, "\n".into()),   // "\n"
            Token::new_with_values(14, 1, "e".into()),    // "e"
            Token::new_with_values(15, 1, " ".into()),    // " "
            Token::new_with_values(16, 1, " ".into()),    // " "
            Token::new_with_values(17, 1, " ".into()),    // " "
            Token::new_with_values(18, 3, "bee".into()),  // "bee"
            Token::new_with_values(21, 1, "!".into()),    // "!"
            Token::new_with_values(22, 1, "!".into()),    // "!"
        ];

        for (i, token) in expected_tokens.iter().enumerate() {
            assert!(tokens[i] == *token, "token {} is invalid, expected [{}]", i, token.value);
        }
    }

    /// To see this result, use the command cargo test -- --nocapture
    #[test]
    fn test_tokenizer_complex() {
        // Instantiate tokenizer
        let tokens = parse_file_tokens("./src/temsync/test-files/complex-tokens".into())
            .expect("tokenizer parse_file failed");

        // Print tokens
        println!("{:?}", tokens);
    }
}
