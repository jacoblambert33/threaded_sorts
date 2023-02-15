
#include"basic_par_merge.h"
// #include<stdint.h> //unsure if needed
#include<assert.h>
#include<stddef.h>
#include<stdlib.h>
#include<stdio.h>
#include<pthread.h>
#include"helpers_sort.h"
#include"insertion.h"
#include"quick.h"
#include"serial_merge.h"
#include"ull_cmp.h"

struct quad_params {
    unsigned long long *a;
		unsigned long long *aux; 
		int lo; 
		int hi;
};

size_t threadcount = 0; 

// these results cutting off to insertion sort. 

//to sort 10k random uint64_t (or long long ints) (work out exactly)
//int CUTOFF = 7;
//Used 2044 threads.
//Time taken 4 seconds 4070 milliseconds 802 microseconds 
//int CUTOFF = 15;
//Used 1022 threads.
//Time taken 1 seconds 1788 milliseconds 658 microseconds 
//int CUTOFF = 31;
//Used 511 threads.
//Time taken 0 seconds 901 milliseconds 23 microseconds 
//int CUTOFF = 63;
//Used 255 threads.
//Time taken 0 seconds 753 milliseconds 983 microseconds 
//int CUTOFF = 127;
//Used 127 threads.
//Time taken 0 seconds 123 milliseconds 275 microseconds 
//int CUTOFF = 255;
//Used 63 threads.
//Time taken 0 seconds 49 milliseconds 662 microseconds 
//int CUTOFF = 511;
//Used 31 threads.
//Time taken 0 seconds 38 milliseconds 689 microseconds 
//int CUTOFF = 1023;
//Used 15 threads.
//Time taken 0 seconds 37 milliseconds 238 microseconds 
//int CUTOFF = 2047;
//Used 7 threads.
//Time taken 0 seconds 61 milliseconds 672 microseconds 
//int CUTOFF = 4095;
//Used 3 threads.
//Time taken 0 seconds 146 milliseconds 374 microseconds 
//int CUTOFF = 8191;
//Used 1 threads.
//Time taken 0 seconds 256 milliseconds 715 microseconds 


// these results cutting off to serial merge sort.
//int CUTOFF = 31;
//Used 39 threads.
//Segmentation fault (core dumped)

// these results for my quick sort. 
//int CUTOFF = 511;
//Used 31 threads.
//Time taken 0 seconds 165 milliseconds 343 microseconds 
//int CUTOFF = 1023;
//Used 15 threads.
//Time taken 0 seconds 152 milliseconds 716 microseconds 
//int CUTOFF = 2047;
//Used 7 threads.
//Time taken 0 seconds 89 milliseconds 995 microseconds
//int CUTOFF = 4095;
int CUTOFF = 8191;
//Used 3 threads.
//Time taken 0 seconds 45 milliseconds 949 microseconds


// this method is currently horrible and i don't know why yet. 

/*		here are hypotheses to explore:

Time taken 4 seconds 4135 milliseconds 404 microseconds 
	Cf. Qsort is around 1 millisecond, Insertion sort around 250 milliseconds, and basic merge sort is about 3 milliseconds on my test input. 

	- there are way too many threads and the cost to start them is prohibitive. 
		current threadcount 9995

	- perhaps i'm doing rework? 
	- perhaps my implementation is wrong - BUT i see that the array is sorted in the end. 

*/ 

void *pmergesort_aux(void *p_quad_params); 

void heremerge(unsigned long long a[], unsigned long long aux[], int lo, int mid, int hi); 




// ENTRY POINT - all the work done in functions from inside this function. this is the API, however, and the only function required by the header. however, implementing only this function is not meant to sort anything by itself. 
//private declarations
//void smergesort_aux(unsigned long long a[], unsigned long long aux[], int lo, int hi);

// public API 
void basic_p_ms(unsigned long long a[], int lo, int hi) {
		size_t len = hi - lo; //length of the input array. 
		//int len = hi - lo; //length of the input array. 
		// Preferred: use size of dereferenced pointer
		unsigned long long *aux = malloc(len * sizeof(*aux)); 
		if (!aux) {
			perror("Unable to allocate array");
			return;
		}

		struct quad_params p = { .a = a, .aux = aux, .lo = lo, .hi = hi };

		//pmergesort_aux(a, aux, lo, hi); 
		pmergesort_aux((void*)&p); 


    assert(is_sorted(a, lo, hi));

		printf("Used %zu threads.\n", threadcount); 

		free(aux);
		//free(p); //need this when i get big again?
} 

// this is what we want to parallelize:
//void pmergesort_aux(unsigned long long a[], unsigned long long aux[], int lo, int hi) {
void *pmergesort_aux(void *p_quad_params) {

		//we have arguments packed into a struct to satisfy the pthreads api. 
		// now, we need to split them into left and right arguments for each thread in our divide and conquer approach. 
		struct quad_params left_p = *((struct quad_params*) p_quad_params); 
		//free(p_quad_params);

		//right can start as a copy of left that we modify. 
		struct quad_params right_p = left_p; 

		// { .a = a, .aux = aux, .lo = lo, .hi = hi };
       
		int lo = left_p.lo; 
		int hi = left_p.hi; 
		int mid = lo + (hi - lo) / 2;

		//if (hi <= lo) return NULL;
		//broken here:	
		if (hi <= lo + CUTOFF) {
			//insertion_sort(left_p.a, lo, hi);
			quick(left_p.a, lo, hi);
			//smergesort(left_p.a, lo, hi);
			// not working bc either i have malloc/free errors in my serial impl OR bc the stack gets too big. it works for 1k but not 10k in this test. 
			//not going to work bc i'm not aware that qsort can let me sort a portion of an array the way my home rolled sorts can. 
			//qsort(left_p.a, hi-lo, sizeof(unsigned long long), cmpfunc);
			return NULL;
    }//*/


		left_p.hi = mid; 
		//right_p.lo = mid+1; //it seems like we're skipping the middle value in insertion sort. BUT i worry if i use the middle value again i might not get correct mergesort behavior. 
		right_p.lo = mid; 
		
		pthread_t t;
		//spawn
		pthread_create(&t, NULL, pmergesort_aux, &left_p);
		threadcount++;
		//main thread can do the right side. 
		pmergesort_aux(&right_p); 

		//sync - wait for left thread.
		// we don't need results so the second argument can be NULL. 
		pthread_join(t, NULL);


		//pmergesort_aux(a, aux, mid + 1, hi);
		heremerge(left_p.a, left_p.aux, lo, mid, hi);
		//printf("current threadcount %zu\n", threadcount);
		return NULL;
} //END pmergesort_aux 



void heremerge(unsigned long long a[], unsigned long long aux[], int lo, int mid, int hi) {

		// precondition: a[lo .. mid] and a[mid+1 .. hi] are sorted subarrays
		assert(is_sorted(a, lo, mid));
		//assert(is_sorted(a, mid+1, hi));
		assert(is_sorted(a, mid, hi));

		// copy to aux[]
		//for (int k = lo; k <= hi; k++) {
		for (int k = lo; k < hi; k++) {
				aux[k] = a[k];
		}

		// merge back to a[]
		int i = lo, j = mid; //j = mid+1;
		//for (int k = lo; k <= hi; k++) {
		for (int k = lo; k < hi; k++) { // originally it was i < mid and j > hi
		if (i >= mid)											 a[k] = aux[j++];
				else if (j >= hi)              a[k] = aux[i++];
				else if (less(aux[j], aux[i])) a[k] = aux[j++];
				else                           a[k] = aux[i++];
		}

		// postcondition: a[lo .. hi] is sorted
    assert(is_sorted(a, lo, hi));
} //END heremerge

