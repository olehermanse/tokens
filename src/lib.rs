pub struct Token<'a> {
    pub string: &'a str,
    pub line: &'a str,
    pub index: usize,
    pub row: usize,
    pub col: usize,
}

impl Token<'_>{

    /// Converts a `&str` into a `Token`
    ///
    /// # Examples
    ///
    /// ```
    ///  let token = tokens::Token::from("ab\n");
    ///
    ///  assert_eq!(token.string, "ab\n");
    ///  assert_eq!(token.line, "ab");
    ///  assert_eq!(token.index, 0);
    /// ```
    pub fn from(string: &str) -> Token {
        let newline = string.find("\n");
        let line = match newline {
            Some(n) => {string.get(0..n).unwrap()}
            None => {string}
        };
        return Token {string: string, line: line, index: 0, row: 1, col: 1};
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
        assert_eq!(token.line, alphabet);
        assert_eq!(token.row, 1);
        assert_eq!(token.col, 1);
        assert_eq!(token.index, 0);
    }

    #[test]
    fn token_from_line() {
        let lines = "abc\ndef\nghi";
        let token = Token::from(lines);
        assert_eq!(token.string, lines);
        assert_eq!(token.line, "abc");
        assert_eq!(token.row, 1);
        assert_eq!(token.col, 1);
        assert_eq!(token.index, 0);
    }
}
