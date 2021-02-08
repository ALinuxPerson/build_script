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

#[cfg(test)]
mod tests {
    use super::VecExt;

    #[test]
    fn test_take() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let first = vec.take(0);
        assert_eq!(first, Some(1));
        let none = vec.take(999);
        assert!(none.is_none())
    }

    #[test]
    fn test_take_first() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let first = vec.take_first();
        assert_eq!(first, Some(1));
        vec.clear();
        let none = vec.take_first();
        assert!(none.is_none())
    }
}
