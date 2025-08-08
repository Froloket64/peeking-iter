//! Parsing-specific iterator, with full peeking support.

use crate::peeking::{Peeking, PeekingIter};

/// An iterator implementing [`Peeking`], but designed specifically for parsing
/// string input.
///
/// **NOTE:** By convention, line numbers start at `1`, while column numbers
/// start at `0`.
pub struct Parser<I>
where
    I: Iterator,
{
    peeking: PeekingIter<I>,
    line: u16,
    col: u16,
}

impl<I: Iterator<Item = char> + Clone> Peeking for Parser<I> {
    fn peek(&mut self) -> Option<Self::Item> {
        self.peeking.peek()
    }

    fn rewind_peeking(&mut self) {
        self.peeking.rewind_peeking();
    }

    fn advance_to_peeked(&mut self) {
        self.peeking.advance_to_peeked();
    }
}

// TODO?: Implement whitespace-skipping
impl<I: Iterator<Item = char> + Clone> Parser<I> {
    /// Wraps the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            peeking: PeekingIter::new(iter),
            line: 1,
            col: 0,
        }
    }

    /// Returns the next item in the inner iterator.
    ///
    /// Resets the peeking iterator.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::Peeking;
    /// # use peeking_iter::parser::Parser;
    /// let mut parser = Parser::new("abcd".chars());
    ///
    /// assert_eq!(parser.peek(), Some('a'));
    /// assert_eq!(parser.next(), Some('a'));
    /// assert_eq!(parser.peek(), Some('b'));
    /// assert_eq!(parser.next(), Some('b'));
    /// ```
    pub fn next(&mut self) -> Option<char> {
        let next = self.peeking.next();

        // NOTE: This assumes that all characters (except newline)
        // advance the col by 1.
        match next {
            None => (),
            Some('\n') => {
                self.line += 1;
                self.col = 0;
            }
            _ => {
                self.col += 1;
            }
        }

        next
    }

    /// Consumes `self` and returns the inner iterator.
    pub fn into_inner(self) -> I {
        PeekingIter::into_inner(self.peeking)
    }

    /// Returns the line number.
    ///
    /// ```rust
    /// # use peeking_iter::parser::Parser;
    /// let mut it = Parser::new("ab\nc".chars());
    ///
    /// it.next();
    /// it.next();
    /// it.next();
    ///
    /// assert_eq!(it.line(), 2);
    /// ```
    pub fn line(&self) -> u16 {
        self.line
    }

    /// Returns the column number.
    ///
    /// **NOTE:** Every _character_ is considered to have the column size of 1.
    pub fn col(&self) -> u16 {
        self.col
    }
}

impl<I: Iterator<Item = char> + Clone> Iterator for Parser<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        Parser::next(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::peeking::Peeking;

    fn next_while() {
        let mut it = Parser::new("ABc".chars());

        assert_eq!(
            it.next_while(|c| c.is_uppercase()),
            "AB".chars().collect::<Vec<_>>()
        );
        assert_eq!(it.peek(), Some('c'));
        assert_eq!(it.next(), Some('c'));
    }
}
