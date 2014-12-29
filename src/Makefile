CC = gcc
AR ?= ar

OS := $(shell uname)

ifeq ($(OS), Linux)
CFLAGS += `python-config --cflags` -fPIC --static
else
CFLAGS += -IC:/Python27/include -LC:/Python27/libs -lpython27 -shared
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
