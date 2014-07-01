use libc::{c_long, c_double, size_t};

pub use base::{PyObject, PyObjectRaw, PyType, start_python, PY_MUTEX};
pub use base::{PyError,
           FromTypeConversionError,
           ToTypeConversionError,
           NullPyObject};

//Duplicate the macro here as macro_export has internal compiler error
pub macro_rules! py_lock(
  ($expr:expr) => ({
  let _g = PY_MUTEX.lock();
  let res = $expr;
  res
  })
)

#[link(name = "python2.7")]
extern {
  fn PyInt_FromLong(ival : c_long) -> *mut PyObjectRaw;
  fn PyInt_AsLong(obj : *mut PyObjectRaw) -> c_long;

  fn PyFloat_FromDouble(value : c_double) -> *mut PyObjectRaw;
  fn PyFloat_AsDouble(obj : *mut PyObjectRaw) -> c_double;

  fn PyTuple_New(size : size_t) -> *mut PyObjectRaw;
  fn PyTuple_GetItem(tuple : *mut PyObjectRaw, pos : size_t) -> *mut PyObjectRaw;
  fn PyTuple_SetItem(tuple : *mut PyObjectRaw, pos : size_t, o : *mut PyObjectRaw);

  fn Py_IncRef(obj: *mut PyObjectRaw);
}

macro_rules! prim_pytype (
  ($base_type:ty, $cast_type:ty, $to:expr, $back:expr) => (
    impl PyType for $base_type {
      fn to_py_object(&self) -> Result<PyObject, PyError> {
        unsafe {
          let raw = py_lock!($to(*self as $cast_type));
          if raw.is_null() {
            Err(ToTypeConversionError)
          } else {
            Ok(PyObject::new(raw))
          }
        }
      }

      //TODO check to see if py_object is correct type before attempting to convert
      fn from_py_object(py_object : PyObject) -> Result<$base_type, PyError>  {
        unsafe {
          if py_object.raw.is_null() {
            Err(FromTypeConversionError)
          } else {
            Ok(py_lock!($back(py_object.raw)) as $base_type)
          }
        }
      }
    }
  )
)

prim_pytype!(f64, f64, PyFloat_FromDouble, PyFloat_AsDouble)
prim_pytype!(f32, f64, PyFloat_FromDouble, PyFloat_AsDouble)
prim_pytype!(i64, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(i32, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(int, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(uint, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(u8, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(u32, c_long, PyInt_FromLong, PyInt_AsLong)
prim_pytype!(u64, c_long, PyInt_FromLong, PyInt_AsLong)

// Helper function to quickly convert from py objects
pub fn from_py_object<A : PyType>(obj : PyObject) -> Result<A, PyError> {
  PyType::from_py_object(obj)
}



macro_rules! tuple_pytype ({$length:expr,$(($refN:ident, $n:expr, $T:ident)),+} => (
  impl<$($T:PyType+Clone),+> PyType for ($($T,)+) {
    fn to_py_object(&self) -> Result<PyObject, PyError> {
      $(let $refN = self.$refN();)+
      unsafe {
        let raw = py_lock!(PyTuple_New($length));
        $(let $refN = try!($refN.to_py_object());)+
        $(Py_IncRef($refN.raw);)+
        $(py_lock!(PyTuple_SetItem(raw, $n, $refN.raw));)+

        if raw.is_null() {
          Err(ToTypeConversionError)
        } else {
          Ok(PyObject::new(raw))
        }
      }
    }

    // TODO check to see if py_object is correct type before attempting to convert
    fn from_py_object(py_object : PyObject) -> Result<($($T,)+), PyError>  {
      unsafe {
        if py_object.raw.is_null() {
          Err(FromTypeConversionError)
        } else {
          let raw = py_object.raw;
          //TODO add size checks?
          $(let $refN = PyObject::new(py_lock!(PyTuple_GetItem(raw, $n)));)+
          $(let $refN = try!(from_py_object::<$T>($refN));)+
          Ok(($($refN,)+))
        }
      }
    }
  }
))

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


#[cfg(test)]
mod test {
  use base::{start_python};
  use super::{from_py_object, PyType};
  macro_rules! try_or_fail (
      ($e:expr) => (match $e { Ok(e) => e, Err(e) => fail!("{}", e) })
  )

  macro_rules! num_to_py_object_and_back (
    ($t:ty, $func_name:ident) => (
      #[test]
      fn $func_name() {
        start_python();
        let value = 123i as $t;
        let py_object = try_or_fail!(value.to_py_object());
        let returned = try_or_fail!(from_py_object::<$t>(py_object));
        assert_eq!(returned, 123i as $t);
      }
    )
  )

  num_to_py_object_and_back!(f32, to_from_f32)
  num_to_py_object_and_back!(f64, to_from_f64)
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
      start_python();
      let py_object = try_or_fail!($val.to_py_object());
      let returned = try_or_fail!(from_py_object::<$T>(py_object));
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
  fn mixed_convert() {
    start_python();
    let value = 123f32;
    let py_object = try_or_fail!(value.to_py_object());
    let returned = try_or_fail!(from_py_object::<int>(py_object));
    assert_eq!(returned, 123);
  }
}
