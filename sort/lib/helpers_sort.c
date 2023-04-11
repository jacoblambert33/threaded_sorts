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
