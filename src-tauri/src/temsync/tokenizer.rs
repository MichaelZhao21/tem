use std::{error::Error, fs::File, io::Read, path::PathBuf};

use super::token::Token;

/// Parses the given file, returning a vector of Tokens
pub fn parse_file_tokens(filename: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    // Open the file
    let mut f = File::open(PathBuf::from(filename))?;

    // Read entire file into a buffer
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // Parse tokens
    parse_tokens(&buffer)
}

/// Parses the given string, returning a vector of Tokens
pub fn parse_string_tokens(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    // Convert string to u8 buffer
    parse_tokens(input)
}

pub fn parse_tokens(buffer: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    // TODO: REPLACE THIS WITH UNICODE SPLITTING AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    // (or maybe not) -> emoji splitting might be weird

    // Iterate through the buffer
    let mut tokens = Vec::<Token>::new();
    let mut start: usize = 0;
    let mut sb = Vec::<u16>::new();

    // Return empty is empty buff
    if buffer.len() == 0 {
        return Ok(tokens);
    }

    for (i, char) in buffer.encode_utf16().enumerate() {
        // If alphanumeric, keep going (appending to current token)
        // Note this accounts for the entire Unicode range
        // See https://www.fileformat.info/info/charset/UTF-16/list.htm
        if (char >= 0x30 && char <= 0x39)         // 0-9
            || (char >= 0x41 && char <= 0x5A)     // A-Z
            || (char >= 0x61 && char <= 0x7A)     // a-z
            || (char >= 0xC0 && char <= 0xD6)     // À-Ö
            || (char >= 0xD8 && char <= 0xF6)     // Ø-ö
            || (char >= 0xF8 && char <= 0x2AF)
        // ø-ʯ
        {
            sb.push(char);
            continue;
        }

        // If we are at the end of a string (start != i), push that onto the stack
        if !sb.is_empty() {
            tokens.push(Token::new(start, String::from_utf16(&sb)?.as_str()).unwrap());
            sb.clear();
        }

        // Then push current character onto stack
        tokens.push(Token::new(i, String::from_utf16(&vec![char])?.as_str()).unwrap());

        // Set start to next character
        start = i + 1;
    }

    // Push last alpha token to output if last char is alphabetical
    if start != buffer.len() {
        tokens.push(Token::new(start, &buffer[start..]).unwrap());
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

        println!("{:?}", tokens);

        // Expected tokens
        let expected_tokens = vec![
            Token::new_with_values(0, 1, "A".into()),    // "A"
            Token::new_with_values(1, 1, " ".into()),    // " "
            Token::new_with_values(2, 2, "AB".into()),   // "AB"
            Token::new_with_values(4, 1, " ".into()),    // " "
            Token::new_with_values(5, 4, "AB12".into()), // "AB12"
            Token::new_with_values(9, 1, "\n".into()),   // "\n"
            Token::new_with_values(10, 1, "a".into()),   // "a"
            Token::new_with_values(11, 1, ".".into()),   // "."
            Token::new_with_values(12, 1, "a".into()),   // "a"
            Token::new_with_values(13, 1, "\n".into()),  // "\n"
            Token::new_with_values(14, 1, "e".into()),   // "e"
            Token::new_with_values(15, 1, " ".into()),   // " "
            Token::new_with_values(16, 1, " ".into()),   // " "
            Token::new_with_values(17, 1, " ".into()),   // " "
            Token::new_with_values(18, 3, "bee".into()), // "bee"
            Token::new_with_values(21, 1, "!".into()),   // "!"
            Token::new_with_values(22, 1, "!".into()),   // "!"
        ];

        for (i, token) in expected_tokens.iter().enumerate() {
            assert!(
                tokens[i] == *token,
                "token {} is invalid, expected [{}]",
                i,
                token.value
            );
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
