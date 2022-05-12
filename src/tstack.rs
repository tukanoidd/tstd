#[derive(Default)]
pub struct TStack<T>(Vec<T>);

impl<T> TStack<T> {
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }
}
