#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "binary_search.h"
#include "debug.h"
#include "helpers_sort.h"
#include "p_merge_sort.h"

void run_small();
void run_medium();
void run_large();

int main(int argc, char **argv) {
  run_small();
  run_medium();
  run_large();

  return 0;
}

void run_small() {
  int n = 16;  // 10000; //4096; //2048; //260; //258; //256; // 64;
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
      // if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  // works on the surface but broken
  // pms(haystack, 0, n);
  // pms_4(haystack, 0, n-1);
  pms_jml(haystack, 0, n - 1);

  bool is_merged = is_sorted(haystack, 0, n);
  // if (DEBUG)
  printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  for (int i = 0; i < n; i++) {
    if (i % n == 0) printf("\n");
    printf("%lld\n", haystack[i]);
  }
  printf("\n");

  assert(is_merged);

  printf("SUCCESS: parallel merge sort on small array.\n");

  free(haystack);
}

void run_medium() {
  int n = 1000;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on small array - size %d.\n", n);

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    haystack[i] = rand();
    // haystack[i] = rand() % 20;  // make nums something i can easily see to
    //  debug.
  }

  if (0) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      // if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  pms_jml(haystack, 0, n - 1);

  bool is_merged = is_sorted(haystack, 0, n);
  // if (DEBUG)
  printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  if (0) {
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
  }
  printf("\n");

  assert(is_merged);

  printf("SUCCESS: parallel merge sort on medium array.\n");

  free(haystack);
}

void run_large() {
  int n = 10000000;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on small array - size %d.\n", n);

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    haystack[i] = rand();
    // haystack[i] = rand() % 20;  // make nums something i can easily see to
    //  debug.
  }

  if (0) {
    printf("setup array is: \n");
    for (int i = 0; i < n; i++) {
      // if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
    printf("\n");
  }

  pms_jml(haystack, 0, n - 1);

  bool is_merged = is_sorted(haystack, 0, n);
  // if (DEBUG)
  printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

  if (0) {
    for (int i = 0; i < n; i++) {
      if (i % n == 0) printf("\n");
      printf("%lld\n", haystack[i]);
    }
  }
  printf("\n");

  assert(is_merged);

  printf("SUCCESS: parallel merge sort on large array.\n");

  free(haystack);
}
