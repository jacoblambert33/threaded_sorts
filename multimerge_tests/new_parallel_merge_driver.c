#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "debug.h"
#include "helpers_sort.h"
#include "p_merge_new.h"

void run_many_parts_merge();
void run_large();
void run_threaded();

int main(int argc, char **argv) {
  run_many_parts_merge();
  run_large();
  run_threaded();

  return 0;
}

void run_many_parts_merge() {
  printf(
      "TEST: serial merge; merge eight sorted arrays size eight each in three "
      "rounds.\n");
  int n = 8;
  int sets = 8;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *aux = malloc((sizeof *aux) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand();
  }

  for (int i = 0; i < sets; i++) {
    qsort((haystack + i * n), n, sizeof(*haystack), cmpfunc);
  }

  // see if i have multiple sorted parts:
  if (DEBUG) {
    printf("setup array is: \n");
    for (int i = 0; i < n * sets; i++) {
      // if (i % n == 0) printf("\n");
      // printf("%lld\n", haystack[i]);
    }
    // printf("\n");
  }

  // confirm that i have sorted parts:

  for (int i = 0; i < sets; i++) {
    // unsigned long long *start_ptr = haystack+(i*n);
    int start_idx = i * n;
    int end_idx = (i + 1) * n;
    // bool is_merged = is_sorted(start_ptr, start_idx, end_idx);
    bool is_merged = is_sorted(haystack, start_idx, end_idx);
    assert(is_merged);
  }

  // merge the pairs.
  //  easiest way: explicit based on eight sets of 16;
  //   take six arbitrary groups and merge into three, then two then one.
  p_merge_new(haystack, aux, 0, 7, 15);
  p_merge_new(haystack, aux, 16, 23, 31);
  p_merge_new(haystack, aux, 32, 39, 47);
  p_merge_new(haystack, aux, 48, 55, 63);

  bool is_merged = is_sorted(haystack, 0, 16);
  assert(is_merged);

  is_merged = is_sorted(haystack, 16, 32);
  assert(is_merged);

  is_merged = is_sorted(haystack, 32, 48);
  assert(is_merged);

  is_merged = is_sorted(haystack, 48, 64);
  assert(is_merged);

  // printf("\nfinal: are the two pieces merged? %s \n\n", (is_merged) ? "yes" :
  // "no");

  // round2:
  p_merge_new(haystack, aux, 0, 15, 31);
  p_merge_new(haystack, aux, 32, 47, 63);

  is_merged = is_sorted(haystack, 0, 32);
  assert(is_merged);

  is_merged = is_sorted(haystack, 33, 64);
  assert(is_merged);

  // round3:

  p_merge_new(haystack, aux, 0, 31, 63);

  is_merged = is_sorted(haystack, 0, 64);

  assert(is_merged);

  printf("\tSUCCESS\n");

  free(haystack);
  free(aux);
}

void run_large() {
  printf("TEST: serial merge in one larger round.\n");
  int n = 1000000;  // 10 mil reasonable max
  int sets = 2;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *aux = malloc((sizeof *aux) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand();
  }

  for (int i = 0; i < sets; i++) {
    qsort((haystack + i * n), n, sizeof(*haystack), cmpfunc);
  }

  // confirm that i have sorted parts:

  for (int i = 0; i < sets; i++) {
    // unsigned long long *start_ptr = haystack+(i*n);
    int start_idx = i * n;
    int end_idx = (i + 1) * n;
    // bool is_merged = is_sorted(start_ptr, start_idx, end_idx);
    bool is_merged = is_sorted(haystack, start_idx, end_idx);
    assert(is_merged);
  }

  clock_t before = clock();
  // merge the pairs.
  //  easiest way: explicit based on eight sets of 16;
  //   take six arbitrary groups and merge into three, then two then one.
  // parallel_merge_t(haystack, 0, n - 1, n, n * sets - 1, result, 0);
  p_merge_new(haystack, aux, 0, n - 1, n * sets - 1);

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

void run_threaded() {
  printf("TEST: threaded merge in one larger round.\n");
  int n = 1000000;  // 10 mil decent max //can't do 100 mil in a reasonable
                    // time.
  int c_ratio = 2;  // 5; //2; //5;//10;
  int sets = 2;
  // int cutoff = 4194304; //1048576; //65536; //16384; //4096; //1024; //256;
  // //64; //16;
  int cutoff = n / c_ratio;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *aux = malloc((sizeof *aux) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand();
  }

  for (int i = 0; i < sets; i++) {
    qsort((haystack + i * n), n, sizeof(*haystack), cmpfunc);
  }

  // confirm that i have sorted parts:

  for (int i = 0; i < sets; i++) {
    // unsigned long long *start_ptr = haystack+(i*n);
    int start_idx = i * n;
    int end_idx = (i + 1) * n;
    // bool is_merged = is_sorted(start_ptr, start_idx, end_idx);
    bool is_merged = is_sorted(haystack, start_idx, end_idx);
    assert(is_merged);
  }

  clock_t before = clock();

  p_merge_new_t(haystack, aux, 0, n - 1, n * sets - 1, cutoff);

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
