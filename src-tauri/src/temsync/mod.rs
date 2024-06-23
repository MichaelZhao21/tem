#[cfg(test)]
mod tests;

mod tokenizer;
mod diffengine;

pub fn main() {
    tokenizer::parse_file_tokens("").expect("parsing should work!");
}