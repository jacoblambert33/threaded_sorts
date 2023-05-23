
#include "p_merge_sort.h"

#include <assert.h>
#include <pthread.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"
#include "p_merge_new.h"

/*
P-MERGE-AUX(A; p1; r1; p2; r2; B; p3)
1 if p1 > r1 and p2 > r2 // are both subarrays empty?
2 return
3 if r1 - p1 < r2 - p2 // second subarray bigger?
4 exchange p1 with p2 // swap subarray roles
5 exchange r1 with r2
6 q1 = (p1 + r1)/2 // midpoint of A[p1 : r1]
7 x = A[q1]		// median of A[p1 : r1] is pivot x
8 q2 = FIND-SPLIT-POINT(A; p2; r2; x) // split A[p2 : r2] around x
9 q3 = p3 + (q1 - p1) + (q2 - p2)  // where x belongs in B...
10 B[q3] = x // ... put it there
11 // Recursively merge A[p1 : q1 - 1] and A[p2 : q2 - 1] into B[p3 : q3 - 1].
12 spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
13 // Recursively merge A[q1 + 1 : r1] and A[q2 : r2] into B[q3 + 1 : r3].
14 spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
15 sync // wait for spawns
"
*/

/* MAYBE  use the individual component in p_merge_new.h
        but right now that's private and will stay that way.
worried about conflicting types as i attempt to retain both this and that
version
*/

void p_merge_aux(unsigned long long a[], int p1, int r1, int p2, int r2,
                 unsigned long long b[], int p3) {
  if ((p1 > r1) && (p2 > r2)) return;

  if ((r1 - p1) < (r2 - p2)) {
    // exch(a, p1, p2);
    // exch(a, r1, r2);
    int t1 = p1;
    p1 = p2;
    p2 = t1;
    int t2 = r1;
    r1 = r2;
    r2 = t2;
  }

  int q1 = (p1 + r1) / 2;

  unsigned long long x = a[q1];

  int q2 = find_split_point(a, p2, r2, x);

  int q3 = p3 + (q1 - p1) + (q2 - p2);

  b[q3] = x;

  p_merge_aux(a, p1, q1 - 1, p2, q2 - 1, b, p3);
  p_merge_aux(a, q1 + 1, r1, q2, r2, b, q3 + 1);
}

/*
P-MERGE(A; p; q; r)
1 let B[p : r] be a new array // allocate scratch array
2 P-MERGE-AUX (A; p; q; q + 1; r; B; p) // merge from A into B
3 parallel for i = p to r // copy B back to A in parallel
4 A[i] = B[i]
*/
void p_merge(unsigned long long a[], int p, int q, int r) {
  // int len = r-p+1;
  int len = r + 1;
  unsigned long long *aux = malloc(len * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }
  p_merge_aux(a, p, q, q + 1, r, aux, p);

  for (int i = p; i < len; i++) a[i] = aux[i];

  free(aux);
}

// ENTRY POINT - all the work done in functions from inside this function.
//	this is the API, however, and the only function required by the header.
//  NOTE: implementing only this function will not sort anything.

// public API
void pms_4(unsigned long long a[], int p, int r) {
  // if the size of the array is 0 or 1 elements:
  if (p >= r) return;

  // get midpoint index.
  int q = (p + r) / 2;

  pms_4(a, p, q);
  pms_4(a, q + 1, r);
  p_merge(a, p, q, r);
}

/* my refactoring below
-----------------------------------------------------
*/

void p_merge_jml(unsigned long long a[], unsigned long long b[], int p, int q,
                 int r) {
  p_merge_aux(a, p, q, q + 1, r, b, p);

  for (int i = p; i < r + 1; i++) a[i] = b[i];
}

void p_merge_sort_jml(unsigned long long a[], unsigned long long b[], int p,
                      int r) {
  // if the size of the array is 0 or 1 elements:
  if (p >= r) return;

  // get midpoint index.
  int q = (p + r) / 2;

  p_merge_sort_jml(a, b, p, q);
  p_merge_sort_jml(a, b, q + 1, r);
  p_merge_jml(a, b, p, q, r);
}

void pms_jml(unsigned long long a[], int p, int r) {
  int length = r - p + 1;

  unsigned long long *aux = malloc(length * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }

  p_merge_sort_jml(a, aux, p, r);

  free(aux);
}
