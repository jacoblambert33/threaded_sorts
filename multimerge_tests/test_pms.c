#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "binary_search.h"
#include "debug.h"
#include "p_merge.h"
#include "parallel_merge_sort.h"

int cmpfunc(const void *a, const void *b);

bool is_sorted(unsigned long long a[], int lo, int hi);

void run_pms();
void run_book();

int main(int argc, char **argv) {
  // run_pms();
  run_book();

  return 0;
}

int cmpfunc(const void *a, const void *b) {
  const unsigned long long ai = *(const unsigned long long *)a;
  const unsigned long long bi = *(const unsigned long long *)b;

  if (ai < bi) {
    return -1;
  } else if (ai > bi) {
    return 1;
  } else {
    return 0;
  }
}

void run_pms() {
  int n = 8;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on small array - size %d.\n", n);

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    // haystack[i] = rand();
    haystack[i] = rand() % 20;  // make nums something i can easily see to
                                // debug.
  }

  if (1) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  // confirm that i have sorted parts:

  // merge the pairs.
  //  easiest way: explicit based on eight sets of 16;
  //   take six arbitrary groups and merge into three, then two then one.
  // api_par_merge(haystack, 64, 0, 7, 8, 15, 0);
  pms(haystack, 0, n - 1);

  bool is_merged = is_sorted(haystack, 0, n);
  // if (DEBUG)
  printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  for (int i = 0; i < n; i++) {
    if (i % n == 0) printf("\n");
    printf("%lld\n", haystack[i]);
  }
  printf("\n");
}

void run_book() {
  int n = 10000;  // 40; //10; //50; //10000; // 100; //18; // 10000; //4096;
                  // //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on small array - size %d.\n", n);

  unsigned long long *haystack = malloc((sizeof *haystack) * n);
  unsigned long long *result = malloc((sizeof *result) * n);

  for (int i = 0; i < n; i++) {
    haystack[i] = rand();
    // haystack[i] = rand() % 20; //make nums something i can easily see to
    // debug.
  }

  if (1) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  // confirm that i have sorted parts:

  // merge the pairs.
  // book_p_merge_sort(haystack, 0, n-1, result, 0);
  book_p_merge_sort_entry(haystack, 0, n - 1, result, 0);

  bool is_merged = is_sorted(result, 0, n);
  // if (DEBUG)
  printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  /*
          for (int i = 0; i < n; i++) {
                          if (i % n == 0) printf("\n");
                          printf("%lld\n", result[i]);
          }
          printf("\n");
  */

  free(haystack);
  free(result);
}
