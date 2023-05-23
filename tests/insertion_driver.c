#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "debug.h"
#include "helpers_sort.h"
#include "insertion.h"

void run_small();
void run_medium();
void run_large();
void generic_comparison_test(int n);
unsigned long long llrand();

int insertion_wins = 0;
int qsort_wins = 0;

int main(int argc, char **argv) {
  run_small();
  run_medium();

  int i = 1;
  while (i < 8) {
    // while (i < 16) {
    // while (i < 32) {
    // while (i < 64) {
    // while (i < 128) {
    // while (i < 256) {
    generic_comparison_test(i++);
  }
  printf("which was faster more often? insertion sort: %d. qsort: %d \n",
         insertion_wins, qsort_wins);
  // run_large();

  generic_comparison_test(100000);
  return 0;
}

void run_small() {
  printf("TEST: insertion sort. small array \n");

  int n = 8;

  unsigned long long *haystack1 = malloc((sizeof *haystack1) * n);
  unsigned long long *haystack2 = malloc((sizeof *haystack2) * n);

  for (int i = 0; i < n; i++) {
    haystack1[i] = llrand();
    haystack2[i] = haystack1[i];
  }

  // see if i have multiple sorted parts:
  if (1) {  //(DEBUG) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%llu\n", haystack1[i]);
    }
    printf("\n");
  }

  bool is_merged = false;

  clock_t before = clock();

  insertion_sort(haystack1, 0, n);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack1, 0, n);
  assert(is_merged);

  before = clock();

  qsort(haystack2, n, sizeof(unsigned long long), cmpfunc);

  difference = clock() - before;
  msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack2, 0, n);
  assert(is_merged);

  printf("\tSUCCESS\n");

  printf("final array is: \n");
  for (int i = 0; i < n; i++) {
    if (i % n == 0) printf("\n");
    printf("%llu\n", haystack1[i]);
  }
  printf("\n");

  free(haystack1);
  free(haystack2);
}

void run_medium() {
  printf("TEST: insertion sort. medium array \n");

  int n = 24;

  unsigned long long *haystack1 = malloc((sizeof *haystack1) * n);
  unsigned long long *haystack2 = malloc((sizeof *haystack2) * n);

  for (int i = 0; i < n; i++) {
    haystack1[i] = llrand();
    haystack2[i] = haystack1[i];
  }

  // see if i have multiple sorted parts:
  if (1) {  //(DEBUG) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%llu\n", haystack1[i]);
    }
    printf("\n");
  }

  bool is_merged = false;

  clock_t before = clock();

  insertion_sort(haystack1, 0, n);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack1, 0, n);
  assert(is_merged);

  before = clock();

  qsort(haystack2, n, sizeof(unsigned long long), cmpfunc);

  difference = clock() - before;
  msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack2, 0, n);
  assert(is_merged);

  printf("\tSUCCESS\n");

  free(haystack1);
  free(haystack2);
}

/*
 */

void run_large() {
  printf("TEST: large insertion merge.\n");
  int n = 100000;  // 100 mil in < 20s, 10mil in 1+ secs.
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
  // quick3_sort_t(haystack, n, 33554431);
  insertion_sort(haystack, 0, n);

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

void generic_comparison_test(int n) {
  printf("TEST: insertion vs. qsort for size %d.\n", n);

  unsigned long long *haystack1 = malloc((sizeof *haystack1) * n);
  unsigned long long *haystack2 = malloc((sizeof *haystack2) * n);

  for (int i = 0; i < n; i++) {
    haystack1[i] = llrand();
    haystack2[i] = haystack1[i];
  }

  /*
// see if i have multiple sorted parts:
if (n < 20) {  //(DEBUG) {
printf("setup array is: \n");
for (int i = 0; i < n; i++) {
if (i % n == 0) printf("\n");
printf("%llu\n", haystack1[i]);
}
printf("\n");
}
  */

  int ins_time = 0;

  bool is_merged = false;

  clock_t before = clock();

  insertion_sort(haystack1, 0, n);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack1, 0, n);
  assert(is_merged);

  ins_time = msec;

  before = clock();

  qsort(haystack2, n, sizeof(unsigned long long), cmpfunc);

  difference = clock() - before;
  msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken: %d seconds %d milliseconds %d microseconds \n\n",
         msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  is_merged = is_sorted(haystack2, 0, n);
  assert(is_merged);

  if (ins_time < msec)
    insertion_wins++;
  else if (msec < ins_time)
    qsort_wins++;

  printf("\tSUCCESS\n");

  free(haystack1);
  free(haystack2);
}

unsigned long long llrand() {
  unsigned long long r = 0;

  for (int i = 0; i < 5; ++i) {
    r = (r << 15) | (rand() & 0x7FFF);
  }

  return r & 0xFFFFFFFFFFFFFFFFULL;
}
