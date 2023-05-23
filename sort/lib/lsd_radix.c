#include "lsd_radix.h"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"

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
// https://algs4.cs.princeton.edu/51radix/LSD.java.html

// API
void lsd_sort(unsigned long long a[], int len) {
  int BITS_PER_BYTE = 8;
  int BITS = 64;
  int R = 1 << BITS_PER_BYTE;
  int MASK = R - 1;
  int w = BITS / BITS_PER_BYTE;

  unsigned long long *aux = malloc(len * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate auxilliary array");
    return;
  }

  for (int d = 0; d < w; d++) {
    // int *count = malloc((R+1) * sizeof(*count));
    int *count = calloc((R + 1), sizeof(*count));
    if (!count) {
      perror("Unable to allocate count array");
      return;
    }

    // compute frequency counts
    for (int i = 0; i < len; i++) {
      int c = (a[i] >> BITS_PER_BYTE * d) & (MASK);
      // long c = (a[i] >> BITS_PER_BYTE*d) &  ( MASK );
      count[c + 1]++;
    }

    // compute cumulates
    for (int r = 0; r < R; r++) count[r + 1] += count[r];

    /* is this only for int32?
    // for most significant byte, 0x80-0xFF comes before 0x00-0x7F
    if (d == w-1) {
            int shift1 = count[R] - count[R/2];
            int shift2 = count[R/2];
            for (int r = 0; r < R/2; r++)
                    count[r] += shift1;
            for (int r = R/2; r < R; r++)
                    count[r] -= shift2;
    }
    */

    // move data
    for (int i = 0; i < len; i++) {
      int c = (a[i] >> BITS_PER_BYTE * d) & (MASK);
      aux[count[c]++] = a[i];
    }

    /* CONSIDER LATER
    // optimization: swap a[] and aux[] references instead of copying
    // (since w is even, the argument a[] to sort() will be the array
    // with the sorted integers)
    int[] temp = a;
            a = aux;
            aux = temp;
    }
    */

    // copy back
    for (int i = 0; i < len; i++) a[i] = aux[i];
  }
}  // END lsd_sort
