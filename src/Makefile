CC = gcc
CFLAGS += `python-config --cflags` -fPIC --static
AR ?= ar

.PHONY: all
all: libmacroexpand.a

%.o: %.c
	$(CC) $< -o $@ -c $(CFLAGS)

libmacroexpand.a: macroexpand.o
	$(AR) rcs ../libmacroexpand.a macroexpand.o

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
