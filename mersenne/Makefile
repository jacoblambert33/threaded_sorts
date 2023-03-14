CC=gcc
CFLAGS=-Wall -Wextra -g -std=gnu11 -lpthread -I lib

all: prng 

prng: prng.c 
	$(CC) $(CFLAGS) $^ -o $@

clean:
	rm -rf fib *.o prng

