#ifndef BINARY_SEARCH_H
#define BINARY_SEARCH_H

// INTERFACE for binary_search.
//  args are:
//   1. element to find.
//   2. array to search.
//   3. index to search from; usually 0 if you want to search the whole array.
//   4. index to search to - can't be higher than the maximum index in the
//   array. usually n-1. i.e., if you want to search for 3 in an array of size 5
//   you make this call:
//	binary_search(3, array, 0, 4);
int binary_search(unsigned long long needle, unsigned long long haystack[],
                  int lo_indx, int hi_indx);

#endif  // BINARY_SEARCH_H
