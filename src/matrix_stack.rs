pub struct MatrixStack<T> {
    inner: Vec<T>,
}

impl<T: Clone> MatrixStack<T> {
    pub fn top(&self) -> Option<T> {
        match self.inner.len() {
            0 => None,
            n => Some(self.inner[n - 1].clone()),
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
