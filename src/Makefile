CC ?= gcc
CFLAGS += `python2.7-config --cflags`
AR ?= ar

.PHONY: all
all: libmacroexpand.a

%.o: %.c
	$(CC) $< -o $@ -c $(CFLAGS)

libmacroexpand.a: macroexpand.o
	$(AR) rcs ../target/libmacroexpand.a macroexpand.o

.PHONY: clean
clean:
	rm -f *.o *.a *.so *.dylib *.rlib *.dll *.dummy *-test
