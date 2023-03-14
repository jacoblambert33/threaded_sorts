#ifndef HELPERS_SORT_H
#define HELPERS_SORT_H

#include <stdbool.h>
//#include <stdint.h>

// is v < w ?
//bool less(uint64_t v, uint64_t w); 
bool less(unsigned long long v, unsigned long long w); 

// exchange a[i] and a[j]  (for indirect sort)
//void exch(uint64_t a[], int i, int j); 
void exch(unsigned long long a[], int i, int j); 

// is the array a[lo..hi) sorted
//bool is_sorted(uint64_t a[], int lo, int hi);
bool is_sorted(unsigned long long a[], int lo, int hi);


#endif //HELPERS_SORT_H
