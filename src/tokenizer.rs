/**
 * Reads files and tokenizes text into tokens. A token is a continuous string of
 * text consisting of only alphanumeric characters and underscores, or only
 * non-underscore punctuation, and no whitespace.
 * 
 * Saves the original text and location of each token within the original text.
 * Provides an interface to replace tokens in the original text with new tokens.
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

struct Token {
    value: String,
    start: usize,
}

impl Token {
    fn new() -> Self {
        Token {
            value: String::new(),
            start: 0, // Can be updated later.
        }
    }
}

pub struct Tokenizer {
    lines: Lines<BufReader<File>>, // Source of input from the file.
    text: String, // Text generated as the lines are iterated over.
    next_statement: Vec<Token>,
    last_token_type: TokenType,
    next_index: usize,
}

impl Tokenizer {
    pub fn new(filename: &str) -> Self {
        let file_path = Path::new(filename);
        let file = File::open(&file_path)
                .expect(&format!("[ ERROR ] Failed to open file {}!", &filename));
        let reader = BufReader::new(file);
        Tokenizer {
            lines: reader.lines(),
            text: String::new(),
            next_statement: Vec::new(),
            last_token_type: TokenType::NONE,
            next_index: 0,
        }
    }

    /**
     * Tokenizes the next statement, terminated by one of ";", "{", or "}",
     * ignoring comments. Records the location of each token in the original
     * text. Returns whether the end of file is reached.
     */ 
    pub fn tokenize_next_statement(&mut self) -> bool {
        self.next_statement = Vec::new();
        loop {
            let lines_next = self.lines.next();
            let line: String;
            match lines_next {
                Some(l) => line = l.unwrap(),
                None => return true,
            }
            self.text.push_str(&line);
            if self.tokenize_line(line) {
                break;
            }
        }

        for t in &self.next_statement {
            println!("{}, {}", t.value, t.start);
        }
        false
    }

    /// Returns whether we have found the end of a statement (";", "{", and "}").
    fn tokenize_line(&mut self, line: String) -> bool {
        let mut end_statement = false;
        let mut token = Token::new();
        token.start = self.next_index;
        for c in line.chars() {
            let next_token_type = Tokenizer::token_type(c);
            if next_token_type == TokenType::NONE {
                // Ignore whitespace, except that it denotes the end of a token.
                self.last_token_type = TokenType::NONE;
                self.next_index += 1;
                continue;
            }

            if next_token_type == self.last_token_type {
                // We are continuing the same token, either an identifier or symbol.
                token.value.push(c);
            } else {
                /* We are starting a new token, either because we went from identifier
                   to symbol, vice versa, or the last char was whitespace. */
                self.add_token(token, next_token_type);
                token = Token {
                    value: c.to_string(),
                    start: self.next_index,
                };
            }

            self.next_index += 1;
            if Tokenizer::is_end_symbol(c) {
                end_statement = true;
                break;
            }
        }
        // the end of a line always means the token has ended
        self.add_token(token, TokenType::NONE);

        end_statement
    }

    fn add_token(&mut self, token: Token, token_type: TokenType) {
        self.last_token_type = token_type;
        if !token.value.is_empty() {
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
