#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "binary_search.h"

int cmpfunc(const void *a, const void *b);

// TESTS:

void find_every_element(unsigned long long *haystack, int size);

void find_elements_outside_bounds(unsigned long long *haystack, int size);

int main(int argc, char **argv) {
  // setup common to all tests:
  //  precondition for binary search is a sorted array:

  int n = 50;

  unsigned long long *haystack = malloc((sizeof *haystack) * n);

  for (int i = 0; i < n; i++) {
    haystack[i] = rand();
  }

  qsort(haystack, n, sizeof(*haystack), cmpfunc);

  // TEST 1
  find_every_element(haystack, n);

  // TEST 2
  find_elements_outside_bounds(haystack, n);

  free(haystack);

  return 0;
}

void find_every_element(unsigned long long *haystack, int size) {
  for (int i = 0; i < size; i++) {
    unsigned long long needle = haystack[i];
    int index = binary_search(needle, haystack, 0, size - 1);
    assert(index >= 0 && index < size);
    printf("needle %lld is at index %d of haystack.\n", needle, index);
  }
}

void find_elements_outside_bounds(unsigned long long *haystack, int size) {
  unsigned long long needle = 0L;
  // find an element smaller than every element in the array:
  int index = binary_search(needle, haystack, 0, size - 1);
  printf("needle %lld is at index %d of haystack.\n", needle, index);
  assert(index == 0);

  // find an element larger than every element in the array:
  needle = 9991025202362L;
  index = binary_search(needle, haystack, 0, size - 1);
  printf("needle %lld is at index %d of haystack.\n", needle, index);
  assert(index == size);
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
