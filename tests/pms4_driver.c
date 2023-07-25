#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <time.h>

#include "debug.h"
#include "helpers_sort.h"
#include "pms4.h"

void run_small();
void run_small_t();
void run_medium();
void run_medium_t();
void run_large();
void run_large_t();

int main(int argc, char **argv) {
  // run_small_t();
  // run_small();
  // run_medium();
  // run_medium_t();
  run_large();
  run_large_t();

  return 0;
}

void run_small_t() {
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

  pms4_t(haystack, 0, n - 1, 0);

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

  pms4_s(haystack, 0, n - 1);

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

void run_medium_t() {
  int n = 1000;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on medium array - size %d.\n", n);

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

  pms4_t(haystack, 0, n - 1, 16);

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

void run_medium() {
  int n = 1000;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: parallel merge sort on medium array - size %d.\n", n);

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

  pms4_s(haystack, 0, n - 1);

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
  printf("TEST: parallel merge sort on large array - size %d.\n", n);

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

  clock_t before = clock();

  pms4_s(haystack, 0, n - 1);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Time taken %d (all) %d seconds %d milliseconds %d microseconds \n",
         msec, msec / 1000000, (msec / 1000) % 1000, msec % 1000);

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

void run_large_t() {
  int n = 10000000;  // 10000; //4096; //2048; //260; //258; //256; // 64;
  printf("TEST: threaded parallel merge sort on large array - size %d.\n", n);

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

  long start, end;
  struct timeval timecheck;

  gettimeofday(&timecheck, NULL);
  start = (long)timecheck.tv_sec * 1000 + (long)timecheck.tv_usec / 1000;

  clock_t before = clock();

  // pms4_t(haystack, 0, n - 1, 4096);
  // pms4_t(haystack, 0, n - 1, 16384);
  // pms4_t(haystack, 0, n - 1, 65536);
  pms4_t(haystack, 0, n - 1, 65536 << 2);

  clock_t difference = clock() - before;
  int msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf(
      "Cpu time taken %d (all) %d seconds %d milliseconds %d microseconds \n",
      msec, msec / 1000000, (msec / 1000) % 1000, msec % 1000);

  gettimeofday(&timecheck, NULL);
  end = (long)timecheck.tv_sec * 1000 + (long)timecheck.tv_usec / 1000;

  printf("%ld milliseconds elapsed\n", (end - start));
  msec = end - start;
  printf("Wall time taken %d (all) %d seconds %d milliseconds \n", msec,
         (msec / 1000) % 1000, msec % 1000);

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
