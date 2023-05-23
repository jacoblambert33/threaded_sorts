#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "debug.h"
#include "helpers_sort.h"
#include "lsd_radix.h"

void run_small();
void run_large();
unsigned long long llrand();

int main(int argc, char **argv) {
  run_small();
  run_large();

  return 0;
}

void run_small() {
  printf("TEST: lsd radix sort. small array \n");

  int n = 8;

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    // haystack[i] = rand();
    haystack[i] = llrand();
  }

  // see if i have multiple sorted parts:
  if (1) {  //(DEBUG) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%llu\n", haystack[i]);
    }
    printf("\n");
  }

  lsd_sort(haystack, n);

  bool is_merged = is_sorted(haystack, 0, n);
  assert(is_merged);

  printf("\tSUCCESS\n");

  printf("final array is: \n");
  for (int i = 0; i < n; i++) {
    if (i % n == 0) printf("\n");
    printf("%llu\n", haystack[i]);
  }
  printf("\n");

  free(haystack);
}

void run_large() {
  printf("TEST: large LSD merge.\n");
  int n = 100000000;  // 100 mil in < 20s, 10mil in 1+ secs.
  int sets = 1;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *aux = malloc((sizeof *aux) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand();
  }

  clock_t before = clock();

  lsd_sort(haystack, n);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, msec / 1000, msec % 1000);

  bool is_merged = is_sorted(haystack, 0, n * sets);
  if (DEBUG)
    printf("\nfinal: are the two pieces merged? %s \n\n",
           (is_merged) ? "yes" : "no");

  assert(is_merged);
  printf("\tSUCCESS\n");

  free(haystack);
  free(aux);
}

unsigned long long llrand() {
  unsigned long long r = 0;

  for (int i = 0; i < 5; ++i) {
    r = (r << 15) | (rand() & 0x7FFF);
  }

  return r & 0xFFFFFFFFFFFFFFFFULL;
}
