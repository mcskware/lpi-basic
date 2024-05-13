//! Applesoft II BASIC lexer
//!
//! The lexer takes the input source code and converts it into tokens.

#[derive(Debug, PartialEq)]
enum ParseState {
    Start,
    Number,
    Identifier,
    String,
}

/// Stub function to return a string
#[must_use]
pub fn lex(input: &str) -> Vec<String> {
    // we want to split the input into tokens
    let mut tokens = Vec::new();
    let mut state = ParseState::Start;
    let mut current_token = String::new();

    for token in input.chars() {
        match token {
            '"' => {
                if state == ParseState::String {
                    current_token.push(token);
                    tokens.push(current_token.clone());
                    current_token.clear();
                    state = ParseState::Start;
                } else {
                    state = ParseState::String;
                    current_token.push(token);
                }
            }
            _ if state == ParseState::String => {
                current_token.push(token);
            }
            ' ' => {
                if state == ParseState::String {
                    current_token.push(token);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                    state = ParseState::Start;
                }
            }
            'E' | 'e' if state == ParseState::Number => {
                current_token.push(token);
            }
            '+' | '-' if state == ParseState::Number => {
                let last_token = current_token.chars().last().unwrap_or(' ');
                if last_token == 'E' || last_token == 'e' {
                    current_token.push(token);
                } else {
                    tokens.push(current_token.clone());
                    current_token.clear();
                    current_token.push(token);
                    state = ParseState::Start;
                }
            }
            '0'..='9' | 'A'..='Z' | 'a'..='z' if state == ParseState::Identifier => {
                current_token.push(token);
            }
            'A'..='Z' | 'a'..='z' => {
                state = ParseState::Identifier;
                current_token.push(token);
            }
            '$' | '%' if state == ParseState::Identifier => {
                current_token.push(token);
                tokens.push(current_token.clone());
                current_token.clear();
                state = ParseState::Start;
            }
            '0'..='9' | '.' => {
                if state != ParseState::Number {
                    state = ParseState::Number;
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                current_token.push(token);
            }
            _ => {
                tokens.push(current_token.clone());
                current_token.clear();
                state = ParseState::Start;
                current_token.push(token);
            }
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // remove empty tokens
    tokens.retain(|token| !token.is_empty());

    tokens
}

#[cfg(test)]
mod tests;
