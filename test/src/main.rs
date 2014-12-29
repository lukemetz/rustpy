extern crate rustpy;
use rustpy::{ToPyType, FromPyType, PyState};

fn main() 
{
	let py = PyState::new();
	let module = py.get_module("math").unwrap();
	let func = module.get_func("sqrt").unwrap();
	let number = (144f32, );
	let args = (&number).to_py_object(&py).unwrap();
	let untyped_res = func.call(&args).unwrap();
	let result = py.from_py_object::<f32>(untyped_res).unwrap();
	assert_eq!(result, 12f32);
	println!("result: {}", result);
}
