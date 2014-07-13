use libc::{c_long, c_double, size_t, c_char};
use base::PyState;


/// Wrapper around the PyObject pointer that the python capi uses.
#[deriving(Show)]
pub struct PyObjectRaw;

#[link(name = "python2.7")]
extern {
  fn Py_Initialize();
  fn Py_Finalize();

  fn PyImport_ImportModule(name : *const c_char) -> *mut PyObjectRaw;

  fn Py_DecRef(obj: *mut PyObjectRaw);

  fn PyObject_CallObject(callable_object : *mut PyObjectRaw, args :*mut PyObjectRaw) -> *mut PyObjectRaw;
  fn PyObject_GetAttrString(object : *mut PyObjectRaw, attr : *const c_char) -> *mut PyObjectRaw;
  fn PyObject_Str(obj: *mut PyObjectRaw) -> *mut PyObjectRaw;

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

  fn PyErr_Fetch(ptype : *mut *mut PyObjectRaw, pvalue : *mut *mut PyObjectRaw, ptraceback : *mut *mut PyObjectRaw);
  fn PyErr_NormalizeException(ptype : *mut *mut PyObjectRaw, pvalue : *mut *mut PyObjectRaw, ptraceback : *mut *mut PyObjectRaw);
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

/// Trait to allow interaction with the python interpreter.
pub trait PythonCAPI {
  #[allow(non_snake_case_functions)]
  unsafe fn Py_Initialize(&self) {
    Py_Initialize();
  }
  #[allow(non_snake_case_functions)]
  unsafe fn Py_Finalize(&self) {
    Py_Finalize();
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyImport_ImportModule(&self, name : *const c_char) -> *mut PyObjectRaw {
    PyImport_ImportModule(name)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyInt_FromLong(&self, ival : c_long) -> *mut PyObjectRaw {
    PyInt_FromLong(ival)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyInt_AsLong(&self, obj : *mut PyObjectRaw) -> c_long {
    PyInt_AsLong(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyFloat_FromDouble(&self, value : c_double) -> *mut PyObjectRaw {
    PyFloat_FromDouble(value)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyFloat_AsDouble(&self, obj : *mut PyObjectRaw) -> c_double {
    PyFloat_AsDouble(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyTuple_New(&self, size : size_t) -> *mut PyObjectRaw {
    PyTuple_New(size)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyTuple_GetItem(&self, tuple : *mut PyObjectRaw, pos : size_t) -> *mut PyObjectRaw {
    PyTuple_GetItem(tuple, pos)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyTuple_SetItem(&self, tuple : *mut PyObjectRaw, pos : size_t, o : *mut PyObjectRaw) {
    PyTuple_SetItem(tuple, pos, o)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyTuple_Size(&self, tuple : *mut PyObjectRaw) -> c_long {
    PyTuple_Size(tuple)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn Py_IncRef(&self, obj: *mut PyObjectRaw) {
    Py_IncRef(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn Py_DecRef(&self, obj: *mut PyObjectRaw) {
    Py_DecRef(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyFloat_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyFloat_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyFloat_CheckExact(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyFloat_CheckExact(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyTuple_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyTuple_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyInt_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyInt_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyString_Check(&self, obj : *mut PyObjectRaw) -> c_long {
    RPyString_Check(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyString_FromString(&self, string : *const c_char) -> *mut PyObjectRaw{
    PyString_FromString(string)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyString_AsString(&self, obj: *mut PyObjectRaw) -> *const c_char {
    PyString_AsString(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyObject_GetAttrString(&self, object : *mut PyObjectRaw, attr : *const c_char) -> *mut PyObjectRaw {
    PyObject_GetAttrString(object, attr)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyErr_Fetch(&self, ptype : *mut *mut PyObjectRaw, pvalue : *mut *mut PyObjectRaw, ptraceback : *mut *mut PyObjectRaw) {
    PyErr_Fetch(ptype, pvalue, ptraceback);
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyErr_NormalizeException(&self, ptype : *mut *mut PyObjectRaw, pvalue : *mut *mut PyObjectRaw, ptraceback : *mut *mut PyObjectRaw) {
    PyErr_NormalizeException(ptype, pvalue, ptraceback);
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyObject_Str(&self, obj: *mut PyObjectRaw) -> *mut PyObjectRaw {
    PyObject_Str(obj)
  }
  #[allow(non_snake_case_functions)]
  unsafe fn PyObject_CallObject(&self, callable_object : *mut PyObjectRaw, args :*mut PyObjectRaw) -> *mut PyObjectRaw {
    PyObject_CallObject(callable_object, args)
  }
}

impl PythonCAPI for PyState {
}
