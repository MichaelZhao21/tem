use meyers::get_diff;

#[cfg(test)]
mod tests;

mod tokenizer;
mod meyers;
mod diffengine;

pub fn main() {
    let a = tokenizer::parse_file_tokens("").expect("parsing should work!");
    let b = tokenizer::parse_file_tokens("").expect("parsing should work 2!");
    let diff = get_diff(&a, &b).expect("diff should work");
    println!("{:?}", diff);
}