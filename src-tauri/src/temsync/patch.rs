use std::{error::Error, fs};

use super::{
    edit::{Edit, EditType},
    tokenizer::parse_string_tokens,
};

/// Generates a patch file given a list of edits.
/// This will ignore the SAME edits and just generate the diffs.
pub fn generate_patch(edits: &Vec<Edit>) -> String {
    let mut out = Vec::<String>::new();

    for e in edits.iter() {
        // Ignore sames
        if e.edit_type == EditType::SAME {
            continue;
        }

        // Add operation and numbers, adjusting for the addition offset due to nature of patch
        out.push(match e.edit_type {
            EditType::INSERT => format!("+{},{}#", e.old_index - 1, e.new_index),
            EditType::DELETE => format!("-{},{}#", e.old_index, e.new_index - 1),
            _ => panic!("Illegal edit type detected: {:?}", e.edit_type),
        });

        // Combine all tokens and add to output
        out.push(
            e.tokens
                .iter()
                .map(|t| escape_chars_and_clone(&t.value))
                .collect::<Vec<String>>()
                .join(""),
        );

        // Add newline to output
        out.push("\n".into());
    }

    out.join("")
}

/// Reads a string patch file and converts it to a list of edits.
pub fn read_patch(raw_content: &str) -> Result<Vec<Edit>, Box<dyn Error>> {
    // Create output vec
    let mut output = Vec::<Edit>::new();

    // Loop through each line
    let lines = raw_content.split('\n');
    for line in lines {
        // If empty line, throw error
        if line == "" {
            return Err("unexpected empty line".into());
        }

        // If invalid line, emit a warning (should have symbol,num,comma,num,#,<text>)
        if !line.starts_with('+') && !line.starts_with('-') {
            return Err("expected + or - at beginning of line, none found".into());
        }
        let pound_idx = line.find('#').ok_or_else(|| "expected #, none found")?;
        let comma_idx = line.find(',').ok_or_else(|| "expected comma, none found")?;
        if comma_idx > pound_idx {
            return Err("expected comma before #, none found".into());
        }
        if comma_idx == 1 || comma_idx + 1 == pound_idx {
            return Err("two line numbers expected (eg. +12,35#), not found".into());
        }
        if pound_idx + 1 == line.len() {
            return Err("chars expected after #, none found".into());
        }

        // Extract required info from line
        let mut ci = line.chars();
        let op = match ci.next().expect("should be able to take char") {
            '+' => EditType::INSERT,
            '-' => EditType::DELETE,
            _ => {
                panic!("should not have another character here");
            }
        };
        let old_idx: usize = (&mut ci)
            .take(comma_idx - 1)
            .collect::<String>()
            .parse()
            .map_err(|e| format!("expected number for old index: {}", e))?;
        ci.next();
        let new_idx: usize = (&mut ci)
            .take(pound_idx - comma_idx - 1)
            .collect::<String>()
            .parse()
            .map_err(|e| format!("expected number for new index: {}", e))?;
        ci.next();
        let tokens = parse_string_tokens(&replace_escaped_chars(ci.collect::<String>()))?;

        // Create the current edit object
        output.push(Edit::new_with_tokens(old_idx, new_idx, op, tokens));
    }

    Ok(output)
}

/// Escape special characters and clone string reference
fn escape_chars_and_clone(s: &str) -> String {
    match s {
        "\n" => "\\n".into(),
        "\r" => "\\r".into(),
        "\t" => "\\t".into(),
        _ => s.to_string(),
    }
}

/// Un-escape special characters
/// TODO: This is horrible and makes like 3 string copies
fn replace_escaped_chars(s: String) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
}

/// Generate and write patch to file
pub fn generate_and_write_patch(edits: &Vec<Edit>, filename: &str) -> Result<(), Box<dyn Error>> {
    // Generate patch
    let patch = generate_patch(edits);

    // Write patch to file
    fs::write(filename, patch)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::temsync::{
        meyers::get_diff, patch::generate_patch, token::Token, tokenizer::parse_file_tokens,
    };

    use super::read_patch;

    #[test]
    pub fn test_generate_patch() {
        let a = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 1, " ".into()),
            Token::new_with_values(2, 3, "car".into()),
            Token::new_with_values(5, 1, " ".into()),
            Token::new_with_values(6, 3, "ate".into()),
            Token::new_with_values(9, 1, " ".into()),
            Token::new_with_values(10, 2, "my".into()),
            Token::new_with_values(12, 1, " ".into()),
            Token::new_with_values(13, 3, "dog".into()),
            Token::new_with_values(16, 1, ".".into()),
        ];
        let b = vec![
            Token::new_with_values(0, 1, "A".into()),
            Token::new_with_values(1, 1, " ".into()),
            Token::new_with_values(2, 3, "red".into()),
            Token::new_with_values(5, 1, " ".into()),
            Token::new_with_values(6, 3, "car".into()),
            Token::new_with_values(9, 1, " ".into()),
            Token::new_with_values(10, 5, "eaten".into()),
            Token::new_with_values(15, 1, " ".into()),
            Token::new_with_values(16, 2, "my".into()),
        ];
        let edits = get_diff(&a, &b).expect("meyers get_diff failed");

        println!("{:?}", edits);

        // Generate patch
        println!("{}", generate_patch(&edits));
    }

    #[test]
    pub fn test_generate_patch_complex() {
        // NOTE: This will break if these functions break; this is not a good unit test lol but im lazy
        let a = parse_file_tokens("./src/temsync/test-files/poem-a")
            .expect("tokenizer parse_file_tokens failed");
        let b = parse_file_tokens("./src/temsync/test-files/poem-b")
            .expect("tokenizer parse_file_tokens failed");

        let edits = get_diff(&a, &b).expect("meyers get_diff failed");
        println!("{}", generate_patch(&edits));
    }

    #[test]
    pub fn test_generate_patch_readme() {
        // NOTE: This will break if these functions break; this is not a good unit test lol but im lazy
        let a = parse_file_tokens("./src/temsync/test-files/readme-a")
            .expect("tokenizer parse_file_tokens failed");
        let b = parse_file_tokens("./src/temsync/test-files/readme-b")
            .expect("tokenizer parse_file_tokens failed");

        let edits = get_diff(&a, &b).expect("meyers get_diff failed");
        println!("{}", generate_patch(&edits));
    }

    #[test]
    pub fn test_read_patch() {
        let patch = "+1,2#red \n-4,5#ate\n+4,6#eaten\n-7,8# dog.";

        let out = read_patch(patch).expect("read_patch failed");
        println!("{:?}", out);
    }
}
