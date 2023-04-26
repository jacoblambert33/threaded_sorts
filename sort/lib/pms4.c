

#include <assert.h>
#include <pthread.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "helpers_sort.h"
#include "p_merge_new.h"
#include "pms4.h"


// for debugging/investigation - count the number of threads we're spawning.
int pms4_count = 0;

// private declaration so impl is meant to go in c file.
struct p_merge_sort_params {
  unsigned long long *a;
  unsigned long long *b;
  int p;
  int q;
  int r;
  int cutoff;
};



// SERIAL:
// internal function that does the work:
// uses merge from 'p_merge_new.h'
void p_merge_sort_s(unsigned long long a[], unsigned long long b[], int p,
                      int r) {
  // if the size of the array is 0 or 1 elements:
  if (p >= r) return;

  // get midpoint index.
  int q = (p + r) / 2;

  p_merge_sort_s(a, b, p, q);
  p_merge_sort_s(a, b, q + 1, r);
  p_merge_new(a, b, p, q, r);
}

// ENTRY POINT - all the work done in functions from inside this function.
void pms4_s(unsigned long long a[], int p, int r) {
  int length = r - p + 1;

  unsigned long long *aux = malloc(length * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }

  p_merge_sort_s(a, aux, p, r);

  free(aux);
}

// THREADED:
// internal function that does the work:
// uses merge from 'p_merge_new.h'
void *p_merge_sort_t(void *params) {

	pms4_count++;

	struct p_merge_sort_params p = *((struct p_merge_sort_params *)params);

  // if the size of the array is 0 or 1 elements:
  if (p.p >= p.r) return NULL;

  // get midpoint index.
  int q = (p.p + p.r) / 2;

	// serial for small arrays:
	// need a cutoff to avoid huge numbers of threads and the overhead there. 
	// one option is to use the length of the first half as a guide:
	int len = p.r - p.p;
	if (len < p.cutoff) {
		
		p_merge_sort_s(p.a, p.b, p.p, q);  
		p_merge_sort_s(p.a, p.b, q+1, p.r);  

		p_merge_new(p.a, p.b, p.p, q, p.r);

		return NULL;
	}



	//prep data structures for recursion:
	struct p_merge_sort_params left = { .a = p.a, .b = p.b, .p = p.p, .r = q, .cutoff = p.cutoff }; 
	struct p_merge_sort_params right = { .a = p.a, .b = p.b, .p = q+1, .r = p.r, .cutoff = p.cutoff }; 

	pthread_t t; 
	pthread_create(&t, NULL, p_merge_sort_t, &right);

  p_merge_sort_t(&left);
	pthread_join(t, NULL);

  p_merge_new_t(p.a, p.b, p.p, q, p.r, p.cutoff);
	//for now i'm using the same size cutoff for both mergesort and merge. 

	return NULL;
}


// ENTRY POINT - all the work done in functions from inside this function.
void pms4_t(unsigned long long a[], int p, int r, int cutoff) {
  int length = r - p + 1;

  unsigned long long *aux = malloc(length * sizeof(*aux));
  if (!aux) {
    perror("Unable to allocate array");
    return;
  }

	struct p_merge_sort_params params = { .a = a, .b = aux, .p = p, .r = r, .cutoff = cutoff };  

  p_merge_sort_t(&params);

	printf("[pms4_t] total threads used: %d\n", pms4_count);  

  free(aux);
}
