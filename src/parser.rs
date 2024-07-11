/// An iterator implementing most (or all) of
/// [`PeekingIter`](crate::PeekingIter)'s API, but designed
/// specifically for parsing string input.
///
/// **NOTE:** By convention, line numbers start at 1, while column numbers
/// start at 0.
pub struct Parser<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    peeking: Option<I>,
    line: u16,
    col: u16,
}

// TODO?: Implement whitespace-skipping
impl<I: Iterator<Item = char> + Clone> Parser<I> {
    /// Wraps the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeking: None,
            line: 1,
            col: 0,
        }
    }

    /// Returns the next item in the inner iterator.
    ///
    /// Resets the peeking iterator.
    pub fn next(&mut self) -> Option<char> {
        self.peeking = None;

        let next = self.iter.next();

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

    /// Peeks the next item in the inner iterator.
    ///
    /// Subsequent calls return subsequent items.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("abc".chars());
    ///
    /// assert_eq!(it.next(), Some('a'));
    /// assert_eq!(it.peek(), Some('b'));
    /// assert_eq!(it.peek(), Some('c'));
    /// assert_eq!(it.next(), Some('b'));
    /// assert_eq!(it.peek(), Some('c'));
    /// assert_eq!(it.peek(), None);
    /// ```
    pub fn peek(&mut self) -> Option<char> {
        self.peeking.get_or_insert_with(|| self.iter.clone()).next()
    }

    /// Peek the `n`th value in the iterator.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("abc".chars());
    ///
    /// assert_eq!(it.peek_nth(2), Some('c'));
    /// assert_eq!(it.next(), Some('a'));
    /// ```
    pub fn peek_nth(&mut self, n: usize) -> Option<char> {
        n.checked_add(1)
            .and_then(|n1| (0..n1).flat_map(|_| self.peek()).last())
    }

    /// Advances the base iterator to the be aligned with the peeking one.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("abc".chars());
    ///
    /// assert_eq!(it.peek(), Some('a'));
    /// assert_eq!(it.peek(), Some('b'));
    ///
    /// it.advance_to_peeked();
    ///
    /// assert_eq!(it.next(), Some('c'));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn advance_to_peeked(&mut self) {
        if let Some(ref peeking) = self.peeking {
            self.iter = peeking.clone();
        }
    }

    /// Rewind the peeking iterator to align with the base one.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("abc".chars());
    ///
    /// assert_eq!(it.peek(), Some('a'));
    /// assert_eq!(it.peek(), Some('b'));
    ///
    /// it.rewind_peeking();
    ///
    /// assert_eq!(it.peek(), Some('a'));
    /// ```
    pub fn rewind_peeking(&mut self) {
        self.peeking = Some(self.iter.clone())
    }

    /// Returns a `Vec<I::Item>` containing all continuous elements that the
    /// predicate returns `true` for.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("ABc".chars());
    ///
    /// assert_eq!(it.next_while(|c| c.is_uppercase()), "AB".to_string());
    /// assert_eq!(it.peek(), Some('c'));
    /// assert_eq!(it.next(), Some('c'));
    /// ```
    pub fn next_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        // let mut result = vec![];
        let mut result = String::new();

        loop {
            match self.peek() {
                None => break,
                Some(x) => {
                    if pred(x) {
                        result.push(x);
                        self.next();
                    } else {
                        break;
                    }
                }
            }
        }

        self.rewind_peeking();

        result
    }

    /// Consumes `self` and returns the inner (base) iterator.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
    /// let mut it = Parser::new("abc".chars());
    ///
    /// it.next();
    ///
    /// assert_eq!(Parser::into_inner(it).collect::<String>(), "bc".to_string());
    /// ```
    pub fn into_inner(value: Self) -> I {
        value.iter
    }

    /// Returns the line number.
    ///
    /// ```rust
    /// # use peeking_iter::Parser;
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
    /// **NOTE:** Every character is assumed to have the column size of 1.
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
