CC=gcc
CFLAGS=-Wall -g

all: bin_search test_p_merge test_pms

bin_search: main.c binary_search.o debug.h
	$(CC) $(CFLAGS) $^ -o $@

test_p_merge: test_p_merge.c binary_search.o helpers_sort.o p_merge.o debug.h
	$(CC) $(CFLAGS) $^ -o $@

test_pms: test_pms.c parallel_merge_sort.o p_merge.o binary_search.o helpers_sort.o debug.h 
	$(CC) $(CFLAGS) $^ -o $@

binary_search.o: binary_search.c binary_search.h debug.h
	$(CC) $(CFLAGS) -c $^

helpers_sort.o: helpers_sort.c helpers_sort.h debug.h
	$(CC) $(CFLAGS) -c $^

p_merge.o: p_merge.c p_merge.h binary_search.h debug.h
	$(CC) $(CFLAGS) -c $^

parallel_merge_sort.o: parallel_merge_sort.c parallel_merge_sort.h debug.h
	$(CC) $(CFLAGS) -c $^

clean:
#	rm -rf bin_search binary_search.o test_p_merge p_merge.o test_pms parallel_merge_sort.o	
	rm -rf bin_search test_p_merge test_pms *.o
