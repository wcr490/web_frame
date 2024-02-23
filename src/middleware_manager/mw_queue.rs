use super::*;

pub struct MQueue {
    inner: VecDeque<Box<dyn Middleware>>,
    data: RequestType,
    is_boot: bool,
}
unsafe impl Send for MQueue {}

pub enum Priority {
    Unknown,
    P1,
    P2,
    P3,
}
pub trait Middleware {
    fn exe(&self, req: RequestType) -> RequestType;
    fn priority(&self) -> Priority;
}

impl MQueue {
    pub fn new() -> Self {
        MQueue {
            inner: VecDeque::new(),
            data: RequestType::Empty,
            is_boot: false,
        }
    }
    pub fn boot(&mut self, req: RequestType) -> bool {
        if let Some(head) = self.peek_mut() {
            head.exe(req);
            self.is_boot = true;
        }
        self.dequeue()
    }
    pub fn enqueue(&mut self, item: Box<dyn Middleware>) {
        self.inner.push_back(item);
    }
    pub fn dequeue(&mut self) -> bool {
        let data_cloned = self.data.clone();
        if self.is_boot {
            if let Some(cur_midware) = self.inner.pop_front() {
                if let Some(__next_midware) = self.peek() {
                    self.data = cur_midware.exe(data_cloned);
                    return true;
                }
                self.is_boot = false;
                return true;
            }
            return false;
        }
        false
    }
    pub fn peek_mut(&mut self) -> Option<&mut Box<dyn Middleware>> {
        self.inner.front_mut()
    }
    pub fn peek(&self) -> Option<&Box<dyn Middleware>> {
        self.inner.front()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
