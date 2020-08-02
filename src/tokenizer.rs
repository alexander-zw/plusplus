/**
 * Tokenizes that reads files and tokenizes text into tokens.
 */
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(PartialEq)]
enum TokenType {
    IDENTIFIER, // alphanumerical or underscore
    SYMBOL, // any punctuation that isn't underscore
    NONE,
}

pub struct Tokenizer {
    lines: Lines<BufReader<File>>,
    next_statement: Vec<String>,
    last_token_type: TokenType,
}

impl Tokenizer {
    pub fn new(filename: &str) -> Self {
        let file_path = Path::new(filename);
        let file = File::open(&file_path)
                .expect(&format!("[ ERROR ] Failed to open file {}!", &filename));
        let reader = BufReader::new(file);
        Tokenizer {
            lines: reader.lines(),
            next_statement: Vec::new(),
            last_token_type: TokenType::NONE,
        }
    }

    // finds where the next statement ends, terminated by one of ;, {, and }, ignoring comments
    pub fn tokenize_next_statement(&mut self) -> &Vec<String> {
        self.next_statement = Vec::new();
        loop {
            let lines_next = self.lines.next();
            let line: String;
            match lines_next {
                Some(l) => line = l.unwrap(),
                None => break,
            }
            if self.tokenize_line(line) {
                break;
            }
        }

        &self.next_statement
    }

    /// Returns whether we have found the end of a statement (";", "{", and "}").
    fn tokenize_line(&mut self, line: String) -> bool {
        let mut end_statement = false;
        let mut token = String::new();
        for c in line.chars() {
            print!("NEXT CHAR: {} ", c);
            let next_token_type = Tokenizer::token_type(c);
            if next_token_type == TokenType::NONE {
                println!("WHITESPACE");
                // Ignore whitespace, except that it denotes the end of a token.
                self.last_token_type = TokenType::NONE;
                continue;
            }

            if next_token_type == self.last_token_type {
                println!("CONTINUE TOKEN");
                // We are continuing the same token, either an identifier or symbol.
                token.push(c);
            } else {
                println!("NEW TOKEN");
                /* We are starting a new token, either because we went from identifier
                   to symbol, vice versa, or the last char was whitespace. */
                self.add_token(token, next_token_type);
                token = c.to_string();
            }

            if Tokenizer::is_end_symbol(c) {
                end_statement = true;
                break;
            }
        }
        // the end of a line always means the token has ended
        self.add_token(token, TokenType::NONE);

        end_statement
    }

    fn add_token(&mut self, token: String, token_type: TokenType) {
        self.last_token_type = token_type;
        if !token.is_empty() {
            self.next_statement.push(token);
        }
    }

    fn token_type(c: char) -> TokenType {
        if c.is_ascii_alphanumeric() || c == '_' {
            TokenType::IDENTIFIER
        } else if c.is_ascii_punctuation() {
            TokenType::SYMBOL
        } else {
            TokenType::NONE
        }
    }

    fn is_end_symbol(c: char) -> bool {
        return c == ';' || c == '{' || c == '}';
    }
}
