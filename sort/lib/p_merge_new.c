#include "p_merge_new.h"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"

// private declaration so impl is meant to go in c file.
struct p_merge_params {
  unsigned long long *a;
  int p1;
  int r1;
  int p2;
  int r2;
  unsigned long long *b;
  int p3;
  int cutoff;
};

// for debugging/investigation - count the number of threads we're spawning.
int p_merge_new_count = 0;

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
void p_merge_aux_p(unsigned long long a[], int p1, int r1, int p2, int r2,
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

  p_merge_aux_p(a, p1, q1 - 1, p2, q2 - 1, b, p3);
  p_merge_aux_p(a, q1 + 1, r1, q2, r2, b, q3 + 1);
}

// threaded parallel merge
void *p_merge_aux_t(void *params) {
  p_merge_new_count++;

  struct p_merge_params p = *((struct p_merge_params *)params);

  if ((p.p1 > p.r1) && (p.p2 > p.r2)) return NULL;

  if ((p.r1 - p.p1) < (p.r2 - p.p2)) {
    int t1 = p.p1;
    p.p1 = p.p2;
    p.p2 = t1;
    int t2 = p.r1;
    p.r1 = p.r2;
    p.r2 = t2;
  }

  int q1 = (p.p1 + p.r1) / 2;

  unsigned long long x = p.a[q1];

  int q2 = find_split_point(p.a, p.p2, p.r2, x);

  int q3 = p.p3 + (q1 - p.p1) + (q2 - p.p2);

  p.b[q3] = x;

  // serial for small arrays:
  // need a cutoff to avoid huge numbers of threads and the overhead there.
  // one option is to use the length of the first half as a guide:
  int len = p.r1 - p.p1;
  if (len < p.cutoff) {
    p_merge_aux_p(p.a, p.p1, q1 - 1, p.p2, q2 - 1, p.b, p.p3);
    p_merge_aux_p(p.a, q1 + 1, p.r1, q2, p.r2, p.b, q3 + 1);

    return NULL;
  }

  // prep data structures for recursion:
  struct p_merge_params left = {.a = p.a,
                                .p1 = p.p1,
                                .r1 = q1 - 1,
                                .p2 = p.p2,
                                .r2 = q2 - 1,
                                .b = p.b,
                                .p3 = p.p3,
                                .cutoff = p.cutoff};
  struct p_merge_params right = {.a = p.a,
                                 .p1 = q1 + 1,
                                 .r1 = p.r1,
                                 .p2 = q2,
                                 .r2 = p.r2,
                                 .b = p.b,
                                 .p3 = q3 + 1,
                                 .cutoff = p.cutoff};

  pthread_t t;
  pthread_create(&t, NULL, p_merge_aux_t, &right);

  p_merge_aux_t(&left);
  pthread_join(t, NULL);

  return NULL;
}

// API
void p_merge_new(unsigned long long a[], unsigned long long b[], int p, int q,
                 int r) {
  p_merge_aux_p(a, p, q, q + 1, r, b, p);

  for (int i = p; i < r + 1; i++) a[i] = b[i];
}

// API
void p_merge_new_t(unsigned long long a[], unsigned long long b[], int p, int q,
                   int r, int cutoff) {
  struct p_merge_params params = {.a = a,
                                  .p1 = p,
                                  .r1 = q,
                                  .p2 = q + 1,
                                  .r2 = r,
                                  .b = b,
                                  .p3 = p,
                                  .cutoff = cutoff};

  p_merge_aux_t(&params);

  for (int i = p; i < r + 1; i++) a[i] = b[i];

  // printf("[p_merge_new_t] total threads used: %d\n", p_merge_new_count);
}

// END
