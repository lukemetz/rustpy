//! A simple library to allow for easy use of python from rust.
//!
//! This library is meant to be middle ware for users wanting to use
//! python libraries from rust. It allows users who want to quickly use exciting
//! tools, at the price of speed, and to get going fast.
//! Originally it was intended to bootstrap machine learning for rust.
//!
//! It provides a way to interact
//! with a python interpreter, via [`PyState`](struct.PyState.html) as well as quick conversion
//! from rust types to python types via the [`ToPyType`](trait.ToPyType.html) and
//! [`FromPyType`](trait.FromPyType.html) traits.
//!
//!
//! ```rust
//! extern crate rustpy;
//! use rustpy::{ToPyType, FromPyType, PyState};
//!
//!
//! fn main() {
//! let py = PyState::new();
//! let module = py.get_module("math").unwrap();
//! let func = module.get_func("sqrt").unwrap();
//! let v = (144f32, );
//! let args = v.to_py_object(&py).unwrap();
//! let untyped_res = func.call(&args).unwrap();
//! let result = py.from_py_object::<f32>(untyped_res).unwrap();
//! assert_eq!(result, 12f32);
//! }
//! ```
//!
#![crate_type = "lib"]

extern crate libc;
#[macro_use]
extern crate lazy_static;

pub use base::{ToPyType, FromPyType, PyState, PyObject, PyObjectRaw, PyError, PyIterator};
pub use primtypes::NoArgs;

mod base;
mod primtypes;
mod ffi;
