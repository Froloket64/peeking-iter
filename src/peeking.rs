/// Iterator adapter that enables infinitely-deep peeking.
///
/// First call to [`peek()`] returns the next element, further calls
/// return further elements without advancing the inner iterator.
///
/// The inner iterator is required to implement [`Clone`].
///
/// Also see [`Peeking`] for more detailed documentation of most methods.
///
/// # Performance
/// If you don't call [`peek()`] at all, this is just as performant as the
/// original iterator.
///
/// When using [`peek()`], this adapter is ~1.5x faster than
/// [`itertools::MultiPeek`] (see `/benches/bench.rs`).
///
/// All operations are `O(1)`, unless they specifically iterate over multiple
/// elements (e.g. [`peek_nth()`], [`next_while()`], etc.)
///
/// [`peek()`]: PeekingIter::peek()
/// [`peek_nth()`]: PeekingIter::peek_nth()
/// [`next_while()`]: PeekingIter::next_while()
/// [`itertools::MultiPeek`]:
/// https://docs.rs/itertools/latest/itertools/structs/struct.MultiPeek.html
pub struct PeekingIter<I> {
    iter: I,
    peeking: Option<I>,
}

/// Enables peeking functionality.
///
/// Requires the type to implement [`Iterator`] as well as `peek()`,
/// `advance_to_peeked()`, and `rewind_peeking()`.
///
/// Implemented by default on [`PeekingIter`] and [`Parser`](crate::Parser).
pub trait Peeking: Iterator {
    /// Peeks the next item in the iterator.
    ///
    /// Subsequent calls return subsequent items.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.next(), Some(0));
    /// assert_eq!(it.peek(), Some(1));
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.next(), Some(1));
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.peek(), None);
    /// ```
    fn peek(&mut self) -> Option<Self::Item>;

    /// Advances the inner iterator to the be aligned with the peeking one.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
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
    fn advance_to_peeked(&mut self);

    /// Rewind the peeking iterator to align with the inner.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.peek(), Some(0));
    /// assert_eq!(it.peek(), Some(1));
    ///
    /// it.rewind_peeking();
    ///
    /// assert_eq!(it.peek(), Some(0));
    /// ```
    fn rewind_peeking(&mut self);

    /// Peek the `n`th value in the iterator.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
    /// let mut it = PeekingIter::new(0..=2);
    ///
    /// assert_eq!(it.peek_nth(2), Some(2));
    /// assert_eq!(it.next(), Some(0));
    /// ```
    fn peek_nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.peek();
        }

        self.peek()
    }

    /// Returns a `Vec<T>` containing all continuous elements that satisfy the
    /// predicate.
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
    /// let mut it = PeekingIter::new(0..=3);
    ///
    /// assert_eq!(it.next_while(|x| *x < 2), vec![0, 1]);
    /// assert_eq!(it.peek(), Some(2));
    /// assert_eq!(it.next(), Some(2));
    /// ```
    fn next_while<F: Fn(&Self::Item) -> bool>(&mut self, pred: F) -> Vec<Self::Item> {
        let mut result = vec![];

        // If `peeking` had already diverged, bring it back
        self.rewind_peeking();

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

    /// Like [`next_while()`](Peeking::next_while()), except consumes the first
    /// element that doesn't suffice (without returning it).
    ///
    /// Doesn't [`peek()`](Self::peek()) at all, so it is faster than
    /// [`next_while()`](Self::next_while()).
    ///
    /// ```rust
    /// # use peeking_iter::peeking::{PeekingIter, Peeking};
    /// let mut it = PeekingIter::new(0..=3);
    ///
    /// assert_eq!(it.next_while1(|x| *x < 2), vec![0, 1]);
    /// assert_eq!(it.peek(), Some(3));
    /// assert_eq!(it.next(), Some(3));
    /// ```
    /// Note the `Some(3)`, instead of `Some(2)`.
    fn next_while1<F: Fn(&Self::Item) -> bool>(&mut self, pred: F) -> Vec<Self::Item> {
        let mut result = vec![];

        loop {
            match self.next() {
                None => break,
                Some(x) => {
                    if pred(&x) {
                        result.push(x)
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

/// Allows converting any iterator to a peeking one (typically by wrapping around it).
pub trait ToPeeking
where Self: Sized {
    fn to_peeking(self) -> PeekingIter<Self>;
}

impl<I: Iterator> PeekingIter<I> {
    /// Wraps the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeking: None,
        }
    }

    /// Returns the next item in the iterator.
    ///
    /// Resets the peeking iterator.
    pub fn next(&mut self) -> Option<I::Item> {
        self.peeking = None;

        self.iter.next()
    }

    /// Consumes `self` and returns the inner iterator.
    pub fn into_inner(value: Self) -> I {
        value.iter
    }
}

impl<I: Iterator + Clone> Peeking for PeekingIter<I> {
    fn peek(&mut self) -> Option<Self::Item> {
        self.peeking.get_or_insert_with(|| self.iter.clone()).next()
    }

    fn advance_to_peeked(&mut self) {
        if let Some(ref peeking) = self.peeking {
            self.iter = peeking.clone();
        }
    }

    fn rewind_peeking(&mut self) {
        self.peeking = None;
    }

    // OPTIM?: Potentially more optimal implementation?
    fn peek_nth(&mut self, n: usize) -> Option<I::Item> {
        self.peeking
            .get_or_insert_with(|| self.iter.clone())
            .skip(n)
            .next()
    }
}

impl<I: Iterator + Clone> Iterator for PeekingIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        PeekingIter::next(self)
    }
}

impl<I: Iterator + Clone> ToPeeking for I {
    fn to_peeking(self) -> PeekingIter<Self> {
        PeekingIter::new(self)
    }
}