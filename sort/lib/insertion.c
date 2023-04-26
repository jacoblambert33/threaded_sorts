#include "insertion.h"

#include <assert.h>

#include "helpers_sort.h"


void insertion_sort(unsigned long long a[], int lo, int hi) {
  for (int i = lo + 1; i < hi; i++) {
    for (int j = i; j > lo && less(a[j], a[j - 1]); j--) {
      exch(a, j, j - 1);
    }
  }
  assert(is_sorted(a, lo, hi));
  return;
}
