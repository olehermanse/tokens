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
    /// let b = b.unwrap();
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
    /// let b = b.expect("This isn't None in the example");
    ///
    /// assert_eq!(a.string, "a");
    /// assert_eq!(b.string, "b");
    /// assert_eq!(b.index, 1);
    /// ```
    pub fn split_at(self: Token<'a>, offset: usize) -> (Token<'a>, Option<Token<'a>>) {
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
        };

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
        };

        return (a, Some(b));
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
