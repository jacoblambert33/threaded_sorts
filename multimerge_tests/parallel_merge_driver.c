#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "debug.h"
#include "helpers_sort.h"
#include "p_merge.h"

void run_many_parts_merge();
void run_book_parm();
void run_large();

int main(int argc, char **argv) {
  run_many_parts_merge();
  run_book_parm();
  run_large();

  return 0;
}

void run_many_parts_merge() {
  printf(
      "TEST: serial merge; merge eight sorted arrays size eight each in three "
      "rounds.\n");
  int n = 8;
  int sets = 8;
  // unsigned long long needle = 1025202362;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *result = malloc((sizeof *result) * n * sets);
  unsigned long long *result2 = malloc((sizeof *result) * n * sets);
  unsigned long long *result3 = malloc((sizeof *result) * n * sets);

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
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  // confirm that i have sorted parts:

  for (int i = 0; i < sets; i++) {
    // unsigned long long *start_ptr = haystack+(i*n);
    int start_idx = i * n;
    int end_idx = (i + 1) * n;
    // bool is_merged = is_sorted(start_ptr, start_idx, end_idx);
    bool is_merged = is_sorted(haystack, start_idx, end_idx);
    assert(is_merged);
    /*
    if (!is_merged)
            printf("sorted run is not sorted %d == %d\n", i, is_merged);
    */
    // printf("sorted run %d == %d\n", i, is_merged);
    /*if (!is_merged) {
            for (int j = start_idx; j < end_idx; j++) {
                    //printf("%lld\n", *(start_ptr+j));
                    printf("%lld\n", haystack[j]);
            }
            printf("\n");
    } */
  }

  // merge the pairs.
  //  easiest way: explicit based on eight sets of 16;
  //   take six arbitrary groups and merge into three, then two then one.
  parallel_merge(haystack, 0, 7, 8, 15, result, 0);
  parallel_merge(haystack, 16, 23, 24, 31, result, 16);
  parallel_merge(haystack, 32, 39, 40, 47, result, 32);
  parallel_merge(haystack, 48, 55, 56, 63, result, 48);

  bool is_merged = is_sorted(result, 0, 16);
  assert(is_merged);

  is_merged = is_sorted(result, 16, 32);
  assert(is_merged);

  is_merged = is_sorted(result, 32, 48);
  assert(is_merged);

  is_merged = is_sorted(result, 48, 64);
  assert(is_merged);

  // round2:
  parallel_merge(result, 0, 15, 16, 31, result2, 0);
  parallel_merge(result, 32, 47, 48, 63, result2, 32);

  is_merged = is_sorted(result2, 0, 32);
  assert(is_merged);

  is_merged = is_sorted(result2, 33, 64);
  assert(is_merged);

  // round3:

  parallel_merge(result2, 0, 31, 32, 63, result3, 0);

  is_merged = is_sorted(result3, 0, 64);

  assert(is_merged);

  printf("\tSUCCESS\n");

  free(haystack);
  free(result);
  free(result2);
  free(result3);
}

void run_book_parm() {
  printf(
      "TEST: threaded merge; merge eight sorted arrays size eight each in "
      "three rounds.\n");
  int n = 8;
  int sets = 8;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *result = malloc((sizeof *result) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand() % 20;  // make nums something i can easily see to
                                // debug.
  }

  for (int i = 0; i < sets; i++) {
    qsort((haystack + i * n), n, sizeof(*haystack), cmpfunc);
  }

  if (DEBUG) {
    // see if i have multiple sorted parts:
    printf("setup array is: \n");
    for (int i = 0; i < n * sets; i++) {
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
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
  parallel_merge_t(haystack, 0, 7, 8, 15, result, 0);
  parallel_merge_t(haystack, 16, 23, 24, 31, result, 16);
  parallel_merge_t(haystack, 32, 39, 40, 47, result, 32);
  parallel_merge_t(haystack, 48, 55, 56, 63, result, 48);

  bool is_merged = is_sorted(result, 0, 16);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  is_merged = is_sorted(result, 16, 32);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  is_merged = is_sorted(result, 32, 48);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  is_merged = is_sorted(result, 48, 64);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  // round2:
  // before the round i need to reset haystack to have the results of the last
  // round. i think i can copy results into it as is. i think i can leave
  // results alone and it will be further altered.
  for (int i = 0; i < n * sets; i++) {
    haystack[i] = result[i];
  }

  parallel_merge_t(haystack, 0, 15, 16, 31, result, 0);
  parallel_merge_t(haystack, 32, 47, 48, 63, result, 32);

  is_merged = is_sorted(result, 0, 32);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  is_merged = is_sorted(result, 33, 64);
  if (DEBUG)
    printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  // round3:
  // same idea as above to start round 2.
  for (int i = 0; i < n * sets; i++) {
    haystack[i] = result[i];
  }

  parallel_merge_t(haystack, 0, 31, 32, 63, result, 0);

  if (DEBUG) {
    // is it right?
    for (int i = 0; i < n * sets; i++) {
      printf("%lld, ", result[i]);
    }
    printf("\n");
  }

  is_merged = is_sorted(result, 0, 64);
  if (DEBUG)
    printf("\nfinal: are the two pieces merged? %s \n\n",
           (is_merged) ? "yes" : "no");

  assert(is_merged);
  printf("\tSUCCESS\n");

  free(haystack);
  free(result);
}

void run_large() {
  printf("TEST: threaded merge in one larger round.\n");
  int n = 1000000;
  int sets = 2;

  unsigned long long *haystack = malloc((sizeof *haystack) * n * sets);
  unsigned long long *result = malloc((sizeof *result) * n * sets);

  for (int i = 0; i < n * sets; i++) {
    haystack[i] = rand();
  }

  for (int i = 0; i < sets; i++) {
    qsort((haystack + i * n), n, sizeof(*haystack), cmpfunc);
  }

  /*
if (DEBUG) {
// see if i have multiple sorted parts:
printf("setup array is: \n");
for (int i = 0; i < n * sets; i++) {
if (i % n == 0) printf("\n");
printf("%lld\n", haystack[i]);
}
printf("\n");
}
  */

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
  parallel_merge_t(haystack, 0, n - 1, n, n * sets - 1, result, 0);

  /*
if (DEBUG) {
// is it right?
for (int i = 0; i < n * sets; i++) {
printf("%lld, ", result[i]);
}
printf("\n");
}
  */

  bool is_merged = is_sorted(result, 0, n * sets);
  if (DEBUG)
    printf("\nfinal: are the two pieces merged? %s \n\n",
           (is_merged) ? "yes" : "no");

  assert(is_merged);
  printf("\tSUCCESS\n");

  free(haystack);
  free(result);
}
