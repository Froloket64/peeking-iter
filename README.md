peeking-iter
------------

A fast and simple iterator adapter that allows peeking with any depth:

```rust
use peeking_iter::PeekingIter

fn main() {
    let it = PeekingIter::new(0..=2);

    assert_eq!(it.next(), Some(0));
    assert_eq!(it.peek(), Some(1));
    assert_eq!(it.peek(), Some(2));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.peek(), Some(2));
    assert_eq!(it.peek(), None);
}
```

# Why?
- ~1.5x faster than itertools' `MultiPeek`
- Has useful methods initially designed with lexing and parsing in mind (see `Parser`)
