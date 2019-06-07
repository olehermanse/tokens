pub struct Token<'a> {
    pub string: &'a str,
    pub line: &'a str,
    pub index: usize,
    pub row: usize,
    pub col: usize,
}

impl<'a> Token<'a> {

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

    /// Splits a `Token` into 2
    ///
    /// # Examples
    ///
    /// ```
    ///  let token = tokens::Token::from("ab");
    ///  let (a,b) = token.split_at(1);
    ///
    ///  assert_eq!(a.string, "a");
    ///  assert_eq!(b.string, "b");
    ///  assert_eq!(b.index, 1);
    /// ```
    pub fn split_at(self: Token<'a>, index: usize) -> (Token<'a>, Token<'a>) {
        let (a, b) = self.string.split_at(index);
        let a = Token {
            string: a,
            index: self.index,
            line: self.line,
            row: self.row,
            col: self.col,
        };
        let b = Token {
            string: b,
            index: self.index + index,
            line: self.line,
            row: self.row,
            col: self.col + index,
        };
        return (a, b);
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

    #[test]
    fn split_ab() {
        let rest = Token::from("ab");
        let (a,b) = rest.split_at(1);
        assert_eq!("a", a.string);
        assert_eq!("b", b.string);
        assert_eq!(a.row, b.row);
        assert_eq!(a.col + 1, b.col);
        assert_eq!(a.index, 0);
        assert_eq!(b.index, 1);
    }

    #[test]
    fn split_abc_1() {
        let original = Token::from("abc");
        let (a,bc) = original.split_at(1);
        assert_eq!("a", a.string);
        assert_eq!("bc", bc.string);
        assert_eq!(a.row, bc.row);
        assert_eq!(a.col + 1, bc.col);
        assert_eq!(a.index, 0);
        assert_eq!(bc.index, 1);
    }

    #[test]
    fn split_abc_2() {
        let original = Token::from("abc");
        let (ab,c) = original.split_at(2);
        assert_eq!("ab", ab.string);
        assert_eq!("c", c.string);
        assert_eq!(ab.row, c.row);
        assert_eq!(ab.col + 2, c.col);
        assert_eq!(ab.index, 0);
        assert_eq!(c.index, 2);
    }
}
