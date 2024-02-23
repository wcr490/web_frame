use super::*;
// alias to keep file tidy
/// Specially used by Config to implement Clone
pub struct Cb(pub Box<dyn Callback>);
///neccessary trait due to multithreading
unsafe impl Sync for Cb {}
unsafe impl Send for Cb {}

/// A macro used to automatically register a struct which has been implemented Callback
///
/// # Parameter
/// * $name - name of the struct
/// * $path - path in the route
/// * $body - Exe content
///
/// # Return
/// * struct $name
#[macro_export]
macro_rules! exe_generator {
    () => {};

    ($name: ident, $path: expr, $method: expr, $body: block) => {
        #[derive(Clone)]
        pub struct $name;
        impl Callback for $name {
            fn call(&self) -> Result<Resp, hyper::Error> {
                $body
            }
            fn method(&self) -> Method {
                $method
            }
            fn path(&self) -> String {
                $path
            }
            fn box_clone(&self) -> Exe {
                Box::new((*self).clone())
            }
        }
    };
}
