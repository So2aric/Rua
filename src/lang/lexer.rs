use std::{iter::Peekable, vec::IntoIter};

use super::token::{Token, TokenKind, Location};

pub struct Lexer {
    text: Peekable<IntoIter<char>>,
    ch: Option<char>,
    loc: Location
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        let mut text = text.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .peekable();
        let ch = text.next();

        Lexer { text, ch, loc: Location::new() }
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut res = vec![];

        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if ch.is_digit(10) {
                let value = self.get_number();
                res.push(self.make_token(
                    TokenKind::Number,
                    Some(value)
                ));
                continue;
            }

            if ch.is_alphabetic() {
                let ident = self.get_ident();

                res.push(
                    match ident.as_str() {
                        "if"        => self.make_token( TokenKind::If, None ),
                        "else"      => self.make_token( TokenKind::Else, None ),
                        "elseif"    => self.make_token( TokenKind::Elseif, None ),
                        "then"      => self.make_token( TokenKind::Then, None ),
                        "do"        => self.make_token( TokenKind::Do, None ),
                        "while"     => self.make_token( TokenKind::While, None ),
                        "end"       => self.make_token( TokenKind::End, None ),
                        "function"  => self.make_token( TokenKind::Function, None ),
                        "true"      => self.make_token( TokenKind::True, None ),
                        "false"     => self.make_token( TokenKind::False, None ),
                        "and"       => self.make_token( TokenKind::And, None ),
                        "not"       => self.make_token( TokenKind::Not, None ),
                        "or"        => self.make_token( TokenKind::Or, None ),

                        _ => self.make_token( TokenKind::Ident, Some(ident) )
                    }
                );
                continue;
            }

            if ['\'', '"'].contains(&ch) {
                let value = self.get_qte_string(ch);
                res.push(self.make_token(
                    TokenKind::String,
                    Some(value)
                ));
                continue;
            }

            match ch {
                '[' => {
                    // check if this is the start of a string
                    if let Some(ch) = self.peek() {
                        if ['=', '['].contains(&ch) {
                            self.advance();

                            let value = self.get_long_string();
                            res.push(self.make_token(
                                TokenKind::String,
                                Some(value)
                            ));
                        // otherwise, it's a left square parenthesis
                        } else {
                            res.push(self.make_token(
                                TokenKind::Lsqr,
                                None
                            ));
                        }
                    }
                },
                ']' => res.push(self.make_token(
                    TokenKind::Rsqr,
                    None
                )),
                '(' => res.push(self.make_token(
                    TokenKind::Lpar,
                    None
                )),
                ')' => res.push(self.make_token(
                    TokenKind::Rpar,
                    None
                )),
                '{' => res.push(self.make_token(
                    TokenKind::Lbrc,
                    None
                )),
                '}' => res.push(self.make_token(
                    TokenKind::Rbrc,
                    None
                )),

                '+' => res.push(self.make_token(
                    TokenKind::Plus,
                    None
                )),
                '-' => res.push(self.make_token(
                    TokenKind::Minus,
                    None
                )),
                '*' => res.push(self.make_token(
                    TokenKind::Mul,
                    None
                )),
                '^' => res.push(self.make_token(
                    TokenKind::Pow,
                    None
                )),
                '/' => {
                    // / or // ?
                    if let Some(_ch @ '/') = self.peek() {
                        self.advance();

                        res.push(self.make_token(
                            TokenKind::IntDiv,
                            None
                        ));
                    } else {
                        res.push(self.make_token(
                            TokenKind::RealDiv,
                            None
                        ));
                    }
                },
                '.' => {
                    // . or .. ?
                    if let Some(_ch @ '.') = self.peek() {
                        self.advance();

                        // .. or ... ?
                        if let Some(_ch @ '.') = self.peek() {
                            self.advance();
    
                            res.push(self.make_token(
                                TokenKind::Arg,
                                None
                            ));
                        // ..
                        } else {
                            res.push(self.make_token(
                                TokenKind::Concat,
                                None
                            ));
                        }
                    // .
                    } else {
                        res.push(self.make_token(
                            TokenKind::Dot,
                            None
                        ));
                    }
                },
                ',' => res.push(self.make_token(
                    TokenKind::Comma,
                    None
                )),
                ':' => res.push(self.make_token(
                    TokenKind::Colon,
                    None
                )),
                ';' => res.push(self.make_token(
                    TokenKind::Semi,
                    None
                )),

                '=' => {
                    // = or == ?
                    if let Some(_ch @ '=') = self.peek() {
                        self.advance();

                        res.push(self.make_token(
                            TokenKind::Eq,
                            None
                        ));
                    } else {
                        res.push(self.make_token(
                            TokenKind::Assign,
                            None
                        ));
                    }
                },
                '~' => {
                    // it must be ~=, so we simply skip the first '~'
                    self.advance();

                    res.push(self.make_token(
                        TokenKind::UnEq,
                        None
                    ));
                }
                '<' => {
                    // < or <= ?
                    if let Some(_ch @ '=') = self.peek() {
                        self.advance();

                        res.push(self.make_token(
                            TokenKind::Le,
                            None
                        ));
                    } else {
                        res.push(self.make_token(
                            TokenKind::Lt,
                            None
                        ));
                    }
                },
                '>' => {
                    // > or >= ?
                    if let Some(_ch @ '=') = self.peek() {
                        self.advance();

                        res.push(self.make_token(
                            TokenKind::Ge,
                            None
                        ));
                    } else {
                        res.push(self.make_token(
                            TokenKind::Gt,
                            None
                        ));
                    }
                },

                _ => panic!("Unexpected char.")
            }

            self.advance();
        }

        res.push(self.make_token( TokenKind::Eof, None ));

        res
    }

    fn advance(&mut self) {
        self.ch = self.text.next();

        if self.ch == Some('\r') || self.ch == Some('\n') {
            self.loc.new_line();
        } else {
            self.loc.advance();
        }
    }

    fn peek(&mut self) -> Option<char> {
        match self.text.peek() {
            Some(ch) => Some(*ch),
            None => None
        }
    }

    fn make_token(&self, kind: TokenKind, value: Option<String>) -> Token {
        Token { kind, value, loc: self.loc }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_number(&mut self) -> String {
        let mut res = String::new();

        while let Some(ch) = self.ch {
            if ch.is_digit(10) {
                res.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // handle real number
        if let Some(ch @ '.') = self.ch {
            res.push(ch);
            self.advance();

            while let Some(ch) = self.ch {
                if ch.is_digit(10) {
                    res.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        res
    }

    fn get_ident(&mut self) -> String {
        let mut res = String::new();

        while let Some(ch) = self.ch {
            if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                res.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        res
    }

    fn get_qte_string(&mut self, qte: char) -> String {
        // skip the first quote
        self.advance();

        let mut res = String::new();

        while let Some(ch) = self.ch {
            if ch != qte {
                res.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // skip the second quote
        self.advance();

        res
    }

    fn get_long_string(&mut self) -> String {
        // since the analyze method has already skip the first '['
        // we could get the count of '=' directly
        let mut count = 0;

        while let Some(ch) = self.ch {
            if ch == '=' {
                count += 1;
                self.advance();
            } else {
                break;
            }
        }

        // skip the second '['
        self.advance();

        let mut res = String::new();

        // to judge whether it's the string's end or not
        let mut temp_res = String::new();
        let mut temp_count = 0;

        while let Some(ch) = self.ch {
            if ch == ']' {
                // is it the first ']'?
                if temp_res.is_empty() {
                    temp_res.push(ch);
                } else {
                    // then it's the second ']'
                    // if the count matches, the string is ended, break the loop
                    if count == temp_count {
                        break;
                    } else {
                        // otherwise, append the temp string to the res
                        res.push_str(&temp_res);
                        res.push(ch);

                        // clear up the temps
                        temp_res.clear();
                        temp_count = 0;
                    }
                }
            } else if ch == '=' && !temp_res.is_empty() {
                temp_res.push(ch);
                temp_count += 1;
            } else {
                // not between two ']'s
                if temp_res.is_empty() {
                    res.push(ch);
                } else {
                    // otherwise, it means that it's not a string's end
                    res.push_str(&temp_res);
                    res.push(ch);

                    temp_res.clear();
                    temp_count = 0;
                }
            }

            self.advance();
        }

        // skip the second ']'
        self.advance();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_string() {
        let mut lexer = Lexer::new("===[]==]]===]");

        assert_eq!(
            "]==]".to_string(),

            lexer.get_long_string()
        );
    }

    #[test]
    fn analyze() {
        let mut lexer = Lexer::new(r#"
            a = 1

            if a + 1 >= 3.5 and a ^ 3 == 2 then
                print("Hello World!")
            end
        "#);

        let res = vec![
            TokenKind::Ident,
            TokenKind::Assign,
            TokenKind::Number,
            TokenKind::If,
            TokenKind::Ident,
            TokenKind::Plus,
            TokenKind::Number,
            TokenKind::Ge,
            TokenKind::Number,
            TokenKind::And,
            TokenKind::Ident,
            TokenKind::Pow,
            TokenKind::Number,
            TokenKind::Eq,
            TokenKind::Number,
            TokenKind::Then,
            TokenKind::Ident,
            TokenKind::Lpar,
            TokenKind::String,
            TokenKind::Rpar,
            TokenKind::End,
            TokenKind::Eof
        ];

        lexer.analyze()
            .into_iter()
            .zip(res.into_iter())
            .for_each(|(l, r)| {
                assert_eq!(l.kind, r);
            });
    }
}