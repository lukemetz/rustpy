use libc::{c_long, size_t};
use std::c_str::CString;
pub use base::{PyObject, ToPyType, FromPyType, PyState, PyIterator};
pub use ffi::{PythonCAPI, PyObjectRaw};
pub use base::PyError;

macro_rules! prim_pytype (
  ($base_type:ty, $cast_type:ty, $to:ident, $back:ident, $check:ident) => (
    impl ToPyType for $base_type {
      fn to_py_object<'a>(&self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
        unsafe {
          let raw = state.$to(*self as $cast_type);
          if raw.is_not_null() && state.$check(raw) > 0 {
            Ok(PyObject::new(state, raw))
          } else {
            Err(PyError::ToTypeConversionError)
          }
        }
      }
    }

    impl FromPyType for $base_type {
      fn from_py_object(state : &PyState, py_object : PyObject) -> Result<$base_type, PyError>  {
        unsafe {
          if py_object.raw.is_not_null() && state.$check(py_object.raw) > 0 {
            Ok(state.$back(py_object.raw) as $base_type)
          } else {
            Err(PyError::FromTypeConversionError)
          }
        }
      }
    }
  )
)

prim_pytype!(f64, f64, PyFloat_FromDouble, PyFloat_AsDouble, PyFloat_Check)
prim_pytype!(f32, f64, PyFloat_FromDouble, PyFloat_AsDouble, PyFloat_Check)
prim_pytype!(i64, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(i32, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(int, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(uint, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(u8, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(u32, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)
prim_pytype!(u64, c_long, PyInt_FromLong, PyInt_AsLong, PyInt_Check)

macro_rules! tuple_pytype ({$length:expr,$(($refN:ident, $n:expr, $T:ident)),+} => (
  impl<$($T:ToPyType),+> ToPyType for ($($T,)+) {
    fn to_py_object<'a>(&self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
      $(let $refN = self.$refN();)+
      unsafe {
        let raw = state.PyTuple_New($length);
        $(let $refN = try!($refN.to_py_object(state));)+
        $(state.Py_IncRef($refN.raw);)+
        $(state.PyTuple_SetItem(raw, $n, $refN.raw);)+

        if raw.is_not_null() {
          Ok(PyObject::new(state, raw))
        } else {
          Err(PyError::ToTypeConversionError)
        }
      }
    }
  }

  impl<$($T:FromPyType),+> FromPyType for ($($T,)+) {
    fn from_py_object(state : &PyState, py_object : PyObject) -> Result<($($T,)+), PyError>  {
      unsafe {
        if py_object.raw.is_null() && state.PyTuple_Check(py_object.raw) > 0 {
          Err(PyError::FromTypeConversionError)
        } else {
          let raw = py_object.raw;
          if state.PyTuple_Size(raw) == $length {
            $(let $refN = state.PyTuple_GetItem(raw, $n);)+
            //TODO is there a better way to do this check?
            let no_null = vec!($($refN.is_not_null(), ) +).iter().all(|&x| x);
            if no_null {
              $(let $refN = PyObject::new(state, $refN);)+
              $(let $refN = try!(state.from_py_object::<$T>($refN));)+
              Ok(($($refN,)+))
            } else {
              Err(PyError::ToTypeConversionError)
            }
          } else {
              Err(PyError::ToTypeConversionError)
          }
        }
      }
    }
  }
))

impl<T: ToPyType> ToPyType for Vec<T> {
  fn to_py_object<'a>(&'a self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let raw = state.PyList_New(self.len() as size_t);
      for (i, item) in self.iter().enumerate() {
        let pyitem = try!(item.to_py_object(state));
        state.Py_IncRef(pyitem.raw);
        state.PyList_SetItem(raw, i as size_t, pyitem.raw);
      }
      if raw.is_not_null() {
        Ok(PyObject::new(state, raw))
      } else {
        Err(PyError::ToTypeConversionError)
      }
    }
  }
}

impl<T: FromPyType> FromPyType for Vec<T> {
  fn from_py_object(state : &PyState, py_object : PyObject) -> Result<Vec<T>, PyError> {
    unsafe {
      if py_object.raw.is_not_null() && state.PyList_Check(py_object.raw) > 0 {
        let raw = py_object.raw;
        let size = state.PyList_Size(raw) as uint;
        let mut v = Vec::with_capacity(size);
        for i in range(0, size) {
          let rawitem = state.PyList_GetItem(raw, i as size_t);
          if rawitem.is_null() { return Err(PyError::FromTypeConversionError); }
          let pyitem = PyObject::new(state, rawitem);
          let item = try!(state.from_py_object::<T>(pyitem));
          v.push(item);
        }
        Ok(v)
      } else {
        Err(PyError::FromTypeConversionError)
      }
    }
  }
}

tuple_pytype!(1,(ref0, 0, A))
tuple_pytype!(2,(ref0, 0, A),(ref1, 1, B))
tuple_pytype!(3, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C))
tuple_pytype!(4, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D))
tuple_pytype!(5, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D),
  (ref4, 4, E))
tuple_pytype!(6, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D),
  (ref4, 4, E), (ref5, 5, F))
tuple_pytype!(7, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D),
  (ref4, 4, E), (ref5, 5, F), (ref6, 6, G))
tuple_pytype!(8, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D),
  (ref4, 4, E), (ref5, 5, F), (ref6, 6, G),(ref7, 7, H))
tuple_pytype!(9, (ref0, 0, A), (ref1, 1, B), (ref2, 2, C), (ref3, 3, D),
  (ref4, 4, E), (ref5, 5, F),(ref6, 6, G),(ref7, 7, H),(ref8, 8, I))

impl ToPyType for String {
  fn to_py_object<'a, 'b>(&'b self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
    // FIXME code duplicated from str slice
    unsafe {
      let raw = state.PyString_FromString(self.to_c_str().unwrap());
      if raw.is_not_null() && state.PyString_Check(raw) > 0 {
        Ok(PyObject::new(state, raw))
      } else {
        Err(PyError::ToTypeConversionError)
      }
    }
  }
}

impl FromPyType for String {
  fn from_py_object(state : &PyState, py_object : PyObject) -> Result<String, PyError>  {
    unsafe {
      if py_object.raw.is_not_null() && state.PyString_Check(py_object.raw) > 0 {
        let c_str = state.PyString_AsString(py_object.raw);
        let string = String::from_str(CString::new(c_str, false).as_str().unwrap());
        Ok(string)
      } else {
        Err(PyError::FromTypeConversionError)
      }
    }
  }
}

impl<'b> ToPyType for &'b str {
  fn to_py_object<'a>(&self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
    unsafe {
      let raw = state.PyString_FromString(self.to_c_str().unwrap());
      if raw.is_not_null() && state.PyString_Check(raw) > 0 {
        Ok(PyObject::new(state, raw))
      } else {
        Err(PyError::ToTypeConversionError)
      }
    }
  }
}

/// Structure that represents an empty tuple in python
pub struct NoArgs;

impl ToPyType for NoArgs {
  fn to_py_object<'a>(&self, state : &'a PyState) -> Result<PyObject<'a>, PyError> {
    Ok(PyObject::empty_tuple(state))
  }
}

impl FromPyType for NoArgs {
  fn from_py_object(_ : &PyState, _ : PyObject) -> Result<NoArgs, PyError>  {
    Ok(NoArgs)
  }
}

#[cfg(test)]
mod test {
  use base::PyState;
  use super::{ToPyType, FromPyType, NoArgs};
  macro_rules! try_or_panic (
      ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
  )

  macro_rules! num_to_py_object_and_back (
    ($t:ty, $func_name:ident) => (
      #[test]
      fn $func_name() {
        let py = PyState::new();
        let value = 123i as $t;
        let py_object = try_or_panic!(value.to_py_object(&py));
        let returned = try_or_panic!(py.from_py_object::<$t>(py_object));
        assert_eq!(returned, 123i as $t);
      }
    )
  )

  num_to_py_object_and_back!(f64, to_from_f64)
  num_to_py_object_and_back!(f32, to_from_f32)
  num_to_py_object_and_back!(i64, to_from_i64)
  num_to_py_object_and_back!(i32, to_from_i32)
  num_to_py_object_and_back!(int, to_from_int)
  num_to_py_object_and_back!(uint, to_from_uint)
  num_to_py_object_and_back!(u8, to_from_u8)
  num_to_py_object_and_back!(u32, to_from_32)
  num_to_py_object_and_back!(u64, to_from_54)

  macro_rules! tuple_to_py_object_and_back (($val:expr, $T:ty, $func_name:ident) => (
    #[test]
    fn $func_name() {
      let py = PyState::new();
      let v = $val;
      let py_object = try_or_panic!(v.to_py_object(&py));
      let returned = try_or_panic!(py.from_py_object::<$T>(py_object));
      assert_eq!(returned, $val);
    }
  ))

  tuple_to_py_object_and_back!((1i,), (int,), to_and_from_tuple1)
  tuple_to_py_object_and_back!((1i,2i), (int,int), to_and_from_tuple2)
  tuple_to_py_object_and_back!((1i,2i,3i), (int,int,int), to_and_from_tuple3)
  tuple_to_py_object_and_back!((1i,2i,3i,4i), (int,int,int,int), to_and_from_tuple4)
  tuple_to_py_object_and_back!((1i,2i,3i,4i,5i), (int,int,int,int,int), to_and_from_tuple5)
  tuple_to_py_object_and_back!((1i,2i,3i,4i,5i,6i), (int,int,int,int,int,int), to_and_from_tuple6)

  #[test]
  fn to_and_from_list() {
    let val = vec!(1i,2i,3i);
    let py = PyState::new();
    let py_object = try_or_panic!(val.to_py_object(&py));
    let returned = try_or_panic!(py.from_py_object::<Vec<int>>(py_object));
    assert_eq!(returned, val);
  }

  #[test]
  fn mixed_convert() {
    let py = PyState::new();
    let value = 123f32;
    let py_object = try_or_panic!(value.to_py_object(&py));
    let result = py.from_py_object::<int>(py_object);
    match result {
      Err(_) => (),
      Ok(x) => panic!("should have failed but got {}", x)
    };
  }

  #[test]
  fn float_to_tuple_should_err() {
    let py = PyState::new();
    let value = 123f32;
    let py_object = try_or_panic!(value.to_py_object(&py));
    let result = py.from_py_object::<(int,int)>(py_object);
    match result {
      Err(_) => (),
      Ok(x) => panic!("should have failed but got {}", x)
    };
  }

  #[test]
  fn tuple_to_float_should_err() {
    let py = PyState::new();
    let value = (123f32, 234f32, 1f32, 3f32);
    let py_object = try_or_panic!(value.to_py_object(&py));
    let result = py.from_py_object::<f32>(py_object);
    match result {
      Err(_) => (),
      Ok(x) => panic!("should have failed but got {}", x)
    };
  }

  #[test]
  fn string_to_py_object_and_back() {
    let py = PyState::new();
    let value = String::from_str("Hello world");
    let py_object = try_or_panic!(value.to_py_object(&py));
    let result = try_or_panic!(py.from_py_object::<String>(py_object));
    assert_eq!(result.as_slice(), "Hello world");
  }

  #[test]
  fn ref_string_to_py_object_and_back_to_string() {
    let py = PyState::new();
    let value = "Hello world";
    let py_object = try_or_panic!(value.to_py_object(&py));
    let result = try_or_panic!(py.from_py_object::<String>(py_object));
    assert_eq!(result.as_slice(), "Hello world");
  }

  #[test]
  fn no_args() {
    // Just Don't fail to convert. Assuming its correct
    let py = PyState::new();
    let value = NoArgs;
    let py_object = try_or_panic!(value.to_py_object(&py));
    let _ = try_or_panic!(py.from_py_object::<NoArgs>(py_object));
  }
}
