/*
 * based on Trie
 * spend a bit more memory to gain a rapid find_method
 * */

use std::collections::HashMap;

#[derive(Default)]
struct RouteNode {
    son: HashMap<String, RouteNode>,
    is_leaf: bool,
}
pub struct Route {
    root: RouteNode,
}

impl Route {
    pub fn new() -> Self {
        Route {
            root: RouteNode::default(),
        }
    }

    pub fn insert(&mut self, prefix: String) {
        let prefix_vec = prefix_to_vec(prefix);
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.into_iter() {
            cur_ptr = cur_ptr.son.entry(element).or_insert(RouteNode::default());
        }
        cur_ptr.is_leaf = true;
    }
    pub fn search(&mut self, prefix: String) -> (bool, Vec<String>) {
        let mut res = Vec::new();
        let prefix_vec = prefix_to_vec(prefix);
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.into_iter() {
            cur_ptr = if cur_ptr.son.contains_key(&element) {
                res.push(element.clone());
                cur_ptr.son.entry(element).or_default()
            } else {
                return (false, res);
            };
        }
        return (true, res);
    }
}

pub fn prefix_to_vec(prefix: String) -> Vec<String> {
    let mut temp = prefix.clone();
    let mut target = Vec::new();
    while let Some(index) = temp.find("/") {
        target.push(temp[0..index - 1].to_string());
        temp = temp[index + 1..temp.len()].to_string();
    }
    target
}
