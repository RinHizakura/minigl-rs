pub struct MatrixStack<T> {
    inner: Vec<T>,
}

impl<T> MatrixStack<T> {
    pub fn top(&mut self) -> Option<&T> {
        match self.inner.len() {
            0 => None,
            n => Some(&self.inner[n - 1]),
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
}

impl<T> Default for MatrixStack<T> {
    fn default() -> Self {
        MatrixStack {
            inner: Default::default(),
        }
    }
}
