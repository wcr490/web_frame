use super::*;

pub struct MQueue {
    inner: VecDeque<Middleware>,
}
pub struct Middleware {}
impl MQueue {
    pub fn new() -> Self {
        MQueue {
            inner: VecDeque::new(),
        }
    }
    pub fn enqueue(&mut self, item: Middleware) {
        self.inner.push_back(item);
    }
    pub fn dequeue(&mut self) -> Option<Middleware> {
        self.inner.pop_front()
    }
    pub fn peek(&self) -> Option<&Middleware> {
        self.inner.front()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
