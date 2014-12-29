CC = gcc
AR ?= ar

OS := $(shell uname)

ifeq ($(OS), Linux)

CFLAGS += `python-config --cflags` -fPIC --static

else

PYTHON_DIR = C:/Python27/include
CFLAGS += -I$(PYTHON_DIR) -fPIC

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
