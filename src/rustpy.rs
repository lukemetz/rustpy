#![crate_id = "rustpy"]
#![crate_type = "lib"]
#![feature(macro_rules)]
#![feature(link_args)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate sync;
extern crate debug;
extern crate alloc;

pub mod base;
pub mod primtypes;
