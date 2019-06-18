pub enum TokenCategory {
    Sequence,
    Alphanumeric,
    Whitespace,
    StringLiteral,
    Symbol,
    Unknown,
}

pub struct Token<'a> {
    pub string: &'a str,
    pub buffer: &'a str,
    pub line_start: usize,
    pub index: usize,
    pub row: usize,
    pub col: usize,
    pub category: TokenCategory,
}

fn get_sequence(s: &str) -> Option<&'static str> {
    let sequences = vec![
        "===", "<=", ">=", "!=", "==", "->", "=>", "*=", "+=", "/=", "%=", "::",
    ];
    for sequence in sequences {
        if s.starts_with(sequence) {
            return Some(sequence);
        }
    }
    return None;
}

fn find_first_token(s: &str) -> (TokenCategory, usize) {
    match get_sequence(s) {
        Some(seq) => {
            return (TokenCategory::Sequence, seq.len());
        }
        None => {}
    };
    let first = s.chars().nth(0).expect("Empty token!");
    let length = s.len();
    if is_alphanumeric(first) {
        let len = match s.find(|c: char| !is_alphanumeric(c)) {
            Some(n) => n,
            None => length,
        };
        return (TokenCategory::Alphanumeric, len);
    }
    if is_symbol(first) {
        return (TokenCategory::Symbol, 1);
    }
    let category = match first {
        ' ' => TokenCategory::Whitespace,
        '\n' => TokenCategory::Whitespace,
        '\t' => TokenCategory::Whitespace,
        '\'' => TokenCategory::StringLiteral,
        '\"' => TokenCategory::StringLiteral,
        _ => panic!(),
    };
    let length = match category {
        TokenCategory::Alphanumeric => match s.find(|c: char| !is_alphanumeric(c)) {
            Some(n) => n,
            None => length,
        },
        TokenCategory::Whitespace => match s.find(|c: char| c != first) {
            Some(n) => n,
            None => length,
        },
        TokenCategory::StringLiteral => {
            let close = s.match_indices(first).nth(1).unwrap().0;
            close + 2 * "'".len() - 1
        }
        TokenCategory::Symbol => 1,
        _ => panic!(),
    };

    return (category, length);
}

fn is_alphanumeric(c: char) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    return alphabet.contains(c);
}

fn is_symbol(c: char) -> bool {
    let symbols = "(){}<>[]:;,.@!/\\|-+=*?&%$#";
    return symbols.contains(c);
}

fn get_line(string: &str) -> &str {
    return match string.find("\n") {
        Some(n) => string.get(0..n).unwrap(),
        None => string,
    };
}

impl TokenCategory {
    pub fn from(s: &str) -> TokenCategory {
        find_first_token(s).0
    }
}

impl<'a> Token<'a> {
    fn assertions(self: &Token<'a>) {
        assert!(self.string.len() > 0);
        assert!(self.buffer.len() > 0);
        assert!(self.buffer.len() >= self.string.len());
        assert!(self.index < self.buffer.len());
        assert!(self.index == 0 || (self.row > 0 || self.col > 0));
        assert!(self.line_start + self.col == self.index);
    }

    /// Converts a `&str` into a `Token`
    ///
    /// # Examples
    ///
    /// ```
    /// let token = tokens::Token::from("ab\n");
    ///
    /// assert_eq!(token.string, "ab\n");
    /// assert_eq!(token.get_line(), "ab");
    /// assert_eq!(token.index, 0);
    /// ```
    pub fn from(string: &str) -> Token {
        let token = Token {
            string: string,
            buffer: string,
            line_start: 0,
            index: 0,
            row: 0,
            col: 0,
            category: TokenCategory::from(string),
        };
        token.assertions();
        return token;
    }

    /// Gets a string slice of the line where the `Token` starts.
    /// The newline character is not included.
    ///
    /// # Examples
    ///
    /// ```
    /// let token = tokens::Token::from("abc\ndef\nghi");
    ///
    /// assert_eq!(token.get_line(), "abc");
    ///
    /// let (a, b) = token.split_at(4);
    /// let b = b.unwrap();
    ///
    /// assert_eq!(a.get_line(), "abc");
    ///
    /// assert_eq!(b.string, "def\nghi");
    /// assert_eq!(b.get_line(), "def");
    /// ```
    pub fn get_line(self: &Token<'a>) -> &'a str {
        self.assertions();
        let string = self.buffer.get(self.line_start..).unwrap();
        return get_line(string);
    }

    /// Splits a `Token` into 2
    ///
    /// # Examples
    ///
    /// ```
    /// let token = tokens::Token::from("ab");
    /// let (a,b) = token.split_at(1);
    /// let b = b.expect("This isn't None in the example");
    ///
    /// assert_eq!(a.string, "a");
    /// assert_eq!(b.string, "b");
    /// assert_eq!(b.index, 1);
    /// ```
    pub fn split_at(self: Token<'a>, offset: usize) -> (Token<'a>, Option<Token<'a>>) {
        self.assertions();
        assert!(offset > 0);
        assert!(self.string.len() >= 1);
        assert!(offset <= self.string.len());
        let (a, b) = self.string.split_at(offset);
        let a = Token {
            string: a,
            buffer: self.buffer,
            index: self.index,
            line_start: self.line_start,
            row: self.row,
            col: self.col,
            category: self.category,
        };
        a.assertions();

        if b.len() == 0 {
            return (a, None);
        }

        let newlines = a.string.match_indices("\n").count();
        let row = self.row + newlines;
        let last_newline = a.string.rfind("\n");
        let (line_start, col) = match last_newline {
            Some(n) => (a.index + n + 1, a.string.len() - n - 1),
            None => (a.line_start, self.col + offset),
        };

        let b = Token {
            string: b,
            buffer: self.buffer,
            index: self.index + offset,
            line_start: line_start,
            row: row,
            col: col,
            category: TokenCategory::from(b),
        };
        b.assertions();

        return (a, Some(b));
    }

    /// Extracts the next token pair
    ///
    /// # Examples
    ///
    /// ```
    /// let token = tokens::Token::from("ab;");
    /// let (ab, semi) = token.next_pair();
    /// let semi = semi.unwrap();
    /// assert_eq!(ab.string, "ab");
    /// assert_eq!(semi.string, ";");
    /// ```
    pub fn next_pair(self: Token<'a>) -> (Token<'a>, Option<Token<'a>>) {
        self.assertions();

        let offset = find_first_token(self.string).1;
        return self.split_at(offset);
    }

    /// Splits an initial token into a vector of tokens, including whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// let initial = tokens::Token::from("ab;");
    /// let tokens = initial.get_tokens_including_whitespace();
    /// for token in tokens {
    ///     print!("{}", token.string);
    /// }
    /// ```
    pub fn get_tokens_including_whitespace(self: Token<'a>) -> Vec<Token<'a>> {
        let (token, remainder) = self.next_pair();
        return match remainder {
            Some(remainder) => {
                let mut a = vec![token];
                let b = remainder.get_tokens();
                a.extend(b);
                a
            }
            None => vec![token],
        };
    }

    /// Splits an initial token into a vector of tokens, including whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// let initial = tokens::Token::from("ab;");
    /// let tokens = initial.get_tokens();
    /// for token in tokens {
    ///     println!("{}", token.string);
    /// }
    /// ```
    pub fn get_tokens(self: Token<'a>) -> Vec<Token<'a>> {
        return self
            .get_tokens_including_whitespace()
            .into_iter()
            .filter(|t| match t.category {
                TokenCategory::Whitespace => false,
                _ => true,
            })
            .collect();
    }

    pub fn get_strings_including_whitespace(self: Token<'a>) -> Vec<&'a str> {
        return self
            .get_tokens_including_whitespace()
            .into_iter()
            .map(|t: Token| t.string)
            .collect();
    }

    pub fn get_strings(self: Token<'a>) -> Vec<&'a str> {
        return self
            .get_tokens()
            .into_iter()
            .map(|t: Token| t.string)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_from_simple() {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let token = Token::from(alphabet);
        assert_eq!(token.string, alphabet);
        assert_eq!(token.buffer, alphabet);
        assert_eq!(token.get_line(), alphabet);
        assert_eq!(token.row, 0);
        assert_eq!(token.col, 0);
        assert_eq!(token.index, 0);
    }

    #[test]
    fn token_from_lines() {
        let lines = "abc\ndef\nghi";
        let token = Token::from(lines);
        assert_eq!(token.string, lines);
        assert_eq!(token.row, 0);
        assert_eq!(token.col, 0);
        assert_eq!(token.index, 0);
    }

    #[test]
    fn split_simple() {
        let rest = Token::from("ab");
        let (a, b) = rest.split_at(1);
        assert_eq!("a", a.string);
        assert_eq!("b", b.unwrap().string);
    }

    #[test]
    fn split_end() {
        let rest = Token::from(";");
        let (a, b) = rest.split_at(1);
        assert_eq!(";", a.string);
        match b {
            Some(_) => {
                panic!();
            }
            None => {}
        };
    }

    #[test]
    #[should_panic]
    fn panic_split_empty() {
        let rest = Token::from("");
        let (_a, _b) = rest.split_at(0);
    }

    #[test]
    #[should_panic]
    fn panic_split_outside() {
        let rest = Token::from("a");
        let (_a, _b) = rest.split_at(2);
    }

    #[test]
    #[should_panic]
    fn panic_split_at_zero() {
        let rest = Token::from("a");
        let (_a, _b) = rest.split_at(0);
    }

    #[test]
    fn get_strings_simple() {
        let v = Token::from("bundle agent main\n{reports:any::'Hello, world';}\n").get_strings();
        assert_eq!(
            v,
            [
                "bundle",
                "agent",
                "main",
                "{",
                "reports",
                ":",
                "any",
                "::",
                "'Hello, world'",
                ";",
                "}",
            ]
        );
    }

    #[test]
    fn get_strings_sequences() {
        let buffer = "\"promise name\"->{} attribute=>{'words','more words'};";
        let initial_token = Token::from(buffer);
        let v = initial_token.get_strings();
        assert_eq!(
            v,
            [
                "\"promise name\"",
                "->",
                "{",
                "}",
                "attribute",
                "=>",
                "{",
                "'words'",
                ",",
                "'more words'",
                "}",
                ";",
            ]
        );
    }

    #[test]
    fn next_simple() {
        let buffer = "age = 40;";
        let remaining = Token::from(buffer);
        let (word, remaining) = remaining.next_pair();
        assert_eq!(word.string, "age");
        let remaining = remaining.unwrap();
        let (space, remaining) = remaining.next_pair();
        assert_eq!(space.string, " ");
        let remaining = remaining.unwrap();
        let (equals, remaining) = remaining.next_pair();
        assert_eq!(equals.string, "=");
        let remaining = remaining.unwrap();
        let (space, remaining) = remaining.next_pair();
        assert_eq!(space.string, " ");
        let remaining = remaining.unwrap();
        let (number, remaining) = remaining.next_pair();
        assert_eq!(number.string, "40");
        let remaining = remaining.unwrap();
        let (semicolon, remaining) = remaining.next_pair();
        assert_eq!(semicolon.string, ";");
        match remaining {
            Some(_) => {
                panic!();
            }
            None => {}
        };
    }

    #[test]
    fn split_abc_1() {
        let original = Token::from("abc");
        let offset = 1;
        let (a, bc) = original.split_at(offset);
        let bc = bc.unwrap();
        assert_eq!("a", a.string);
        assert_eq!("bc", bc.string);
        assert_eq!(a.get_line(), "abc");
        assert_eq!(bc.get_line(), "abc");
        assert_eq!(a.row, bc.row);
        assert_eq!(a.col + offset, bc.col);
        assert_eq!(a.index, 0);
        assert_eq!(bc.index, offset);
    }

    #[test]
    fn split_abc_2() {
        let original = Token::from("abc");
        let offset = 2;
        let (ab, c) = original.split_at(offset);
        let c = c.unwrap();
        assert_eq!("ab", ab.string);
        assert_eq!("c", c.string);
        assert_eq!(ab.get_line(), "abc");
        assert_eq!(c.get_line(), "abc");
        assert_eq!(ab.row, c.row);
        assert_eq!(ab.col + offset, c.col);
        assert_eq!(ab.index, 0);
        assert_eq!(c.index, offset);
    }

    #[test]
    fn split_lines() {
        // Semi-realsitic example, splits 15 tokens (including whitespace):
        let buffer = "def main():\n    a=0\n    return\n";
        let original = Token::from(buffer);

        let (def, rest) = original.split_at("def".len());
        let rest = rest.unwrap();
        assert_eq!(def.string, "def");
        assert_eq!(def.row, 0);
        assert_eq!(def.col, 0);

        let (space, rest) = rest.split_at(" ".len());
        let rest = rest.unwrap();
        assert_eq!(space.string, " ");
        assert_eq!(space.row, 0);
        assert_eq!(space.col, "def".len());

        let (main, rest) = rest.split_at("main".len());
        let rest = rest.unwrap();
        assert_eq!(main.string, "main");
        assert_eq!(main.row, 0);
        assert_eq!(main.col, "def ".len());

        let (open, rest) = rest.split_at("(".len());
        let rest = rest.unwrap();
        assert_eq!(open.string, "(");
        assert_eq!(open.row, 0);
        assert_eq!(open.col, "def main".len());

        let (close, rest) = rest.split_at(")".len());
        let rest = rest.unwrap();
        assert_eq!(close.string, ")");
        assert_eq!(close.row, 0);
        assert_eq!(close.col, "def main(".len());

        let (colon, rest) = rest.split_at(":".len());
        let rest = rest.unwrap();
        assert_eq!(colon.string, ":");
        assert_eq!(colon.row, 0);
        assert_eq!(colon.col, "def main()".len());

        let (newline, rest) = rest.split_at("\n".len());
        let rest = rest.unwrap();
        assert_eq!(newline.string, "\n");
        assert_eq!(newline.row, 0);
        assert_eq!(newline.col, "def main():".len());

        let (indentation, rest) = rest.split_at(4);
        let rest = rest.unwrap();
        assert_eq!(indentation.string, "    ");
        assert_eq!(indentation.row, 1);
        assert_eq!(indentation.col, 0);

        let (a, rest) = rest.split_at("a".len());
        let rest = rest.unwrap();
        assert_eq!(a.string, "a");
        assert_eq!(a.row, 1);
        assert_eq!(a.col, 4);

        let (equals, rest) = rest.split_at("=".len());
        let rest = rest.unwrap();
        assert_eq!(equals.string, "=");
        assert_eq!(equals.row, 1);
        assert_eq!(equals.col, "    a".len());

        let (zero, rest) = rest.split_at("0".len());
        let rest = rest.unwrap();
        assert_eq!(zero.string, "0");
        assert_eq!(zero.row, 1);
        assert_eq!(zero.col, "    a=".len());

        let (newline, rest) = rest.split_at("\n".len());
        let rest = rest.unwrap();
        assert_eq!(newline.string, "\n");
        assert_eq!(newline.row, 1);
        assert_eq!(newline.col, "    a=0".len());

        let (indentation, rest) = rest.split_at(4);
        let rest = rest.unwrap();
        assert_eq!(indentation.string, "    ");
        assert_eq!(indentation.row, 2);
        assert_eq!(indentation.col, 0);

        let (ret, final_newline) = rest.split_at("return".len());
        let final_newline = final_newline.unwrap();
        assert_eq!(ret.string, "return");
        assert_eq!(ret.row, 2);
        assert_eq!(ret.col, 4);

        assert_eq!(final_newline.string, "\n");
        assert_eq!(final_newline.row, 2);
        assert_eq!(final_newline.col, "    return".len());
        assert_eq!(final_newline.buffer, buffer);
        assert_eq!(final_newline.index, final_newline.buffer.len() - 1);
        assert_eq!(final_newline.get_line(), "    return");
    }
}
