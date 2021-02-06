//! Some other extra utility functions/traits that are only used internally within
//! [`build_script`](crate).

/// Some useful vector extensions.
pub trait VecExt<T> {
    /// Take ownership of an item. If the index is out of range, this returns [`None`](None).
    fn take(&mut self, index: usize) -> Option<T>;

    /// Take ownership of the first item in a [`Vec`](Vec). If the vector is empty, this returns
    /// [`None`](None).
    fn take_first(&mut self) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn take(&mut self, index: usize) -> Option<T> {
        if self.get(index).is_none() {
            None
        } else {
            self.drain(index..index + 1).next()
        }
    }

    fn take_first(&mut self) -> Option<T> {
        self.take(0)
    }
}
