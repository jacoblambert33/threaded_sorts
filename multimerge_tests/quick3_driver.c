#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "debug.h"
#include "helpers_sort.h"
#include "quick3_radix.h"

void run_small();
void run_medium();
void run_large();
unsigned long long llrand();

int main(int argc, char **argv) {
  // run_small();
  run_medium();
  run_large();

  return 0;
}

void run_small() {
  printf("TEST: quick3 radix sort. small array \n");

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

  quick3_sort(haystack, n);
  quick3_sort_t(haystack, n, 3);

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

void run_medium() {
  printf("TEST: quick3 radix sort. medium array \n");

  int n = 24;

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    // haystack[i] = rand();
    haystack[i] = llrand();
  }
  haystack[0] = 27746639296693664;
  haystack[1] = 5855599344522922;
  haystack[2] = 72905761055942874;
  haystack[3] = 120681916629214561;
  haystack[4] = 134514524321646123;
  haystack[5] = 145654727656638779;

  // see if i have multiple sorted parts:
  if (1) {  //(DEBUG) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%llu\n", haystack[i]);
    }
    printf("\n");
  }

  // quick3_sort(haystack, n);
  quick3_sort_t(haystack, n, 3);

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

/*
 */

void run_large() {
  printf("TEST: large quick3 merge.\n");
  int n = 100000000;  // 100 mil in < 20s, 10mil in 1+ secs.
  int sets = 1;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *aux = malloc((sizeof *aux) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = llrand();
  }

  clock_t before = clock();

  // quick3_sort(haystack, n);
  // quick3_sort_t(haystack, n, 15);
  // quick3_sort_t(haystack, n, 31);
  // quick3_sort_t(haystack, n, 63);
  // quick3_sort_t(haystack, n, 127);
  // quick3_sort_t(haystack, n, 255);
  // quick3_sort_t(haystack, n, 4095);
  // quick3_sort_t(haystack, n, 8191);
  // quick3_sort_t(haystack, n, 16383);
  // quick3_sort_t(haystack, n, 32767);
  // quick3_sort_t(haystack, n, 65535);
  // quick3_sort_t(haystack, n, 131071);
  // quick3_sort_t(haystack, n, 262143);
  // quick3_sort_t(haystack, n, 262143);
  // quick3_sort_t(haystack, n, 524287);
  // quick3_sort_t(haystack, n, 1048575);
  // quick3_sort_t(haystack, n, 2097151);
  // quick3_sort_t(haystack, n, 4194303);
  // quick3_sort_t(haystack, n, 8388607);
  // quick3_sort_t(haystack, n, 16777213);
  quick3_sort_t(haystack, n, 33554431);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken %d (all) %d seconds %d milliseconds %d microseconds \n",
         msec, msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  bool is_merged = is_sorted(haystack, 0, n * sets);
  if (DEBUG)
    printf("\nfinal: are the two pieces merged? %s \n\n",
           (is_merged) ? "yes" : "no");

  /*
  printf("final array is: \n");
  for (int i = 0; i < n; i++) {
          if (i % n == 0) printf("\n");
          printf("%llu\n", haystack[i]);
  }
  printf("\n");
  */

  for (int i = 1; i < n; i++)
    if (haystack[i] < haystack[i - 1])
      printf("incorrectly found %llu before %llu\n", haystack[i - 1],
             haystack[i]);

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
