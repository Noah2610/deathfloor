/// Merge types together.
/// Consumes both values and returns a new instance of `Self`.
pub trait Merge {
    fn merge(self, other: Self) -> Self;
}
