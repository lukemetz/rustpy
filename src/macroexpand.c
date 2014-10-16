#include <Python.h>

int RPyFloat_Check(PyObject* obj) {
  return PyFloat_Check(obj);
}

int RPyFloat_CheckExact(PyObject* obj) {
  return PyFloat_CheckExact(obj);
}

int RPyTuple_Check(PyObject* obj) {
  return PyTuple_Check(obj);
}

int RPyList_Check(PyObject* obj) {
  return PyList_Check(obj);
}

int RPyInt_Check(PyObject* obj) {
  return PyInt_Check(obj);
}

int RPyString_Check(PyObject* obj) {
  return PyString_Check(obj);
}
