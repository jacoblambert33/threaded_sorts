#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include "fibonacci.h"

#define DEBUG 0



int main(int argc, char **argv) {

  if (argc != 2) {
    printf("Usage: %s <int>\n", argv[0]);
    return EXIT_FAILURE;
  }


  long input = strtol(argv[1], NULL, 10);

	//for processor time:
  int msec = 0;  

	//for wall time:
	struct timespec start, finish;
	double elapsed;



	// Test 1 - threaded fib:
	clock_gettime(CLOCK_MONOTONIC, &start);
  clock_t before = clock();

  long ans = *((long *)fib_t(&input));

  clock_t difference = clock() - before;
	clock_gettime(CLOCK_MONOTONIC, &finish);

  msec = difference * 1000000 / CLOCKS_PER_SEC;

	int secs = msec / 1000000;
	int mils = (msec - (secs * 1000000)) / 1000;
	int mcro = msec % 1000; 

	elapsed = (finish.tv_sec - start.tv_sec);
	elapsed += (finish.tv_nsec - start.tv_nsec) / 1000000000.0;

  printf("Calculated answer for threaded fib(%ld) with %ld threads is: %ld\n", input, total_threads, ans );
  printf("Processor time %d seconds %d milliseconds %d microseconds \n", secs, mils, mcro);			
  printf("Wall time %lf seconds \n\n", elapsed); 



	// Test 3 - regular fib:
	clock_gettime(CLOCK_MONOTONIC, &start);
  before = clock();

  ans = fib(input);

  difference = clock() - before;
	clock_gettime(CLOCK_MONOTONIC, &finish);

  msec = difference * 1000000 / CLOCKS_PER_SEC;

	secs = msec / 1000000;
	mils = (msec - (secs * 1000000)) / 1000;
	mcro = msec % 1000; 

	elapsed = (finish.tv_sec - start.tv_sec);
	elapsed += (finish.tv_nsec - start.tv_nsec) / 1000000000.0;

  printf("Calculated answer for standard fib(%ld) is: %ld\n", input, ans);
  printf("Processor time %d seconds %d milliseconds %d microseconds \n",
				secs, mils, mcro); 			
  printf("Wall time %lf seconds \n\n", elapsed); 


  return EXIT_SUCCESS;
}
