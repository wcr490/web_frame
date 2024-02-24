use super::*;

pub type MwQueueMap = HashMap<Flag, MwQueue>;

pub struct MwQueue {
    inner: VecDeque<Box<dyn Middleware>>,
    data: RequestType,
    is_boot: bool,
}
unsafe impl Send for MwQueue {}
unsafe impl Sync for MwQueue {}

pub struct Flag(String);

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

impl MwQueue {
    pub fn new() -> Self {
        MwQueue {
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

#[macro_export]
macro_rules! mw_queue_generator {
    () => {};
    ($queue: expr, $($midwares: expr),+) => {
        {
            // let vec = [$($midwares),+];
            // for mw in vec {
            //     $queue.enqueue(Box::new(mw));
            // }
            $($queue.enqueue(Box::new($midwares));)+
        }

    };
}

#[macro_export]
macro_rules! mw_queue_map_generator {
    () => {};
    ($map: expr, $($flag: expr => $queue: expr),+) => {
        {
            $($map.insert($flag, $queue);)+
        }
    };
}
#[test]
fn test() {}
