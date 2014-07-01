use libc::c_char;
use sync::one::{Once, ONCE_INIT};
use sync::mutex::{StaticMutex, MUTEX_INIT};

static mut PY_START: Once = ONCE_INIT;
pub static mut PY_MUTEX : StaticMutex = MUTEX_INIT;

pub struct PyObjectRaw;

#[link(name = "python2.7")]
extern {
  fn Py_Initialize();

  fn PyImport_ImportModule(name : *const c_char) -> *mut PyObjectRaw;

  fn Py_DecRef(obj: *mut PyObjectRaw);

  fn PyObject_CallObject(callable_object : *mut PyObjectRaw, args :*mut PyObjectRaw) -> *mut PyObjectRaw;
  fn PyObject_GetAttrString(object : *mut PyObjectRaw, attr : *const c_char) -> *mut PyObjectRaw;
}

//#[macro_export]
pub macro_rules! py_lock(
  ($expr:expr) => ({
  let _g = PY_MUTEX.lock();
  let res = $expr;
  res
  })
)

pub fn start_python() {
  unsafe {
    PY_START.doit(|| {
      py_lock!(Py_Initialize());
    });
  }
}

pub struct PyObject {
  pub raw : *mut PyObjectRaw
}


impl PyObject {
  pub fn new(py_object_raw : *mut PyObjectRaw) -> PyObject {
    assert!(py_object_raw.is_not_null());
    PyObject { raw : py_object_raw }
  }

  pub fn get_func(&self, string : &str) -> Result<PyObject, PyError> {
    unsafe {
      let py_func = py_lock!(PyObject_GetAttrString(self.raw, string.to_c_str().unwrap()));
      if py_func.is_null() {
        Err(NullPyObject)
      } else {
        Ok(PyObject::new(py_func))
      }
    }
  }

  pub fn call (&self, py_object : &PyObject) -> Result<PyObject, PyError>{
    unsafe {
      let py_ret = py_lock!(PyObject_CallObject(self.raw, py_object.raw));
      if py_ret.is_null() {
        Err(NullPyObject)
      } else {
        Ok(PyObject::new(py_ret))
      }
    }
  }
}

impl Drop for PyObject {
  fn drop(&mut self) {
    unsafe {
      Py_DecRef(self.raw);
    }
  }
}

#[deriving(Show)]
pub enum PyError {
  FromTypeConversionError,
  ToTypeConversionError,
  StringConversionError,
  NullPyObject,
}

pub trait PyType {
  fn to_py_object(&self) -> Result<PyObject, PyError>;
  fn from_py_object(py_object : PyObject) -> Result<Self, PyError>;
}

pub fn get_module(module_name : &str) -> Result<PyObject, PyError> {
  unsafe {
    let string = module_name.to_c_str().unwrap();
    let py_module = py_lock!(PyImport_ImportModule(string));
    if py_module.is_null() {
      Err(NullPyObject)
    } else {
      Ok(PyObject::new(py_module))
    }
  }
}

#[cfg(test)]
mod test {
  use super::{get_module, start_python};
  use primtypes::{PyType, from_py_object};
  macro_rules! try_or_fail (
      ($e:expr) => (match $e { Ok(e) => e, Err(e) => fail!("{}", e) })
  )

  #[test]
  fn test_get_module() {
    start_python();
    let pyobj = get_module("math");
    match pyobj {
      Err(_) => fail!("Failed to import math"),
      Ok(x) => assert!(x.raw.is_not_null())
    }
  }

  #[test]
  fn math_sqrt() {
    start_python();
    let module = try_or_fail!(get_module("math"));
    let func = try_or_fail!(module.get_func("sqrt"));
    let arg = try_or_fail!((144f32,).to_py_object());
    let py_result = try_or_fail!(func.call(&arg));
    let result = try_or_fail!(from_py_object::<f32>(py_result));
    assert_eq!(result, 12f32);
  }

  #[test]
  fn math_pow() {
    start_python();
    let module = try_or_fail!(get_module("math"));
    let func = try_or_fail!(module.get_func("pow"));
    let arg = try_or_fail!((3f32, 2f32).to_py_object());
    let py_result = try_or_fail!(func.call(&arg));
    let result = try_or_fail!(from_py_object::<f32>(py_result));
    assert_eq!(result, 9f32);
  }

}
