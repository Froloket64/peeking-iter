/// Iterator adapter that enables infinitely-deep peeking.
///
/// First call to [`peek()`] returns the next element, further calls
/// return further elements without advancing the base iterator.
///
/// The inner iterator is required to implement [`Clone`].
///
/// # Performance
/// It you don't call [`peek()`] at all, this is just as performant as the
/// original iterator.
///
/// This adapter is ~1.5x faster than [`itertools::MultiPeek`] (see
/// `/benches/bench.rs`).
///
/// [`peek()`]: PeekingIter::peek()
/// [`itertools::MultiPeek`]:
/// https://docs.rs/itertools/latest/itertools/structs/struct.MultiPeek.html
pub struct PeekingIter<I: Iterator> {
    iter: I,
    peeking: Option<I>,
}

impl<I: Iterator + Clone> PeekingIter<I> {
    /// Wraps the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeking: None,
        }
    }

    /// Returns the next item in the inner iterator.
    ///
    /// Resets the peeking iterator.
    pub fn next(&mut self) -> Option<I::Item> {
        self.peeking = None;

        self.iter.next()
    }

    /// Peeks the next item in the inner iterator.
    ///
    /// Subsequent calls return subsequent items.
    ///
    /// ```rust
    /// # use peeking_iter::PeekingIter;
    ///
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.next(), Some(0));
    /// assert_eq!(it.peek(), Some(1));
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.next(), Some(1));
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.peek(), None);
    /// ```
    pub fn peek(&mut self) -> Option<I::Item> {
        self.peeking.get_or_insert_with(|| self.iter.clone()).next()
    }

    /// Advances the base iterator to the be aligned with the peeking one.
    ///
    /// ```rust
    /// # use peeking_iter::PeekingIter;
    ///
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.peek(), Some(0));
    /// assert_eq!(it.peek(), Some(1));
    ///
    /// it.advance_to_peeked();
    ///
    /// assert_eq!(it.next(), Some(2));
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
    /// # use peeking_iter::PeekingIter;
    ///
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.peek(), Some(0));
    /// assert_eq!(it.peek(), Some(1));
    ///
    /// it.rewind_peeking();
    ///
    /// assert_eq!(it.peek(), Some(0));
    /// ```
    pub fn rewind_peeking(&mut self) {
        self.peeking = Some(self.iter.clone())
    }

    /// Returns a `Vec<I::Item>` containing all continuous elements that the
    /// predicate returns `true` for.
    ///
    /// ```rust
    /// # use peeking_iter::PeekingIter;
    ///
    /// let mut it = PeekingIter::new(0..=3);
    ///
    /// assert_eq!(it.next_while(|x| *x < 2), vec![0, 1]);
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.next(), Some(2));
    /// ```
    pub fn next_while<F: Fn(&I::Item) -> bool>(&mut self, pred: F) -> Vec<I::Item> {
        let mut result = vec![];

        loop {
            match self.peek() {
                None => break,
                Some(x) => {
                    if pred(&x) {
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

    /// Returns the inner (base) iterator.
    pub fn into_inner(self) -> I {
        self.iter
    }
}

impl<I: Iterator + Clone> Iterator for PeekingIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        PeekingIter::next(self)
    }
}

#[cfg(test)]
mod tests {
    use super::PeekingIter;

    #[test]
    fn peek() {
        let mut it = PeekingIter::new(0..=2);

        assert_eq!(it.next(), Some(0));
        assert_eq!(it.peek(), Some(1));
        assert_eq!(it.peek(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.peek(), Some(2));
        assert_eq!(it.peek(), None);
    }
}
