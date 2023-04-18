
#include "parallel_merge_sort.h"

#include <assert.h>
#include <pthread.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"
#include "p_merge.h"

struct pms_params {
  unsigned long long *a;
  unsigned long long *result;
  unsigned long long *tmp;
  int lo;
  int hi;
  int start;
};

size_t par_threadcount = 0;

int THIS_CUTOFF =
    31;  // 8191; //4095; //2047; //1023;//511; //255; //127; //63; //31;//15;

void *pms_aux(void *p_quad_params);
void *book_serial(void *pms);
void *book_parallel(void *pms);

// ENTRY POINT - all the work done in functions from inside this function.
//	this is the API, however, and the only function required by the header.
//  NOTE: implementing only this function will not sort anything.

// public API
void pms(unsigned long long a[], int lo, int hi) {
  size_t len = hi - lo;  // + 1; //length of the input array.
  // int len = hi - lo; //length of the input array.
  //  Preferred: use size of dereferenced pointer
  unsigned long long *aux = malloc(len * sizeof(*aux));
  unsigned long long *tmp = malloc(len * sizeof(*tmp));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }
  if (!tmp) {
    perror("Unable to allocate array");
    return;
  }

  struct pms_params p = {
      .a = a, .result = aux, .tmp = aux, .lo = lo, .hi = hi, .start = lo};

  // pmergesort_aux(a, aux, lo, hi);
  // pms_aux((void*)&p);
  // book_serial((void *)&p);
  book_p_merge_sort(a, lo, hi, aux, lo);

  // assert(is_sorted(aux, lo, hi));
  assert(is_sorted(p.result, lo, hi));

  printf("Used %zu threads.\n", par_threadcount);

  for (int i = lo; i < hi; i++) a[i] = aux[i];

  free(aux);
  free(tmp);
  // free(p); //need this when i get big again?
}

// public API
void pms_t(unsigned long long a[], int lo, int hi) {
  size_t len = hi - lo;  // + 1; //length of the input array.
  // int len = hi - lo; //length of the input array.
  //  Preferred: use size of dereferenced pointer
  unsigned long long *aux = malloc(len * sizeof(*aux));
  unsigned long long *tmp = malloc(len * sizeof(*tmp));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }
  if (!tmp) {
    perror("Unable to allocate array");
    return;
  }

  struct pms_params p = {
      .a = a, .result = aux, .tmp = aux, .lo = lo, .hi = hi, .start = lo};

  // pmergesort_aux(a, aux, lo, hi);
  // pms_aux((void*)&p);
  // serial((void*)&p);
  // book_serial((void *)&p);
  book_p_merge_sort(a, lo, hi, aux, lo);

  // assert(is_sorted(aux, lo, hi));
  assert(is_sorted(p.result, lo, hi));

  printf("Used %zu threads.\n", par_threadcount);

  for (int i = lo; i < hi; i++) a[i] = aux[i];

  free(aux);
  free(tmp);
  // free(p); //need this when i get big again?
}

// public API
void book_p_merge_sort_entry(unsigned long long A[], int p, int r,
                             unsigned long long B[], int s) {
  struct pms_params params = {
      .a = A, .result = B, .tmp = B, .lo = p, .hi = r, .start = s};

  book_serial((void *)&params);
  // book_parallel((void*)&params);
}

/*
P-MERGE-SORT (A, p, r, B, s)
1		n = r - p + 1
2		if n == 1
3			B[s] = A[p]
4		else let T[1..n] be a new array
5			q = p + r / 2
6			q' = q - p + 1
7			spawn P-MERGE-SORT(A, p, q, T, 1)
8			P-MERGE-SORT(A, q + 1, r, T, q' + 1)
9			sync
10		P-MERGE(T, 1, q', q' + 1, n, B, s)
*/
void book_p_merge_sort(unsigned long long A[], int p, int r,
                       unsigned long long B[], int s) {
  int n = r - p + 1;
  // int n = r - p;
  if (n == 1)
    B[s] = A[p];
  else {
    // unsigned long long *T = malloc(n * sizeof(*T));
    unsigned long long *T = malloc((n + 1) * sizeof(*T));
    if (!T) {
      perror("Unable to allocate array");
      return;
    }
    int q = (p + r) / 2;
    int qp = q - p + 1;
    // book_p_merge_sort(A, p, q, T, 1);
    // book_p_merge_sort(A, q + 1, r, T, qp + 1);
    // parallel_merge_t(T, 1, qp, qp + 1, n, B, s);
    book_p_merge_sort(A, p, q, T, 1);
    book_p_merge_sort(A, q + 1, r, T, qp + 1);
    // parallel_merge_t(T, 1, qp, qp + 1, n-1, B, s);
    parallel_merge(T, 1, qp, qp + 1, n, B, s);
    // book_p_merge_sort(A, p, q, T, 0);
    // book_p_merge_sort(A, q + 1, r, T, qp+1);
    // parallel_merge(T, p, qp, qp + 1, n-1, B, s);

    // example of working test:
    // parallel_merge(haystack, 0, 7, 8, 15, result, 0);
    free(T);
  }
}

// step 2, book version serial but form required for pthreads:
// void book_p_merge_sort(unsigned long long A[], int p, int r, unsigned long
// long B[], int s) {

void *book_serial(void *prm) {
  struct pms_params input = *((struct pms_params *)prm);

  int n = input.hi - input.lo + 1;
  int r = input.hi;

  if (n == 1)
    input.result[input.start] = input.a[input.lo];
  else {
    unsigned long long *T = malloc(n * sizeof(*T));
    if (!T) {
      perror("Unable to allocate array");
      return NULL;
    }
    int q = (input.lo + input.hi) / 2;
    int qp = q - input.lo + 1;

    struct pms_params left = {.a = input.a,
                              .result = T,
                              .tmp = T,
                              .lo = input.lo,
                              .hi = q,
                              .start = 1};
    book_serial((void *)&left);
    // book_p_merge_sort(left.a, left.lo, q, T, 1);
    struct pms_params right = {.a = input.a,
                               .result = T,
                               .tmp = T,
                               .lo = q + 1,
                               .hi = r,
                               .start = qp + 1};
    book_serial((void *)&right);
    // book_p_merge_sort(left.a, q+1, left.hi, T, qp + 1);
    parallel_merge_t(T, 1, qp, qp + 1, n, input.result, input.start);

    // free(T);
  }
  // free(T);
  return NULL;
}

// STEP 3: parallelize book_serial
/* TODO: WARNING - this is broken. valgrind makes me think the issue is with my
p_merge.c and binary_search.c files. i need to go back through my tests from the
bottom up to correct this. it works on arrays <= 40 but not 50 - where i get a
corrupt or double free. however, i need to move on for now so i will use the
serial version at this time. fwiw, book_serial is buggy too - but the issue
happens at 100k items. 10k items appears fine.
*/
void *book_parallel(void *prm) {
  struct pms_params input = *((struct pms_params *)prm);

  int n = input.hi - input.lo + 1;
  int r = input.hi;

  if (n == 1)
    input.result[input.start] = input.a[input.lo];
  else {
    unsigned long long *T = malloc(n * sizeof(*T));
    // unsigned long long *T = malloc((n+1) * sizeof(*T));
    // unsigned long long *T = malloc((n+2) * sizeof(*T));
    if (!T) {
      perror("Unable to allocate array");
      return NULL;
    }
    int q = (input.lo + input.hi) / 2;
    int qp = q - input.lo + 1;

    struct pms_params left = {.a = input.a,
                              .result = T,
                              .tmp = T,
                              .lo = input.lo,
                              .hi = q,
                              .start = 1};
    struct pms_params right = {.a = left.a,
                               .result = T,
                               .tmp = T,
                               .lo = q + 1,
                               .hi = r,
                               .start = qp + 1};

    if (n < THIS_CUTOFF) {
      book_serial((void *)&left);
      book_serial((void *)&right);
      // serial(&left);
      // serial(&right);
    } else {
      pthread_t t;
      // spawn
      pthread_create(&t, NULL, book_parallel, &left);
      par_threadcount++;
      // main thread can do the right side.

      book_parallel(&right);

      // sync - wait for left thread.
      //  we don't need results so the second argument can be NULL.
      pthread_join(t, NULL);
    }

    // book_p_merge_sort(left.a, left.lo, q, T, 1);
    // book_p_merge_sort(left.a, q+1, left.hi, T, qp + 1);
    parallel_merge_t(T, 1, qp, qp + 1, n, input.result, input.start);

    // free(T);
  }
  // free(T);
  return NULL;
}

void *pms_aux(void *fpms_params) {
  struct pms_params left = *((struct pms_params *)fpms_params);
  int lo = left.lo;
  int hi = left.hi;
  int mid = lo + (hi - lo) / 2;
  int mid_prime = mid - lo + 1;

  // unsigned long long *result = left.aux;

  int len = hi - lo;  // + 1;
  if (len == 1)
    left.result[left.start] = left.a[left.lo];
  else {
    unsigned long long *tmp = malloc(len * sizeof(*tmp));

    if (!tmp) {
      perror("Unable to allocate array");
      return NULL;
    }

    // right can start as a copy of left that we modify.
    struct pms_params right = left;

    // left.lo = left.lo; //unchanged
    left.hi = mid;
    left.start = 1;
    left.tmp = tmp;

    // right.hi = left.hi //unchanged.
    right.lo = mid;
    right.start = mid_prime + 1;
    right.tmp = tmp;

    if (len < THIS_CUTOFF) {
      // BREAKING THIS ON PURPOSE: come back here later TODO
      // serial(&left);
      // serial(&right);
      assert(0);
    } else {
      pthread_t t;
      // spawn
      pthread_create(&t, NULL, pms_aux, &left);
      par_threadcount++;
      // main thread can do the right side.

      pms_aux(&left);
      pms_aux(&right);

      // sync - wait for left thread.
      //  we don't need results so the second argument can be NULL.
      pthread_join(t, NULL);
    }

    parallel_merge_t(left.tmp, 1, mid_prime, mid_prime + 1, len, left.result,
                     left.start);

    free(tmp);

    return NULL;
  }  // END else
  return NULL;
}  // END pmergesort_aux
