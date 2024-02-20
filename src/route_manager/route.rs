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

/// alias to keep file tidy
pub type Resp = Response<BoxBody<Bytes, hyper::Error>>;
pub type Exe = Box<dyn Callback>;
/// function in Exe must implement this trait
pub trait Callback {
    /// main function in a Exe
    /// deal with Request translated from the client
    /// also, Middleware will be added
    fn call(&self) -> Result<Resp, hyper::Error>;

    /// illustrate the path that the node locates
    fn path(&self) -> String;
    /// used by the Clone trait
    fn box_clone(&self) -> Box<dyn Callback>;
}
impl Clone for Exe {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Default page to tell visitors that the path is undefined
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

/// the main part of this route-recognizer
/// Simply, it is a complex version of Trie
struct RouteNode {
    /// a map wrap paths and nodes of current node's sons
    son: HashMap<String, RouteNode>,
    /// the wraped function
    exe: Exe,
    /// the flag embodies whether the node is an availible page
    exeable: bool,
}
/// allow you to init a empty Route for debug
impl Default for RouteNode {
    fn default() -> Self {
        RouteNode {
            son: HashMap::default(),
            exe: Box::new(DefaultCallback),
            exeable: false,
        }
    }
}

/// the root
pub struct Route {
    /// normally, it is just a shell with nothing really used
    addr_vec: Vec<String>,
    root: RouteNode,
}
/* IMPORTANT */
impl Route {
    pub fn new() -> Self {
        Route {
            addr_vec: Vec::new(),
            root: RouteNode::default(),
        }
    }
    ///insert a path into the route
    ///
    /// # Arguement
    /// * `path`
    ///
    // Attention: after registering the path, the related exe should be registered
    // TODO: provide more User friendly interfaces to combine these procedures together
    pub fn insert_path(&mut self, path: String) {
        self.addr_vec.push(path.clone());
        let prefix_vec = path_to_vec(path);
        let mut cur_ptr = &mut self.root;
        for element in prefix_vec.into_iter() {
            cur_ptr = cur_ptr.son.entry(element).or_insert(RouteNode::default());
        }
        cur_ptr.exeable = true;
        cur_ptr.exe = Box::new(DefaultCallback);
    }
    /// insert exe into the concrete path in the route
    ///
    /// # Arguement
    /// * `exe`
    /// * `path`
    ///
    /// # Return
    /// whether successful determined by the path
    pub fn insert_exe(&mut self, exe: Exe, path: String) -> bool {
        if self.addr_vec().contains(&path) {
            let vec = path_to_vec(path);
            let mut cur_ptr = &mut self.root;
            for ele in vec {
                cur_ptr = cur_ptr.son.entry(ele).or_default();
            }
            cur_ptr.exe = exe;

            return true;
        }
        return false;
    }
    /// search in the route
    ///
    /// # Arguement
    /// * `path`
    ///
    /// # Return
    /// ## a tuple includes bool and Vec
    /// * bool value shows whether the path is valid
    /// * Vec get the same part
    ///
    /// # Example
    /// a route has the path /example/apple named example_route
    /// ```
    /// let path = String::from("example/banana");
    /// let (is_valid_path, same_part) = example_route.search(path);
    /// ```
    pub fn search(&mut self, path: String) -> (bool, Vec<String>) {
        let mut res = Vec::new();
        let prefix_vec = path_to_vec(path.clone());
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
    /// get a map contains the path and Exe
    ///
    /// # Arguement
    ///
    /// # Return
    /// * a map
    pub fn exe_map(&self) -> HashMap<String, &Exe> {
        let mut res = HashMap::new();

        /* BUG  */
        /* solve 2.20  */
        for (path, exe) in self.into_iter() {
            res.insert(path, exe);
        }
        res
    }
}
/*
 * Iterator for Exe storaged in the RouteNode
 * */

/// adopt the stack-structure to exhaustive the trie-like route regularly
pub struct ExeIter<'a> {
    stack: Vec<&'a RouteNode>,
}
/// generate the Iterator
impl<'a> IntoIterator for &'a Route {
    type Item = (String, &'a Exe);
    type IntoIter = ExeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ExeIter {
            stack: vec![&self.root],
        }
    }
}
/// main part of implementing Iterator
impl<'a> Iterator for ExeIter<'a> {
    type Item = (String, &'a Exe);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            /* BUG - if the node is exeable, it's sons will be ignored  */
            /* solve 2.20  */
            if node.exeable {
                let item = (node.exe.path(), &node.exe);
                for (_, son) in &node.son {
                    self.stack.push(son);
                }
                return Some(item);
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

/// deal with the raw path
///
/// # Arguement
/// * path
///
/// # Return
/// * a vector which storages path without "/"
///
/// # Example
/// ```
/// let path = String::from("raw/example/waiting/for/advanced/treatment");
/// let vector = path_to_vec(path);
/// println!("{:#?}", vector);
/// ```
/// output:
/// [
///     "raw",
///     "example",
///     "waiting",
///     "for",
///     "advanced",
///     "treatment",
/// ]
pub fn path_to_vec(path: String) -> Vec<String> {
    let mut temp = path.clone();
    let mut target = Vec::new();
    while let Some(index) = temp.find("/") {
        if index != 0 {
            target.push(temp[0..index].to_string());
        }
        temp = temp[(index + 1)..temp.len()].to_string();
    }
    target.push(temp);
    target
}
