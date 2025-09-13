use std::ops::Range;

#[derive(Debug)]
pub struct Analyser<T> {
    stream: Vec<T>,
    range: Range<usize>,
}

impl<T> Analyser<T> {
    pub fn new(stream: Vec<T>) -> Self {
        Self {
            stream,
            range: 0..0,
        }
    }
    /// Get the current list of stream's elements
    pub fn get(&self) -> &[T] {
        &self.stream[self.range.clone()]
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
    pub fn stream(&self) -> &[T] {
        &self.stream
    }

    /// Returns if the analyser can skip `skip` elements and then have a length of `length`.
    pub fn able_to(&self, skip: usize, length: usize) -> bool {
        self.range.end + skip + length <= self.stream.len()
    }
    /// Returns if the analyser can increase its size by `by`
    pub fn able_to_increase(&self, by: usize) -> bool {
        self.able_to(0, by)
    }
    /// Set the range of the analyser. Panics if the range is not valid
    pub fn set(&mut self, range: Range<usize>) -> &mut Self {
        assert!(
            range.start <= range.end,
            "Invalid analyser range ({:?}). The start must be lower than the end index.",
            range
        );
        assert!(
            range.end <= self.stream.len(),
            "Analyser out of bounds ({:?}). Maximum accepted : {}.",
            range,
            self.stream.len()
        );
        self.range = range;

        self
    }
    /// Go to the next tokens after the ending ones.
    /// This method uses the `set` method to edit the range.
    pub fn next(&mut self, skip: usize, length: usize) -> &mut Self {
        self.set((self.range.end + skip)..(self.range.end + skip + length))
    }
    /// Increase the size of the analyser's content.
    /// This method uses the `set` method to edit the range.
    pub fn increase(&mut self, by: usize) -> &mut Self {
        self.set(self.range.start..(self.range.end + by))
    }

    /// If the analyser range is < than the `length` parameter, try to increase it.
    /// Returns if the analyser, after being modified has a length >= than the given one.
    pub fn min_len(&mut self, length: usize) -> bool {
        if self.range.len() < length && self.able_to_increase(length) {
            self.increase(length);
        };

        self.range.len() >= length
    }

    /// Lookup forward items.
    /// Returns None if the given arguments result to an invalid range.
    pub fn lookup(&self, skip: usize, length: usize) -> Option<&[T]> {
        if self.able_to(skip, length) {
            Some(&self.stream[(self.range.end + skip)..(self.range.end + skip + length)])
        } else {
            None
        }
    }
    /// Returns true if the range is [end, end[
    pub fn process_finished(&self) -> bool {
        self.range.is_empty() && self.range.end == self.stream.len()
    }
}
