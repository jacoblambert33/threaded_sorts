#ifndef HELPERS_SORT_H
#define HELPERS_SORT_H

#include <stdbool.h>

// Adapted from:
//	https://algs4.cs.princeton.edu/21elementary/Insertion.java.html

// is v < w ?
bool less(unsigned long long v, unsigned long long w);

// exchange a[i] and a[j]  (for indirect sort)
// NOTE: i >= 0 (must exist in the array). j < size of the array.
//  IMPORTANT: j cannot BE the length of the array.
void exch(unsigned long long a[], int i, int j);

// i.e., binary search.
//  perhaps should assert(is_sorted) before calling; however, this function
//  O(log n) and that one O(n) so it would slow us down unnecessarily; just
//  don't call it on an unsorted array.
int find_split_point(unsigned long long a[], int p, int r,
                     unsigned long long x);

// is the array a[lo..hi) sorted
// NOTE: is_sorted MUST take the length of the array as the upper bound
//  (if you intend to check the entire array.)
bool is_sorted(unsigned long long a[], int lo, int hi);

// untested: a comparator for qsort (or others)
int cmpfunc(const void *a, const void *b);

#endif  // HELPERS_SORT_H
