use super::*;

pub type MwQueueMap = HashMap<Flag, MwQueue>;

#[derive(Clone)]
pub struct MwQueue {
    inner: VecDeque<Box<dyn Middleware>>,
    data: RequestType,
    is_boot: bool,
}
unsafe impl Send for MwQueue {}
unsafe impl Sync for MwQueue {}

#[derive(Hash, Eq, Clone)]
pub struct Flag(pub String);
impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        other.0 == self.0
    }
}

pub enum Priority {
    Unknown,
    P1,
    P2,
    P3,
}
pub trait Middleware {
    fn exe(&self, req: RequestType) -> RequestType;
    fn priority(&self) -> Priority;
    fn box_clone(&self) -> Box<dyn Middleware>;
}
impl Clone for Box<dyn Middleware> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl MwQueue {
    pub fn new() -> Self {
        MwQueue {
            inner: VecDeque::new(),
            data: RequestType::GetEmpty,
            is_boot: false,
        }
    }
    pub fn boot(&mut self, req: RequestType) -> bool {
        if let Some(head) = self.peek_mut() {
            println!("boot");
            self.data = head.exe(req);
            self.inner.pop_front();
            self.is_boot = true;
        }
        self.dequeue()
    }
    pub fn enqueue(&mut self, item: Box<dyn Middleware>) {
        self.inner.push_back(item);
    }
    pub fn dequeue(&mut self) -> bool {
        println!("dequeue: {:#?}", self.data);
        println!("{}", self.inner.len());
        let data_cloned = self.data.clone();
        if self.is_boot {
            if let Some(cur_midware) = self.inner.pop_front() {
                if let Some(__next_midware) = self.peek() {
                    self.data = cur_midware.exe(data_cloned);
                    self.dequeue();
                    return true;
                } else {
                    self.data = cur_midware.exe(data_cloned);
                    println!("finish");
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
