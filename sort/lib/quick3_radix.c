#include "quick3_radix.h"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"
#include "insertion.h"

struct q3_params {
  unsigned long long *a;
  int lo;
  int hi;
  int d;
  int cutoff;
};

size_t q3_threadcount = 0;

// Adapted from:
// https://algs4.cs.princeton.edu/51radix/Quick3string.java.html
//  and
// https://www.coursera.org/learn/algorithms-part2/lecture/crkd5/3-way-radix-quicksort

int BITS_PER_BYTE = 8;
int BITS_PER_INT = 64;
int MASK = 255;  // 0xFF, or 255.
int Q3_CUTOFF = 15;

int charAt(unsigned long long s, int d) {
  // assert d >= 0 and d is less than or equal to the 7th byte d[7]
  if (d == 8) return -1;
  int shift = BITS_PER_INT - BITS_PER_BYTE * d - BITS_PER_BYTE;
  return (s >> shift) & MASK;
}

// void quick3(unsigned long long a[], int lo, int hi, int d, unsigned long long
// aux[]) {
void quick3(unsigned long long a[], int lo, int hi, int d) {
  // cutoff to insertion sort for small subarrays
  // if (hi <= lo ) {
  if (hi <= lo + Q3_CUTOFF) {
    insertion_sort(
        a, lo,
        hi +
            1);  // the hi+1 is right but not as intended from this perspective.
                 // consider going back to change the insertion sort interface.
    return;
  }

  // no more bits
  if (d == 7) return;

  int lt = lo;
  int gt = hi;

  // int shift = BITS_PER_INT - BITS_PER_BYTE*d - BITS_PER_BYTE;
  // int v = (a[lo] >> shift) & MASK;
  int v = charAt(a[lo], d);

  int i = lo + 1;
  while (i <= gt) {
    // int t = (a[i] >> shift) & MASK;
    int t = charAt(a[i], d);
    if (t < v)
      exch(a, lt++, i++);
    else if (t > v)
      exch(a, i, gt--);
    else
      i++;
  }

  // a[lo..lt-1] < v = a[lt..gt] < a[gt+1..hi].
  quick3(a, lo, lt - 1, d);
  if (v >= 0) quick3(a, lt, gt, d + 1);
  quick3(a, gt + 1, hi, d);

  // no more bits ???
  // assume no more is d == 7.
  // if (d == 8) return;
}

// threaded - with cutoff as param
// void quick3_t(unsigned long long a[], int lo, int hi, int d, int cutoff) {
void *quick3_t(void *params) {
  q3_threadcount++;

  struct q3_params p = *((struct q3_params *)params);

  // cutoff to insertion sort for small subarrays
  // if (hi <= lo ) {
  if (p.hi <= p.lo + p.cutoff) {
    // insertion_sort(p.a, p.lo, p.hi+1); //the hi+1 is right but not as
    // intended from this perspective. consider going back to change the
    // insertion sort interface.
    qsort(p.a + p.lo, p.hi - p.lo + 1, sizeof(unsigned long long), cmpfunc);
    return NULL;
  }

  // no more bits
  if (p.d == 7) return NULL;

  int lt = p.lo;
  int gt = p.hi;

  int v = charAt(p.a[p.lo], p.d);

  int i = p.lo + 1;
  while (i <= gt) {
    int t = charAt(p.a[i], p.d);
    if (t < v)
      exch(p.a, lt++, i++);
    else if (t > v)
      exch(p.a, i, gt--);
    else
      i++;
  }

  struct q3_params left = {
      .a = p.a, .lo = p.lo, .hi = lt - 1, .d = p.d, .cutoff = p.cutoff};

  struct q3_params mid = {
      .a = p.a, .lo = lt, .hi = gt, .d = p.d + 1, .cutoff = p.cutoff};

  struct q3_params right = {
      .a = p.a, .lo = gt + 1, .hi = p.hi, .d = p.d, .cutoff = p.cutoff};

  // a[lo..lt-1] < v = a[lt..gt] < a[gt+1..hi].
  pthread_t tl;
  pthread_t tm;

  pthread_create(&tl, NULL, quick3_t, &left);

  if (v >= 0) {
    pthread_create(&tm, NULL, quick3_t, &mid);
  }

  quick3_t(&right);
  pthread_join(tl, NULL);
  if (v >= 0) {
    pthread_join(tm, NULL);
  }

  return NULL;
}

// API
void quick3_sort(unsigned long long a[], int len) {
  /*
  unsigned long long *aux = malloc(len * sizeof(*aux));
if (!aux) {
perror("Unable to allocate auxilliary array");
return;
}
  quick3(a, 0, len-1, 0, aux);
  */

  quick3(a, 0, len - 1, 0);
}

// API
void quick3_sort_t(unsigned long long a[], int len, int cutoff) {
  struct q3_params p = {
      .a = a, .lo = 0, .hi = len - 1, .d = 0, .cutoff = cutoff};

  quick3_t(&p);

  printf("[quick3_t] total threads used: %ld\n", q3_threadcount);
}
