#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"

int cmpfunc(const void *a, const void *b);

// TESTS:

void test_less();
void test_exch(unsigned long long *haystack, int size);
void test_is_sorted(unsigned long long *haystack, int size);


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
	test_less();

  // TEST 2
	test_exch(haystack, n); 

  // TEST 3
  qsort(haystack, n, sizeof(*haystack), cmpfunc); //reset input
	test_is_sorted(haystack, n); 

  free(haystack);

  return 0;
}

void test_less() {

	unsigned long long v = 3L;
	unsigned long long w = 4L;

	bool ans = less(v, w); 
	assert(ans); //assert v is less than w. 
	
	ans = less(w, v); 
	assert(!ans); //assert w is NOT less than v. 

	ans = less(v, v); 
	assert(!ans); //assert v is NOT less than v. 

}


void test_exch(unsigned long long *haystack, int size) {

	unsigned long long v = haystack[0];
	unsigned long long w = haystack[size-1];

	//precondition - for this test, assume a sorted array where the first and last elements are known and not the same. 
	// we know the first element is: and the last is:  
	//printf("v is %lld and w is %lld\n", v, w); 	

	assert(v != w);
	assert(v == 35005211);
	assert(w == 2145174067);
	exch(haystack, 0, size-1);

	v = haystack[0];
	w = haystack[size-1];
	assert(v == 2145174067);
	assert(w == 35005211);
	
}

void test_is_sorted(unsigned long long *haystack, int size) {

	//precondition - haystack is sorted.
	assert(is_sorted(haystack, 0, size));
	exch(haystack, size-2, size-1);
	assert(!is_sorted(haystack, 0, size));

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
