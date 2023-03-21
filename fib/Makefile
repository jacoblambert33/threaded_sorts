CC=gcc
CFLAGS=-Wall -Wextra -g -std=gnu11 -lpthread -I lib

all: fib 

fib: main.c fibonacci.o
	$(CC) $(CFLAGS) $^ -o $@

fibonacci.o: lib/fibonacci.c lib/fibonacci.h
	$(CC) $(CFLAGS) -c $^ 

clean:
	rm -rf fib *.o fib

