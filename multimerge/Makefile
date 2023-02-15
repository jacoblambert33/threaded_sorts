CC=gcc
CFLAGS=-Wall -g

all: bin_search test_p_merge

bin_search: main.c binary_search.o
	$(CC) $(CFLAGS) $^ -o $@

test_p_merge: test_p_merge.c binary_search.o p_merge.o
	$(CC) $(CFLAGS) $^ -o $@

binary_search.o: binary_search.c binary_search.h
	$(CC) $(CFLAGS) -c $^

p_merge.o: p_merge.c p_merge.h
	$(CC) $(CFLAGS) -c $^

clean:
	rm -rf bin_search binary_search.o test_p_merge p_merge.o

