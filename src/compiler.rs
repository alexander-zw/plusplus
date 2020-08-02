/**
 * Makes use of the tokenizer to compile ++ into JavaScript.
 */
use crate::tokenizer::Tokenizer;

pub struct Compiler {
    tokenizer: Tokenizer,
}

impl Compiler {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Compiler { tokenizer }
    }

    pub fn compile(&mut self) -> Vec<String> {
        self.tokenizer.tokenize_next_statement();

        Vec::new()
    }
}
