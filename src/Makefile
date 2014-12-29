CC = gcc
AR ?= ar

PYTHON_INCLUDE_DIR = C:/Python27/include
PYTHON_LIB_DIR = C:/Python27/libs

OS := $(shell uname)

ifeq ($(OS), Linux)
CFLAGS += `python-config --cflags` -fPIC --static
else
CFLAGS += -I$(PYTHON_INCLUDE_DIR) -fPIC --static
LDFLAGS += -I$(PYTHON_LIB_DIR)
endif

.PHONY: all
all: libmacroexpand.a

%.o: %.c
	$(CC) $< -o $@ -c $(CFLAGS) $(LDFLAGS)

libmacroexpand.a: macroexpand.o
	$(AR) rcs ../libmacroexpand.a macroexpand.o

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
