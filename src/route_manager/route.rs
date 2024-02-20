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
    exeable: bool,
}
pub struct Route {
    addr_vec: Vec<String>,
    root: RouteNode,
}

pub trait Callback {
    fn path(&self) -> String;
    fn call(&self) -> Result<Resp, hyper::Error>;
    fn box_clone(&self) -> Box<dyn Callback>;
}
impl Default for RouteNode {
    fn default() -> Self {
        RouteNode {
            son: HashMap::default(),
            exe: Box::new(DefaultCallback),
            exeable: false,
        }
    }
}
impl Clone for Exe {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct DefaultCallback;
impl Callback for DefaultCallback {
    fn path(&self) -> String {
        "/unknown".to_string()
    }
    fn call(&self) -> Result<Resp, hyper::Error> {
        println!("unknown path -> default");
        Ok::<_, hyper::Error>(Response::new(full(fs::read_to_string("404.html").unwrap())))
    }
    fn box_clone(&self) -> Exe {
        Box::new((*self).clone())
    }
}

impl Route {
    pub fn new() -> Self {
        Route {
            addr_vec: Vec::new(),
            root: RouteNode::default(),
        }
    }

    pub fn insert_path(&mut self, prefix: String) {
        self.addr_vec.push(prefix.clone());
        let prefix_vec = prefix_to_vec(prefix);
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.into_iter() {
            cur_ptr = cur_ptr.son.entry(element).or_insert(RouteNode::default());
        }
        cur_ptr.exeable = true;
        cur_ptr.exe = Box::new(DefaultCallback);
    }
    pub fn insert_exe(&mut self, exe: Exe, prefix: String) -> bool {
        if self.addr_vec().contains(&prefix) {
            let vec = prefix_to_vec(prefix);
            let mut cur_ptr = &mut self.root;
            for ele in vec {
                cur_ptr = cur_ptr.son.entry(ele).or_default();
            }
            cur_ptr.exe = exe;
            return true;
        }
        return false;
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
            if cur_ptr.exeable && element.eq(prefix_vec.last().unwrap()) {
                return (true, res);
            }
        }
        return (true, res);
    }
    pub fn addr_vec(&mut self) -> Vec<String> {
        self.addr_vec.clone()
    }
    pub fn exe_map(&self) -> HashMap<String, &Exe> {
        let mut res = HashMap::new();
        for (path, exe) in self.into_iter() {
            res.insert(path, exe);
        }
        res
    }
}
pub struct ExeIter<'a> {
    stack: Vec<&'a RouteNode>,
}
impl<'a> IntoIterator for &'a Route {
    type Item = (String, &'a Exe);
    type IntoIter = ExeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ExeIter {
            stack: vec![&self.root],
        }
    }
}
impl<'a> Iterator for ExeIter<'a> {
    type Item = (String, &'a Exe);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if node.exeable {
                return Some((node.exe.path(), &node.exe));
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
