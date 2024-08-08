use meyers::get_diff;
use patch::{generate_and_write_patch, generate_patch, read_patch};

mod token;
mod tokenizer;
mod edit;
mod meyers;
mod patch;

// TEMP: To get rid of warnings -- this will not be used until we have server stuff setup TT
pub fn main() {
    let a = tokenizer::parse_file_tokens("").expect("parsing should work!");
    let b = tokenizer::parse_file_tokens("").expect("parsing should work 2!");
    let diff = get_diff(&a, &b).expect("diff should work");
    generate_patch(&diff);
    let _ = generate_and_write_patch(&diff, "");
    let _ = read_patch("");
    println!("{:?}", diff);
}