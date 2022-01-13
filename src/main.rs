use std::env;
pub mod select;
pub mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lintable_args: Vec<String> = Vec::new();

    for arg in args {
        if arg.ends_with(".sql") {
            lintable_args.push(arg);
        }
    }

    for lintable_file in lintable_args {
        tokenizer::tokenize_file(&lintable_file);
    }
}
