CC=gcc
CFLAGS=-Wall -Wextra -g -std=gnu11 -lpthread -I lib

all: fib 

fib: main.c simple_queue.o
	$(CC) $(CFLAGS) $^ -o $@

threadpool.o: lib/threadpool.c lib/threadpool.h
	$(CC) $(CFLAGS) -c $^ 

simple_queue.o: lib/simple_queue.c lib/simple_queue.h
	$(CC) $(CFLAGS) -c $^

clean:
	rm -rf fib *.o 

