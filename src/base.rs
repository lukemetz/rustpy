use libc::{c_long, c_double, size_t, c_char};
use sync::mutex::{StaticMutex, MUTEX_INIT, Guard};

static mut PY_MUTEX : StaticMutex = MUTEX_INIT;

pub struct PyObjectRaw;

#[link(name = "python2.7")]
extern {
  fn Py_Initialize();
  fn Py_Finalize();

  fn PyImport_ImportModule(name : *const c_char) -> *mut PyObjectRaw;

  fn Py_DecRef(obj: *mut PyObjectRaw);

  fn PyObject_CallObject(callable_object : *mut PyObjectRaw, args :*mut PyObjectRaw) -> *mut PyObjectRaw;
  fn PyObject_GetAttrString(object : *mut PyObjectRaw, attr : *const c_char) -> *mut PyObjectRaw;

  fn PyInt_FromLong(ival : c_long) -> *mut PyObjectRaw;
  fn PyInt_AsLong(obj : *mut PyObjectRaw) -> c_long;

  fn PyFloat_FromDouble(value : c_double) -> *mut PyObjectRaw;
  fn PyFloat_AsDouble(obj : *mut PyObjectRaw) -> c_double;

  fn PyTuple_New(size : size_t) -> *mut PyObjectRaw;
  fn PyTuple_GetItem(tuple : *mut PyObjectRaw, pos : size_t) -> *mut PyObjectRaw;
  fn PyTuple_SetItem(tuple : *mut PyObjectRaw, pos : size_t, o : *mut PyObjectRaw);
  fn PyTuple_Size(tuple : *mut PyObjectRaw) -> c_long;

  fn PyString_FromString(string : *const c_char) -> *mut PyObjectRaw;
  fn PyString_AsString(obj: *mut PyObjectRaw) -> *const c_char;

  fn Py_IncRef(obj: *mut PyObjectRaw);
}

#[link(name = "python2.7")]
#[link(name = "macroexpand", kind = "static")]
extern {
  fn RPyFloat_Check(obj : *mut PyObjectRaw) -> c_long;
  fn RPyFloat_CheckExact(obj : *mut PyObjectRaw) -> c_long;
  fn RPyTuple_Check(obj : *mut PyObjectRaw) -> c_long;
  fn RPyInt_Check(obj : *mut PyObjectRaw) -> c_long;
  fn RPyString_Check(obj : *mut PyObjectRaw) -> c_long;
}

/// Struct to control interaction with the python interpreter.
///
/// There can only be one active PyState at a time, as on initialization
/// a shared mutex gets locked. This allows for safe-ish execution of
/// python at the cost of increased risk of deadlocks.

pub struct PyState {
  #[allow(dead_code)]
  guard : Guard<'static>
}

impl PyState {
  /// Get a new instance of the python interpreter.
  pub fn new() -> PyState {
    unsafe {
      let guard = PY_MUTEX.lock();
      Py_Initialize();
      PyState { guard : guard }
    }
  }

  /// Return the PyObject at the associated name. Will `Err` if no module found.
  pub fn get_module<'a>(&'a self, module_name : &str) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let string = module_name.to_c_str().unwrap();
      let py_module = PyImport_ImportModule(string);
      if py_module.is_not_null() {
        Ok(PyObject::new(self, py_module))
      } else {
        Err(NullPyObject)
      }
    }
  }

  /// Helper function to convert `PyObject` back to rust types.
  pub fn from_py_object<A : PyType>(&self, obj : PyObject) -> Result<A, PyError> {
    PyType::from_py_object(self, obj)
  }

  #[allow(non_snake_case_functions)]
  pub unsafe fn PyInt_FromLong(&self, ival : c_long) -> *mut PyObjectRaw {
    PyInt_FromLong(ival)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyInt_AsLong(&self, obj : *mut PyObjectRaw) -> c_long {
    PyInt_AsLong(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyFloat_FromDouble(&self, value : c_double) -> *mut PyObjectRaw {
    PyFloat_FromDouble(value)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyFloat_AsDouble(&self, obj : *mut PyObjectRaw) -> c_double {
    PyFloat_AsDouble(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyTuple_New(&self, size : size_t) -> *mut PyObjectRaw {
    PyTuple_New(size)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyTuple_GetItem(&self, tuple : *mut PyObjectRaw, pos : size_t) -> *mut PyObjectRaw {
    PyTuple_GetItem(tuple, pos)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyTuple_SetItem(&self, tuple : *mut PyObjectRaw, pos : size_t, o : *mut PyObjectRaw) {
    PyTuple_SetItem(tuple, pos, o)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyTuple_Size(&self, tuple : *mut PyObjectRaw) -> c_long {
    PyTuple_Size(tuple)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn Py_IncRef(&self, obj: *mut PyObjectRaw) {
    Py_IncRef(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn Py_DecRef(&self, obj: *mut PyObjectRaw) {
    Py_DecRef(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyFloat_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyFloat_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyFloat_CheckExact(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyFloat_CheckExact(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyTuple_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyTuple_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyInt_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyInt_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyString_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyString_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyString_FromString(&self, string : *const c_char) -> *mut PyObjectRaw{
    PyString_FromString(string)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyString_AsString(&self, obj: *mut PyObjectRaw) -> *const c_char {
    PyString_AsString(obj)
  }
  #[allow(non_snake_case_functions)]
  pub unsafe fn PyObject_GetAttrString(&self, object : *mut PyObjectRaw, attr : *const c_char) -> *mut PyObjectRaw {
    PyObject_GetAttrString(object, attr)
  }
}

impl Drop for PyState {
  fn drop(&mut self) {
    unsafe {
      Py_Finalize();
    }
  }
}

/// Wrapper around python PyObject.

pub struct PyObject<'a> {
  pub state : &'a PyState,
  pub raw : *mut PyObjectRaw
}

impl<'a> PyObject<'a> {
  /// Wrap a raw PyObject pointer. Should not be called by user
  pub fn new(state : &'a PyState, py_object_raw : *mut PyObjectRaw) -> PyObject<'a> {
    assert!(py_object_raw.is_not_null());
    PyObject { state : state, raw : py_object_raw }
  }

  pub fn empty_tuple(state :&'a PyState) -> PyObject<'a> {
    unsafe {
      let raw = state.PyTuple_New(0);
      PyObject::new(state, raw)
    }
  }

  /// Get PyObject corresponding to a function
  pub fn get_func(&self, string : &str) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let py_func = self.state.PyObject_GetAttrString(self.raw, string.to_c_str().unwrap());
      if py_func.is_null() {
        Err(NullPyObject)
      } else {
        Ok(PyObject::new(self.state, py_func))
      }
    }
  }

  /// Call a PyObject with the tuple provided in `args`
  pub fn call(&self, args: &PyObject) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let py_ret = PyObject_CallObject(self.raw, args.raw);
      if py_ret.is_null() {
        Err(NullPyObject)
      } else {
        Ok(PyObject::new(self.state, py_ret))
      }
    }
  }

  /// Helper function to call returning type
  pub fn call_with_ret<'a, T : PyType>(&'a self, args: &PyObject) -> Result<T, PyError> {
    self.call(args).and_then(|x| {
      self.state.from_py_object::<T>(x)
    })
  }
}

#[unsafe_destructor]
impl<'a> Drop for PyObject<'a> {
  fn drop(&mut self) {
    unsafe {
     self.state.Py_DecRef(self.raw);
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
  fn to_py_object<'a>(&self, state : &'a PyState) -> Result<PyObject<'a>, PyError>;
  fn from_py_object(state : &PyState, py_object : PyObject) -> Result<Self, PyError>;
}

#[cfg(test)]
mod test {
  use super::PyState;
  use primtypes::{PyType, PyObject};
  macro_rules! try_or_fail (
      ($e:expr) => (match $e { Ok(e) => e, Err(e) => fail!("{}", e) })
  )

  #[test]
  fn test_empty_tuple_should_not_fail() {
    let py = PyState::new();
    let _ = PyObject::empty_tuple(&py);
  }

  #[test]
  fn test_get_module() {
    let py = PyState::new();
    let pyobj = py.get_module("math");
    match pyobj {
      Err(_) => fail!("Failed to import math"),
      Ok(x) => assert!(x.raw.is_not_null())
    }
  }

  #[test]
  fn math_sqrt() {
    let py = PyState::new();
    let module = try_or_fail!(py.get_module("math"));
    let func = try_or_fail!(module.get_func("sqrt"));
    let arg = try_or_fail!((144f32,).to_py_object(&py));
    let py_result = try_or_fail!(func.call(&arg));
    let result = try_or_fail!(py.from_py_object::<f32>(py_result));
    assert_eq!(result, 12f32);
  }

  #[test]
  fn math_pow() {
    let py = PyState::new();
    let module = try_or_fail!(py.get_module("math"));
    let func = try_or_fail!(module.get_func("pow"));
    let arg = try_or_fail!((3f32, 2f32).to_py_object(&py));
    let py_result = try_or_fail!(func.call(&arg));
    let result = try_or_fail!(py.from_py_object::<f32>(py_result));
    assert_eq!(result, 9f32);
  }

  #[test]
  fn test_call_with_ret() {
    let py = PyState::new();
    let module = try_or_fail!(py.get_module("math"));
    let func = try_or_fail!(module.get_func("pow"));
    let arg = try_or_fail!((3f32, 2f32).to_py_object(&py));
    let result = try_or_fail!(func.call_with_ret::<f32>(&arg));
    assert_eq!(result, 9f32);
  }
}
