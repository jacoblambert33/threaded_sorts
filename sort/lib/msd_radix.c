#include "msd_radix.h"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"
#include "insertion.h"

/*
#define BITS_PER_BYTE 8					// probably does not
change
#define BITS 64
// each int is 64 bits
//int BITS = 64;
// each int is 64 bits
#define R (1 << BITS_PER_BYTE)			// each byte is between 0 and
255
#define MASK R - 1 // 0xFF #define w ( BITS / BITS_PER_BYTE )		// each
int is 8 bytes
*/

// Adapted from:
// https://algs4.cs.princeton.edu/51radix/MSD.java.html
//  and
// https://www.coursera.org/learn/algorithms-part2/lecture/gFxwG/msd-radix-sort

// MSD sort from a[lo] to a[hi], starting at the dth byte
// private static void sort(int[] a, int lo, int hi, int d, int[] aux) {
void msd(unsigned long long a[], int lo, int hi, int d,
         unsigned long long aux[]) {
  int BITS_PER_BYTE = 8;
  // int BITS = 64;
  // int R = 1 << BITS_PER_BYTE;
  int BITS_PER_INT = 64;
  int R = 256;
  int MASK = R - 1;  // 0xFF, or 255.
  // int w = BITS / BITS_PER_BYTE;
  int CUTOFF = 15;

  // cutoff to insertion sort for small subarrays
  // if (hi <= lo ) {
  if (hi <= lo + CUTOFF) {
    insertion_sort(
        a, lo,
        hi +
            1);  // the hi+1 is right but not as intended from this perspective.
                 // consider going back to change the insertion sort interface.
    return;
  }

  // no more bits
  // assume no more is d == 7.
  // if (d == 8) return;

  // compute frequency counts (need R = 256)
  // int *count = calloc(R+1, sizeof(*count));
  int *count = calloc(R + 2, sizeof(*count));
  if (!count) {
    perror("Unable to allocate count array");
    return;
  }

  int shift = BITS_PER_INT - BITS_PER_BYTE * d - BITS_PER_BYTE;
  for (int i = lo; i <= hi; i++) {
    int c = (a[i] >> shift) & MASK;
    count[c + 1]++;
  }

  // transform counts to indices
  for (int r = 0; r < R; r++) count[r + 1] += count[r];

  // not true for UNSIGNED ints. this is for negative numbers:
  //  see:
  //  https://stackoverflow.com/questions/25597831/questions-on-an-implementation-of-radix-sort-in-java
  // for most significant byte, 0x80-0xFF comes before 0x00-0x7F
  /*
          if (d == 0) {
                          int shift1 = count[R] - count[R/2];
                          int shift2 = count[R/2];
                          count[R] = shift1 + count[1];   // to simplify
     recursive calls later for (int r = 0; r < R/2; r++) count[r] += shift1; for
     (int r = R/2; r < R; r++) count[r] -= shift2;
          }
  */

  // distribute
  for (int i = lo; i <= hi; i++) {
    int c = (a[i] >> shift) & MASK;
    aux[count[c]++] = a[i];
  }

  // copy back
  for (int i = lo; i <= hi; i++) a[i] = aux[i - lo];

  // no more bits
  // assume no more is d == 7.
  // move up and try 8
  if (d == 7) return;

  ///* assume no special case.
  // special case for most significant byte
  // if (d == 0 && count[R/2] > 0)
  //		msd(a, lo, lo + count[R/2] - 1, d+1, aux);

  // special case for other bytes
  // if (d != 0 && count[0] > 0)
  if (count[0] > 0) msd(a, lo, lo + count[0] - 1, d + 1, aux);
  //*/

  // recursively sort for each character
  // (could skip r = R/2 for d = 0 and skip r = R for d > 0)
  for (int r = 0; r < R; r++)
    if (count[r + 1] > count[r])
      msd(a, lo + count[r], lo + count[r + 1] - 1, d + 1, aux);
}

// API
void msd_sort(unsigned long long a[], int len) {
  unsigned long long *aux = malloc(len * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate auxilliary array");
    return;
  }

  msd(a, 0, len - 1, 0, aux);
}
