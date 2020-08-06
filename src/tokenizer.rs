/**
 * Reads files and tokenizes text into tokens. A token is a continuous string of
 * text consisting of only alphanumeric characters and underscores, or one
 * non-underscore punctuation. Whitespace, comments, and Non-ASCII characters are
 * not part of tokens and only serve to separate tokens.
 * 
 * Saves the original text and location of each token within the original text.
 * Provides an interface to replace tokens in the original text with new tokens.
 * 
 * Although some characters together for a keyword, the tokenizer treats them as
 * separate tokens for ease of implementation.
 */
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(PartialEq, Clone)]
enum TokenType {
    Identifier, // Alphanumerical or underscore.
    Symbol, // Any punctuation that isn't underscore.
    BlockComment, // We are in the middle of a block comment.
    LineComment, // We are in the middle of a single-line comment.
    None, // We just finished a token, and the next character is a new one (or whitespace).
}

struct Token {
    value: String,
    start: usize,
    token_type: TokenType,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenType::Identifier => {
                write!(f, "Identifier")
            }
            TokenType::Symbol => {
                write!(f, "Symbol")
            }
            TokenType::BlockComment => {
                write!(f, "BlockComment")
            }
            TokenType::LineComment => {
                write!(f, "LineComment")
            }
            TokenType::None => {
                write!(f, "None")
            }
        }
    }
}

impl Token {
     // start and token_type should be updated later.
    fn new() -> Self {
        Token {
            value: String::new(),
            start: 0,
            token_type: TokenType::None,
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
            last_token_type: TokenType::None,
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
            let line: String;
            match self.lines.next() {
                Some(l) => line = l.unwrap(),
                None => return true,
            }
            self.text.push_str(&format!("{}\n", &line));
            if self.tokenize_line(line) {
                break;
            }
        }

        for t in &self.next_statement {
            println!("{}, {}, {}", t.value, t.start, t.token_type);
        }
        false
    }

    fn tokenize_line(&mut self, line: String) -> bool {
        let mut end_statement = false;
        let mut last_char_is_star = false; // Used to identify "*/".
        let mut token = Token::new();
        for c in line.chars() {
            if self.last_token_type == TokenType::LineComment {
                self.next_index += 1;
                continue; // The rest of this line will be ignored, but increment index.
            }
            if self.last_token_type == TokenType::BlockComment {
                // Scan the line for "*/" but ignore anything else until comment is closed.
                if c == '/' && last_char_is_star {
                    self.last_token_type = TokenType::None;
                }
                last_char_is_star = c == '*';
                self.next_index += 1;
                continue;
            }

            let next_token_type = Tokenizer::char_token_type(c);
            if next_token_type == TokenType::None {
                // Ignore whitespace, except that it denotes the end of a token.
                self.last_token_type = TokenType::None;
                self.next_index += 1;
                continue;
            }

            if next_token_type == self.last_token_type {
                /* We are continuing the same token, either an identifier or symbol.
                   For now, treat consecutive symbols as a single token, but
                   separate before adding them. */
                token.value.push(c);
            } else {
                /* We are starting a new token, either because we went from identifier
                   to symbol, vice versa, or the last char was whitespace. */
                self.add_token(token, next_token_type.clone());
                token = Token {
                    value: c.to_string(),
                    start: self.next_index,
                    token_type: next_token_type,
                };
            }

            self.next_index += 1;
            if Tokenizer::is_end_symbol(c) {
                self.last_token_type = TokenType::Symbol;
                end_statement = true;
                break;
            }
        }
        match self.last_token_type {
            // If block comment, do nothing.
            TokenType::BlockComment => (),
            // If single-line comment, don't add a token, but end the comment.
            TokenType::LineComment => self.last_token_type = TokenType::None,
            // Otherwise, the end of a line always means the token has ended.
            _ => self.add_token(token, TokenType::None),
        }
        self.next_index += 1; // Account for newline at end.

        end_statement
    }

    /**
     * For symbol tokens, first strips out comments, then separates symbols into single
     * tokens. Then adds them to tokenizer. Ignores empty tokens. Sets token type.
     */
    fn add_token(&mut self, token: Token, next_token_type: TokenType) {
        self.last_token_type = next_token_type;
        if token.value.is_empty() {
            return;
        }
        let first_char = token.value.chars().next().unwrap();
        if Tokenizer::char_token_type(first_char) != TokenType::Symbol {
            self.next_statement.push(token);
            return;
        }

        let stripped_tokens = self.strip_comments(token);
        for t in stripped_tokens {
            // Separate string of symbols into individual char tokens.
            for (i, c) in t.value.chars().enumerate() {
                self.next_statement.push(Token {
                    value: c.to_string(),
                    start: t.start + i,
                    token_type: t.token_type.clone(),
                });
            }
        }
    }

    /**
     * Removes parts of the token that are comments and sets self.last_token_type
     * appropriately. If block comments separate the token, splits token into
     * multiple tokens.
     */
    fn strip_comments(&mut self, mut token: Token) -> Vec<Token> {
        // First remove all block comments, taking care to handle the "//*" case.
        let mut stripped_tokens = Vec::new();
        while !token.value.is_empty() {
            let block_comment_start: usize;
            match token.value.find("/*") {
                Some(i) => block_comment_start = i,
                None => break,
            }
            let token_chars: Vec<char> = token.value.chars().collect();
            if block_comment_start != 0 && token_chars[block_comment_start - 1] == '/' {
                break; // This "/*" is actually part of "//*", skip.
            }

            let block_comment_end; // Index of first character after "*/".
            match Tokenizer::find_substring(&token.value, "*/", block_comment_start + 2) {
                Some(i) => block_comment_end = i + 2,
                None => {
                    self.last_token_type = TokenType::BlockComment;
                    break;
                },
            }
            stripped_tokens.push(Token {
                value: token.value[..block_comment_start].to_string(),
                start: token.start,
                token_type: token.token_type.clone(),
            });
            token = Token {
                value: token.value[block_comment_end..].to_string(),
                start: token.start + block_comment_end,
                token_type: token.token_type,
            };
        }
        // Then find the "//" and ignore anything after it, if we are not in a block comment.
        if self.last_token_type != TokenType::BlockComment {
            match token.value.find("//") {
                Some(line_comment_start) => {
                    self.last_token_type = TokenType::LineComment;
                    token.value = token.value[..line_comment_start].to_string();
                },
                None => (),
            }
            stripped_tokens.push(token);
        }

        stripped_tokens
    }

    /**
     * Based on the character returns the guessed token type: Identifier, Symbol, or None
     * (whitespace). Does not handle comments.
     */
    fn char_token_type(c: char) -> TokenType {
        if c.is_ascii_alphanumeric() || c == '_' {
            TokenType::Identifier
        } else if c.is_ascii_punctuation() {
            TokenType::Symbol
        } else {
            TokenType::None
        }
    }

    /// Returns whether we have found the end of a statement (";", "{", and "}").
    fn is_end_symbol(c: char) -> bool {
        return c == ';' || c == '{' || c == '}';
    }

    fn find_substring(string: &String, target: &str, start: usize) -> Option<usize> {
        // get() and find() both return an option, so we need to unwrap once.
        string.get(start..)
              .map(|s| s.find(target).map(|i| start + i))
              .unwrap_or(None)
    }
}
