ST = default
CC=gcc
CFLAGS=-Wall -g -std=gnu11 -lpthread -I lib/ -D${ST}

all: sort 

sort: main.c lib/insertion.o lib/helpers_sort.o lib/serial_merge.o lib/basic_par_merge.o lib/ull_cmp.o lib/quick.o
	$(CC) $(CFLAGS) $^ -o $@

lib/basic_par_merge.o: lib/basic_par_merge.c lib/basic_par_merge.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/insertion.o: lib/insertion.c lib/insertion.h 
	$(CC) $(CFLAGS) -c $< -o $@

lib/serial_merge.o: lib/serial_merge.c lib/serial_merge.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/helpers_sort.o: lib/helpers_sort.c lib/helpers_sort.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/ull_cmp.o: lib/ull_cmp.c lib/ull_cmp.h
	$(CC) $(CFLAGS) -c $< -o $@

lib/quick.o: lib/quick.c lib/quick.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf sort lib/basic_par_merge.o lib/serial_merge.o lib/insertion.o lib/helpers_sort.o lib/ull_cmp.o lib/quick.o

