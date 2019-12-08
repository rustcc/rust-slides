TARGETS:=morse fuzz

all: $(TARGETS)

morse: morse.o main.o

CFLAGS:=-std=c99

fuzz: morse.c morse.h fuzz.c
	clang -g -O1 -std=c99 -fsanitize=fuzzer,address,signed-integer-overflow morse.c fuzz.c -o $@

clean:
	rm -f $(TARGETS) $(wildcard *.o)

.PHONY: all clean
