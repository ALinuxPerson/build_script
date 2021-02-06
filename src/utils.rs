pub trait VecExt<T> {
    fn take(&mut self, index: usize) -> Option<T>;
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
