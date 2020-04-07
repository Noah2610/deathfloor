/// Merge types together.
pub trait Merge: Sized {
    /// Merge other value into self.
    fn merge(&mut self, other: Self);

    /// Consumes both values, merges them together,
    /// and returns a new instance of `Self`.
    fn merged(mut self, other: Self) -> Self {
        self.merge(other);
        self
    }
}
