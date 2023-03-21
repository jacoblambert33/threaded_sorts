#include "binary_search.h"

int binary_search(unsigned long long needle, unsigned long long haystack[],
                  int lo_indx, int hi_indx) {
  // eventually i need is_sorted here, since this method only makes sense on
  // sorted arrays.

  int lo = lo_indx;
  int hi = hi_indx < lo ? lo : hi_indx + 1;

  while (lo < hi) {
    int mid = lo + (hi - lo) / 2;

    if (needle <= haystack[mid])
      hi = mid;
    else
      lo = mid + 1;
  }
  return hi;
}
