#include "helpers_sort.h"

// is v < w ?
bool less(unsigned long long v, unsigned long long w) {
  return (v < w) ? true : false;
}

// exchange a[i] and a[j]  (for indirect sort)
void exch(unsigned long long a[], int i, int j) {
  unsigned long long swap = a[i];
  a[i] = a[j];
  a[j] = swap;
}

// is the array a[lo..hi) sorted
bool is_sorted(unsigned long long a[], int lo, int hi) {
  for (int i = lo + 1; i < hi; i++)
    if (less(a[i], a[i - 1])) return false;
  return true;
}
