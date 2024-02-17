use super::super::hyper_manager::server::full;
use std::collections::HashMap;
use std::fs;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::Response;
/*
 * based on Trie
 * spend a bit more memory to gain a rapid find_method
 * */

pub type Resp = Response<BoxBody<Bytes, hyper::Error>>;
pub type Exe = Box<dyn Callback>;
struct RouteNode {
    son: HashMap<String, RouteNode>,
    exe: Exe,
    can_exe: bool,
}
pub struct Route {
    addr_vec: Vec<String>,
    root: RouteNode,
}
pub trait Callback {
    fn call(&self) -> Result<Resp, hyper::Error>;
}
impl Default for RouteNode {
    fn default() -> Self {
        RouteNode {
            son: HashMap::default(),
            exe: Box::new(DefaultCallback),
            can_exe: false,
        }
    }
}
pub struct ExeIter<'a> {
    stack: Vec<&'a RouteNode>,
}
impl<'a> IntoIterator for &'a Route {
    type Item = &'a Exe;
    type IntoIter = ExeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ExeIter {
            stack: vec![&self.root],
        }
    }
}
impl<'a> Iterator for ExeIter<'a> {
    type Item = &'a Exe;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if node.can_exe {
                return Some(&node.exe);
            }
            let k: Vec<&String> = node.son.keys().collect();
            for i in (0..k.len()).rev() {
                if let Some(son) = node.son.get(k[i]) {
                    self.stack.push(son);
                }
            }
        }
        None
    }
}
pub struct DefaultCallback;
impl Callback for DefaultCallback {
    fn call(&self) -> Result<Resp, hyper::Error> {
        Ok::<_, hyper::Error>(Response::new(full(
            fs::read_to_string("hello.html").unwrap(),
        )))
    }
}

impl Route {
    pub fn new() -> Self {
        Route {
            addr_vec: Vec::new(),
            root: RouteNode::default(),
        }
    }

    pub fn insert(&mut self, prefix: String) {
        self.addr_vec.push(prefix.clone());
        let prefix_vec = prefix_to_vec(prefix);
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.into_iter() {
            cur_ptr = cur_ptr.son.entry(element).or_insert(RouteNode::default());
        }
        cur_ptr.can_exe = true;
    }
    pub fn search(&mut self, prefix: String) -> (bool, Vec<String>) {
        let mut res = Vec::new();
        let prefix_vec = prefix_to_vec(prefix.clone());
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.clone().into_iter() {
            cur_ptr = if cur_ptr.son.contains_key(&element) {
                res.push(element.clone());
                cur_ptr.son.entry(element.clone()).or_default()
            } else {
                return (false, res);
            };
            if cur_ptr.can_exe && element.eq(prefix_vec.last().unwrap()) {
                return (true, res);
            }
        }
        return (true, res);
    }
    pub fn addr_vec(&mut self) -> Vec<String> {
        self.addr_vec.clone()
    }
}

fn prefix_to_vec(prefix: String) -> Vec<String> {
    let mut temp = prefix.clone();
    let mut target = Vec::new();
    while let Some(index) = temp.find("/") {
        // println!("before: vec = {:#?}\n\rindex = {}", target, index);
        // println!("before: str = {:#?}", temp);
        target.push(temp[0..index].to_string());
        temp = temp[(index + 1)..temp.len()].to_string();
        // println!("after: str = {:#?}", temp);
        // println!("after: str = {:#?}", target);
    }
    target.push(temp);
    target
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        let mut route = Route::new();
        route.insert("112/22/888/aaa".to_string());
        route.insert("112/22/8".to_string());
        route.insert("112/1".to_string());
        let vec = route.addr_vec();
        assert_eq!(vec[1], "112/22/888/aaa");
        assert_eq!(vec[0], "112/22/8");
        assert_eq!(vec[2], "112/1");
    }
}
