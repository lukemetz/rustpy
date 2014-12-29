CC = gcc
<<<<<<< HEAD
<<<<<<< HEAD
=======
CFLAGS += `python-config --cflags` -fPIC --static
>>>>>>> 4e9c1a0... Testing on linux
=======
>>>>>>> dae0304... Makefile for windows
AR ?= ar

OS := $(shell uname)

ifeq ($(OS), Linux)
<<<<<<< HEAD
CFLAGS += `python-config --cflags` -fPIC --static
else
CFLAGS += -IC:/Python27/include -LC:/Python27/libs -lpython27 -shared
=======

CFLAGS += `python-config --cflags` -fPIC --static

else

PYTHON_DIR = C:/Python27/include
CFLAGS += -I$(PYTHON_DIR) -fPIC --static

>>>>>>> dae0304... Makefile for windows
endif

.PHONY: all
all: libmacroexpand.a

%.o: %.c
	$(CC) $< -o $@ -c $(CFLAGS)

libmacroexpand.a: macroexpand.o
	$(AR) rcs ../libmacroexpand.a macroexpand.o

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
