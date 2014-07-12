/*!
A simple library to allow for easy use of python from rust.

This library is meant to be middle ware for users wanting to use
python libraries from rust. It allows users who want to quickly use exciting
tools, at the price of speed, and to get going fast.
Originally it was intended to bootstrap machine learning for rust.

It provides a way to interact
with a python interpreter, via [`PyState`](struct.PyState.html) as well as quick conversion
from rust types to python types via the [`PyType`](trait.PyType) trait.


```rust
extern crate rustpy;
use rustpy::{PyType, PyState};

fn main() {
  let py = PyState::new();
  let module = try!(py.get_module("math"));
  let func = try!(module.get_func("sqrt"));
  let args = try!((144f32, ).to_py_object(&py));
  let untyped_res = try!(func.call(&args));
  let result = try!(py.from_py_object::<f32>(untyped_res));
  assert_eq!(result, 12f32);
}
```
*/
#![crate_name = "rustpy"]
#![crate_type = "lib"]
#![feature(macro_rules)]
#![feature(link_args)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate sync;
extern crate debug;
extern crate alloc;

pub use base::{PyType, PyState, PyObject, PyObjectRaw, PyError};

mod base;
mod primtypes;
