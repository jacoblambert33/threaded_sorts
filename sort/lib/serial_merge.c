
#include "serial_merge.h"
// #include<stdint.h> //unsure if needed
#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"

// void insertion_sort(uint64_t a[], int lo, int hi);
//  ENTRY POINT - all the work done in functions from inside this function. this
//  is the API, however, and the only function required by the header. however,
//  implementing only this function is not meant to sort anything by itself.
// private declarations
void smergesort_aux(unsigned long long a[], unsigned long long aux[], int lo,
                    int hi);

void merge(unsigned long long a[], unsigned long long aux[], int lo, int mid,
           int hi);

// public API
void smergesort(unsigned long long a[], int lo, int hi) {
  size_t len = hi - lo;  // length of the input array.
  // int len = hi - lo; //length of the input array.
  //  Preferred: use size of dereferenced pointer
  unsigned long long *aux = malloc(len * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }
  /* not necessary since i do it in the merge function.
  // iterate through array, initializing values.
  for (size_t i = lo; i < hi; ++i) {
          aux[i] = a[i];
  }
  */
  smergesort_aux(a, aux, lo, hi);
  assert(is_sorted(a, lo, hi));

  // tmp don't do this while i investigate double free error
  // free(aux);
}

void smergesort_aux(unsigned long long a[], unsigned long long aux[], int lo,
                    int hi) {
  if (hi <= lo + 1) return;
  int mid = lo + (hi - lo) / 2;
  smergesort_aux(a, aux, lo, mid);
  // smergesort_aux(a, aux, mid + 1, hi);
  smergesort_aux(a, aux, mid, hi);
  merge(a, aux, lo, mid, hi);
}  // END smergesort_aux

void merge(unsigned long long a[], unsigned long long aux[], int lo, int mid,
           int hi) {
  // precondition: a[lo .. mid] and a[mid+1 .. hi] are sorted subarrays
  assert(is_sorted(a, lo, mid));
  // assert(is_sorted(a, mid+1, hi));
  assert(is_sorted(a, mid, hi));

  // copy to aux[]
  // for (int k = lo; k <= hi; k++) {
  for (int k = lo; k < hi; k++) {
    aux[k] = a[k];
  }

  // merge back to a[]
  // int i = lo, j = mid+1;
  int i = lo, j = mid;
  // for (int k = lo; k <= hi; k++) {
  for (int k = lo; k < hi; k++) {
    // if (i > mid)											 a[k] =
    // aux[j++];
    if (i >= mid) a[k] = aux[j++];
    // else if (j > hi)               a[k] = aux[i++];
    else if (j >= hi)
      a[k] = aux[i++];
    else if (less(aux[j], aux[i]))
      a[k] = aux[j++];
    else
      a[k] = aux[i++];
  }

  // postcondition: a[lo .. hi] is sorted
  assert(is_sorted(a, lo, hi));
}  // END merge
