CC = gcc
CFLAGS = -g

all: 01

createdir:
	mkdir -p bin

01: createdir 01.c
	gcc 01.c -o bin/01
	./bin/01

clean:
	rm -r bin