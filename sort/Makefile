ST = default
CC=gcc
CFLAGS=-Wall -g -std=gnu11 -lpthread -I lib/ -D${ST}
# options for D flag are: QSORT, MEQ, INSERT, SMS, BPMS, PMS

all: sort 

sort: main.c lib/insertion.o lib/helpers_sort.o lib/serial_merge.o lib/basic_par_merge.o lib/quick.o lib/p_merge.o lib/binary_search.o lib/parallel_merge_sort.o lib/p_merge_new.o lib/p_merge_sort.o lib/pms4.o
	$(CC) $(CFLAGS) $^ -o $@

lib/pms4.o: lib/pms4.c lib/pms4.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/p_merge_new.o: lib/p_merge_new.c lib/p_merge_new.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/p_merge_sort.o: lib/p_merge_sort.c lib/p_merge_sort.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/basic_par_merge.o: lib/basic_par_merge.c lib/basic_par_merge.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/parallel_merge_sort.o: lib/parallel_merge_sort.c lib/parallel_merge_sort.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/insertion.o: lib/insertion.c lib/insertion.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/serial_merge.o: lib/serial_merge.c lib/serial_merge.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/helpers_sort.o: lib/helpers_sort.c lib/helpers_sort.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/binary_search.o: lib/binary_search.c lib/binary_search.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/p_merge.o: lib/p_merge.c lib/p_merge.h
	$(CC) $(CFLAGS) -c $< -o $@

#lib/ull_cmp.o: lib/ull_cmp.c lib/ull_cmp.h
#	$(CC) $(CFLAGS) -c $< -o $@

lib/quick.o: lib/quick.c lib/quick.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf sort lib/*.o

#lib/basic_par_merge.o lib/serial_merge.o lib/insertion.o lib/helpers_sort.o lib/ull_cmp.o lib/quick.o lib/p_merge.o lib/parallel_merge_sort.o


