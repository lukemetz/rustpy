use sync::mutex::{StaticMutex, MUTEX_INIT, Guard};
use std::ptr;
use std::mem::transmute;
use std::fmt;
pub use ffi::{PythonCAPI, PyObjectRaw};

static mut PY_MUTEX : StaticMutex = MUTEX_INIT;

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
      let state = PyState { guard : guard };
      state.Py_Initialize();
      state
    }
  }

  /// Return the PyObject at the associated name. Will `Err` if no module found.
  pub fn get_module<'a>(&'a self, module_name : &str) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let string = module_name.to_c_str().unwrap();
      let py_module = self.PyImport_ImportModule(string);

      let exception = self.get_result_exception();

      if exception.is_err() {
        Err(exception.err().unwrap())
      } else if py_module.is_not_null() {
        Ok(PyObject::new(self, py_module))
      } else {
        Err(PyError::NullPyObject)
      }
    }
  }

  /// Helper function to convert `PyObject` back to rust types.
  pub fn from_py_object<A : FromPyType>(&self, obj : PyObject) -> Result<A, PyError> {
    FromPyType::from_py_object(self, obj)
  }

  /// Low level function to check for python inturpreter errors
  pub fn get_result_exception(&self) -> Result<(), PyError> {
    unsafe {
      let ptype : *mut PyObjectRaw = ptr::null_mut();
      let pvalue : *mut PyObjectRaw = ptr::null_mut();
      let ptraceback : *mut PyObjectRaw = ptr::null_mut();
      self.PyErr_Fetch(transmute(&ptype),
                       transmute(&pvalue),
                       transmute(&ptraceback));
      self.PyErr_NormalizeException(transmute(&ptype),
                       transmute(&pvalue),
                       transmute(&ptraceback));
      if pvalue.is_null() {
        Ok(())
      } else {
        let base = PyObject::new(self, self.PyObject_Str(pvalue));
        let error_type_string = self.PyObject_GetAttrString(ptype, "__name__".to_c_str().unwrap());
        let error_type = PyObject::new(self, error_type_string);
        let base_string = self.from_py_object::<String>(base).unwrap();
        let error_type_string = self.from_py_object::<String>(error_type).unwrap();
        Err(PyError::PyException(error_type_string + " : ".to_string() + base_string))
      }
    }
  }
}

impl Drop for PyState {
  fn drop(&mut self) {
    // This is a bug. Numpy should properly clean up after itself but it doesnt.
    // This will continue to allow for multiple PyState, but will probably
    // cause memory leaks.
    //unsafe {
      //self.Py_Finalize();
    //}
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

  /// Constructor for empty PyObject tuple for void functions
  pub fn empty_tuple(state :&'a PyState) -> PyObject<'a> {
    unsafe {
      let raw = state.PyTuple_New(0);
      PyObject::new(state, raw)
    }
  }

  /// Get PyObject corresponding to a function
  pub fn get_func(&self, string : &str) -> Result<PyObject<'a>, PyError> {
      self.get_member_obj(string)
  }

  /// Get a member variable as PyObject
  pub fn get_member_obj(&self, name: &str) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let py_member = self.state.PyObject_GetAttrString(self.raw, name.to_c_str().unwrap());
      let exception = self.state.get_result_exception();
      if exception.is_err() {
        Err(exception.err().unwrap())
      } else if py_member.is_null() {
        Err(PyError::NullPyObject)
      } else {
        Ok(PyObject::new(self.state, py_member))
      }
    }
  }

  /// Get member variable as native type
  pub fn get_member<T : FromPyType>(&self, name: &str) -> Result<T, PyError> {
    self.get_member_obj(name).and_then(|x| {
      self.state.from_py_object(x)
    })
  }

  /// Call a PyObject with the tuple provided in `args`
  pub fn call(&self, args: &PyObject) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let py_ret = self.state.PyObject_CallObject(self.raw, args.raw);
      let exception = self.state.get_result_exception();
      if exception.is_err() {
        Err(exception.err().unwrap())
      } else if py_ret.is_null() {
        Err(PyError::NullPyObject)
      } else {
        Ok(PyObject::new(self.state, py_ret))
      }
    }
  }

  /// Helper function to call returning type
  pub fn call_with_ret<'a, T : FromPyType>(&'a self, args: &PyObject) -> Result<T, PyError> {
    self.call(args).and_then(|x| {
      self.state.from_py_object::<T>(x)
    })
  }

  pub fn call_func<'a, I : ToPyType>(&'a self, name : &str, args : I) -> Result<PyObject<'a>, PyError> {
    self.get_func(name).and_then(|x| {
      args.to_py_object(self.state).and_then(|input| {
          x.call(&input)
      })
    })
  }

  pub fn call_func_with_ret<'a, I : ToPyType, R : FromPyType>(&'a self, name : &str, args : I) -> Result<R, PyError> {
    self.get_func(name).and_then(|x| {
      args.to_py_object(self.state).and_then(|input| {
          x.call_with_ret(&input)
      })
    })
  }

  fn get_iter<'a>(&'a self) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let py_iter = self.state.PyObject_GetIter(self.raw);
      if py_iter.is_null() {
        Err(PyError::NullPyObject)
      } else {
        Ok(PyObject::new(self.state, py_iter))
      }
    }
  }

  /// Get a Rust iterator wrapping a Python iterator.
  ///
  /// Returns an error if this object does not implement the Python
  /// iterator protocol.
  pub fn iter<'a, T : FromPyType>(&'a self) -> Result<PyIterator<'a, T>, PyError> {
    self.get_iter().and_then(|py_object|
      PyIterator::new(py_object)
    )
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

// TODO this seems unsafe / bad. Should probably shift to ARC or something
impl<'a> Clone for PyObject<'a> {
  fn clone(&self) -> PyObject<'a> {
    unsafe {
      self.state.Py_IncRef(self.raw);
    }
    PyObject::new(self.state, self.raw.clone())
  }
}

impl<'a> fmt::Show for PyObject<'a> {
  fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {
    unsafe {
      let string = self.state.PyObject_Str(self.raw);
      let result = self.state.from_py_object::<String>(PyObject::new(self.state, string)).unwrap();
      write!(fmt, "PyObject{{{}}}", result)
    }
  }
}

/// Possible errors while using rustpy
///
/// Generally speaking, all errors are from this library or user
/// interaction with this library such as passing in wrong types of PyObject.
/// The PyExecption error is an exception from python that causes a function or
/// operation to fail.
#[deriving(Show)]
pub enum PyError {
  FromTypeConversionError,
  ToTypeConversionError,
  StringConversionError,
  PyException(String),
  NullPyObject,
  NotAnIterator,
}

/// Rust type that can be converted to a Python object
pub trait ToPyType {
  fn to_py_object<'a>(&'a self, state : &'a PyState) -> Result<PyObject<'a>, PyError>;
}

/// Rust type that can be extracted from a Python object
pub trait FromPyType {
  fn from_py_object<'a>(state : &'a PyState, py_object : PyObject<'a>) -> Result<Self, PyError>;
}

/// Wrapper around a Python iterator
pub struct PyIterator<'a, T> { py_object : PyObject<'a> }

impl<'a, T : FromPyType> PyIterator<'a, T> {
  /// Create Rust iterator from a Python object implementing the
  /// iterator protocol.
  pub fn new(obj : PyObject<'a>) -> Result<PyIterator<'a, T>, PyError> {
    unsafe {
      if obj.state.PyIter_Check(obj.raw) != 0 {
        Ok(PyIterator { py_object : obj })
      } else {
        Err(PyError::NotAnIterator)
      }
    }
  }

  fn next_py_object(&mut self) -> Option<PyObject<'a>> {
    unsafe {
      let py_next = self.py_object.state.PyIter_Next(self.py_object.raw);
      if py_next.is_null() {
        None
      } else {
        Some(PyObject::new(self.py_object.state, py_next))
      }
    }
  }
}

impl<'a, T : FromPyType> Iterator<Result<T, PyError>> for PyIterator<'a, T> {
  fn next(&mut self) -> Option<Result<T, PyError>> {
    self.next_py_object().and_then(|py_object|
      Some(self.py_object.state.from_py_object(py_object))
    )
  }
}

#[cfg(test)]
mod test {
  use super::PyState;
  use primtypes::{ToPyType, FromPyType, PyObject};
  use super::PyError;
  macro_rules! try_or_panic (
      ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
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
      Err(_) => panic!("Failed to import math"),
      Ok(x) => assert!(x.raw.is_not_null())
    }
  }

  #[test]
  fn math_sqrt() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let func = try_or_panic!(module.get_func("sqrt"));
    let input = (144f32,);
    let arg = try_or_panic!(input.to_py_object(&py));
    let py_result = try_or_panic!(func.call(&arg));
    let result = try_or_panic!(py.from_py_object::<f32>(py_result));
    assert_eq!(result, 12f32);
  }

  #[test]
  fn math_pow() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let func = try_or_panic!(module.get_func("pow"));
    let input = (3f32, 2f32);
    let arg = try_or_panic!(input.to_py_object(&py));
    let py_result = try_or_panic!(func.call(&arg));
    let result = try_or_panic!(py.from_py_object::<f32>(py_result));
    assert_eq!(result, 9f32);
  }

  #[test]
  fn test_exceptions_module() {
    let py = PyState::new();
    let module = py.get_module("mathSpelledWrong");
    match module {
      Ok(_) => panic!("Did not return Err"),
      Err(PyError::PyException(s)) => assert_eq!(s.as_slice(), "ImportError : No module named mathSpelledWrong"),
      Err(e) => panic!("Got unexpected error: {}", e)
    };
  }

  #[test]
  fn test_exceptions_function_lookup() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let func = module.get_func("powMissSpelled");
    match func {
      Ok(_) => panic!("Did not return Err"),
      Err(PyError::PyException(s)) => assert_eq!(s.as_slice(), "AttributeError : 'module' object has no attribute 'powMissSpelled'"),
      Err(e) => panic!("Got unexpected error: {}", e)
    };
  }

  #[test]
  fn test_exceptions_function_call() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let func = try_or_panic!(module.get_func("pow"));
    let input = (3f32, 2f32, 314i);
    let badarg = try_or_panic!(input.to_py_object(&py));
    let res = func.call(&badarg);
    match res {
      Ok(_) => panic!("Did not return Err"),
      Err(PyError::PyException(s)) => assert_eq!(s.as_slice(), "TypeError : pow expected 2 arguments, got 3"),
      Err(e) => panic!("Got unexpected error: {}", e)
    };
  }

  #[test]
  fn test_call_with_ret() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let func = try_or_panic!(module.get_func("pow"));
    let input = (3f32, 2f32);
    let arg = try_or_panic!(input.to_py_object(&py));
    let result = try_or_panic!(func.call_with_ret::<f32>(&arg));
    assert_eq!(result, 9f32);
  }

  #[test]
  fn test_call_func() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let obj = try_or_panic!(module.call_func("pow", (3f32, 2f32)));
    let result = try_or_panic!(py.from_py_object::<f32>(obj));

    assert_eq!(result, 9f32);
  }

  #[test]
  fn test_call_func_with_ret() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let result : f32 = try_or_panic!(module.call_func_with_ret("pow", (3f32, 2f32)));
    assert_eq!(result, 9f32);
  }

  #[test]
  fn test_get_member() {
    let py = PyState::new();
    let module = try_or_panic!(py.get_module("math"));
    let result : f32 = try_or_panic!(module.get_member("pi"));
    assert!(result - 3.141593 < 0.001);
  }

  #[test]
  fn test_py_object_show() {
    let py = PyState::new();
    assert_eq!(format!("{}", (1i, 2f32).to_py_object(&py).unwrap()), "PyObject{(1, 2.0)}".to_string());
  }

  #[test]
  fn iterate_list() {
    let val = vec!(1i,2,3);
    let py = PyState::new();
    let py_object = try_or_panic!(val.to_py_object(&py));
    let returned = try_or_panic!(py_object.iter()).map(|x| x.unwrap()).collect::<Vec<int>>();
    assert_eq!(returned, val);
  }

  #[test]
  fn iterate_combinations() {
    let py = PyState::new();
    let itertools = try_or_panic!(py.get_module("itertools"));
    let comb = try_or_panic!(itertools.call_func("combinations", (vec![1i,2,3,4], 2i)));
    let result = try_or_panic!(comb.iter()).map(|x| x.unwrap()).collect::<Vec<(int, int)>>();
    assert_eq!(vec![(1i,2),(1,3),(1,4),(2,3),(2,4),(3,4)], result);
  }

  #[test]
  fn iterate_count() {
    let py = PyState::new();
    let itertools = try_or_panic!(py.get_module("itertools"));
    let perm = try_or_panic!(itertools.call_func("combinations", (vec![1i,2,3,4], 2i)));
    let count = try_or_panic!(perm.iter::<(int, int)>()).count();
    assert_eq!(6, count);
  }
}
