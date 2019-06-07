pub struct Token<'a> {
    pub string: &'a str,
    pub buffer: &'a str,
    pub line_start: usize,
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
    /// let token = tokens::Token::from("ab\n");
    ///
    /// assert_eq!(token.string, "ab\n");
    /// assert_eq!(token.get_line(), "ab");
    /// assert_eq!(token.index, 0);
    /// ```
    pub fn from(string: &str) -> Token {
        return Token {
            string: string,
            buffer: string,
            line_start: 0,
            index: 0,
            row: 0,
            col: 0,
        };
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
    ///
    /// assert_eq!(a.get_line(), "abc");
    ///
    /// assert_eq!(b.string, "def\nghi");
    /// assert_eq!(b.get_line(), "def");
    /// ```
    pub fn get_line(self: &Token<'a>) -> &'a str {
        let string = self.buffer.get(self.line_start..).unwrap();
        return match string.find("\n") {
            Some(n) => string.get(0..n).unwrap(),
            None => string,
        };
    }

    /// Splits a `Token` into 2
    ///
    /// # Examples
    ///
    /// ```
    /// let token = tokens::Token::from("ab");
    /// let (a,b) = token.split_at(1);
    ///
    /// assert_eq!(a.string, "a");
    /// assert_eq!(b.string, "b");
    /// assert_eq!(b.index, 1);
    /// ```
    pub fn split_at(self: Token<'a>, offset: usize) -> (Token<'a>, Token<'a>) {
        assert!(offset > 0);
        assert!(self.string.len() >= 2);
        assert!(offset < self.string.len());
        let (a, b) = self.string.split_at(offset);
        let a = Token {
            string: a,
            buffer: self.buffer,
            index: self.index,
            line_start: self.line_start,
            row: self.row,
            col: self.col,
        };
        let line_start = match a.string.rfind("\n") {
            Some(n) => a.index + n + 1,
            None => a.line_start,
        };
        let b = Token {
            string: b,
            buffer: self.buffer,
            index: self.index + offset,
            line_start: line_start,
            row: self.row,
            col: self.col + offset,
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
        assert_eq!(token.buffer, alphabet);
        assert_eq!(token.get_line(), alphabet);
        assert_eq!(token.row, 0);
        assert_eq!(token.col, 0);
        assert_eq!(token.index, 0);
    }

    #[test]
    fn token_from_line() {
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
        assert_eq!("b", b.string);
    }

    #[test]
    fn split_abc_1() {
        let original = Token::from("abc");
        let offset = 1;
        let (a, bc) = original.split_at(offset);
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
        assert_eq!("ab", ab.string);
        assert_eq!("c", c.string);
        assert_eq!(ab.get_line(), "abc");
        assert_eq!(c.get_line(), "abc");
        assert_eq!(ab.row, c.row);
        assert_eq!(ab.col + offset, c.col);
        assert_eq!(ab.index, 0);
        assert_eq!(c.index, offset);
    }
}
