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

/*
FIND-SPLIT-POINT(A; p; r; x)
1 low = p // low end of search range
2 high = r + 1 // high end of search range
3 while low < high // more than one element?
4 mid = (low + high)2 // midpoint of range
5 if x <= A[mid] // is answer q <= mid?
6 high = mid // narrow search to AŒlow W mid�
7 else low = mid + 1 // narrow search to AŒmid C 1 W high�
8 return low
*/
int find_split_point(unsigned long long a[], int p, int r, unsigned long long x) {
  int low = p;
  int high = r + 1;
  while (low < high) {
    int mid = (low + high) / 2;
    if (x <= a[mid])
      high = mid;
    else
      low = mid + 1;
  }
  return low;
}



