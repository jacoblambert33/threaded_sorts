#include"binary_search.h"
#include"p_merge.h"
#include"debug.h"
#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
#include<assert.h>



int cmpfunc (const void * a, const void * b); 

bool is_sorted(unsigned long long a[], int lo, int hi); 

void run_basic_merge(); 

void run_basic_merge_t(); 

void run_many_parts_merge(); 

void run_parm(); 

void run_book_parm(); 



int main(int argc, char **argv) {

/*
	run_basic_merge(); 
	run_basic_merge_t(); 
	run_many_parts_merge();
	run_parm();
*/
	run_book_parm();

	return 0; 

}


int cmpfunc (const void * a, const void * b) {

    const unsigned long long ai = *( const unsigned long long * )a;
    const unsigned long long bi = *( const unsigned long long * )b;

    if( ai < bi )
    {
        return -1;
    }
    else if( ai > bi )
    {
        return 1;
    }
    else
    {
        return 0;
    }

}

void run_basic_merge() {

	printf("TEST: serial version parallel merge; one merge two sorted arrays size 50 each\n");
	int n = 50;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack1 = malloc((sizeof *haystack1)*n);
	unsigned long long *haystack2 = malloc((sizeof *haystack2)*n);
	unsigned long long *haystack = malloc((sizeof *haystack)*n*2);
	unsigned long long *result = malloc((sizeof *result)*n*2);
	
	for (int i = 0; i < n; i++) {
		haystack1[i] = rand(); 
	}
	for (int i = 0; i < n; i++) {
		haystack2[i] = rand(); 
	}

	qsort(haystack1, n, sizeof(*haystack1), cmpfunc); 
	qsort(haystack2, n, sizeof(*haystack2), cmpfunc); 


	//make two sorted halves
	for (int i = 0; i < n*2; i++) {

		if (i < n)
			haystack[i] = haystack1[i];
		else
			haystack[i] = haystack2[i-n];	
	}
	
	//see if i have two sorted arrays:
	if (DEBUG) {
		printf("setup array is: \n");
		for (int i = 0; i < n*2; i++) {
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	//invoke p_merge once
	parallel_merge(haystack, result, 0, 49, 50, 99, 0); 

	
	//see if i have two sorted arrays:
	if (DEBUG) {
		printf("\nfinished:\n\n");
		for (int i = 0; i < n*2; i++) {
			//printf("%lld\n", result[i]);
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	bool is_merged = is_sorted(result, 0, 2*n); 

	if (DEBUG) 
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
	assert(is_merged); 
	printf("\tSUCCESS\n");
}

void run_basic_merge_t() {

	printf("TEST: threaded merge; one merge two sorted arrays size 50 each\n");
	int n = 50;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack1 = malloc((sizeof *haystack1)*n);
	unsigned long long *haystack2 = malloc((sizeof *haystack2)*n);
	unsigned long long *haystack = malloc((sizeof *haystack)*n*2);
	unsigned long long *result = malloc((sizeof *result)*n*2);
	
	for (int i = 0; i < n; i++) {
		haystack1[i] = rand(); 
	}
	for (int i = 0; i < n; i++) {
		haystack2[i] = rand(); 
	}

	qsort(haystack1, n, sizeof(*haystack1), cmpfunc); 
	qsort(haystack2, n, sizeof(*haystack2), cmpfunc); 


	//make two sorted halves
	for (int i = 0; i < n*2; i++) {

		if (i < n)
			haystack[i] = haystack1[i];
		else
			haystack[i] = haystack2[i-n];	
	}
	
	//see if i have two sorted arrays:
	if (DEBUG) {
		printf("setup array is: \n");
		for (int i = 0; i < n*2; i++) {
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	//invoke p_merge once
	//parallel_merge(haystack, result, 0, 49, 50, 99, 0); 
	api_par_merge(haystack, 100, 0, 49, 50, 99, 0); 

	
	//see if i have two sorted arrays:
	if (DEBUG) {
		printf("\nfinished:\n\n");
		for (int i = 0; i < n*2; i++) {
			//printf("%lld\n", result[i]);
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	bool is_merged = is_sorted(result, 0, 2*n); 

	if (DEBUG) 
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
	assert(is_merged); 
	printf("\tSUCCESS\n");
}

void run_many_parts_merge() {

	printf("TEST: serial merge; merge eight sorted arrays size eight each in three rounds.\n");
	int n = 8;
	int sets = 8;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack = malloc((sizeof *haystack)*n*sets);
	unsigned long long *result = malloc((sizeof *result)*n*sets);
	unsigned long long *result2 = malloc((sizeof *result)*n*sets);
	unsigned long long *result3 = malloc((sizeof *result)*n*sets);
	
	for (int i = 0; i < n*sets; i++) {
		haystack[i] = rand(); 
	}

	for (int i = 0; i < sets; i++) {
		qsort((haystack+i*n), n, sizeof(*haystack), cmpfunc); 
	}

	//see if i have multiple sorted parts:
	if (DEBUG) {
		printf("setup array is: \n");
		for (int i = 0; i < n*sets; i++) {
			if (i % n == 0) printf("\n");
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	//confirm that i have sorted parts: 
	
	for (int i = 0; i < sets; i++) {
		
		//unsigned long long *start_ptr = haystack+(i*n);
		int start_idx = i*n;
		int end_idx = (i+1)*n;
		//bool is_merged = is_sorted(start_ptr, start_idx, end_idx); 
		bool is_merged = is_sorted(haystack, start_idx, end_idx); 
		assert(is_merged); 
		/*
		if (!is_merged)
			printf("sorted run is not sorted %d == %d\n", i, is_merged); 
		*/
		//printf("sorted run %d == %d\n", i, is_merged); 
		/*if (!is_merged) {
			for (int j = start_idx; j < end_idx; j++) {
				//printf("%lld\n", *(start_ptr+j));
				printf("%lld\n", haystack[j]);
			}
			printf("\n");
			
		} */
	}

	//merge the pairs. 
	// easiest way: explicit based on eight sets of 16;
	//  take six arbitrary groups and merge into three, then two then one.
	parallel_merge(haystack, result, 0, 7, 8, 15, 0); 
	parallel_merge(haystack, result, 16, 23, 24, 31, 16); 
	parallel_merge(haystack, result, 32, 39, 40, 47, 32); 
	parallel_merge(haystack, result, 48, 55, 56, 63, 48); 

	bool is_merged = is_sorted(result, 0, 16); 

	is_merged = is_sorted(result, 16, 32); 

	is_merged = is_sorted(result, 32, 48); 

	is_merged = is_sorted(result, 48, 64); 

	//round2:
	parallel_merge(result, result2, 0, 15, 16, 31, 0); 
	parallel_merge(result, result2, 32, 47, 48, 63, 32); 

	is_merged = is_sorted(result2, 0, 32); 

	is_merged = is_sorted(result2, 33, 64); 

	//round3:

	parallel_merge(result2, result3, 0, 31, 32, 63, 0); 

	is_merged = is_sorted(result3, 0, 64); 

	assert(is_merged); 

	printf("\tSUCCESS\n");

}

void run_parm() {

	printf("TEST: threaded merge; merge eight sorted arrays size eight each in three rounds.\n");
	int n = 8;
	int sets = 8;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack = malloc((sizeof *haystack)*n*sets);
	//unsigned long long *result = malloc((sizeof *result)*n*sets);
	//unsigned long long *result2 = malloc((sizeof *result)*n*sets);
	//unsigned long long *result3 = malloc((sizeof *result)*n*sets);
	
	for (int i = 0; i < n*sets; i++) {
		haystack[i] = rand(); 
		//haystack[i] = rand() % 20; //make nums something i can easily see to debug. 
	}

	for (int i = 0; i < sets; i++) {
		qsort((haystack+i*n), n, sizeof(*haystack), cmpfunc); 
	}

	if (DEBUG) {
		//see if i have multiple sorted parts:
		printf("setup array is: \n");
		for (int i = 0; i < n*sets; i++) {
			if (i % n == 0) printf("\n");
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	//confirm that i have sorted parts: 
	
	for (int i = 0; i < sets; i++) {
		
		//unsigned long long *start_ptr = haystack+(i*n);
		int start_idx = i*n;
		int end_idx = (i+1)*n;
		//bool is_merged = is_sorted(start_ptr, start_idx, end_idx); 
		bool is_merged = is_sorted(haystack, start_idx, end_idx); 
		assert(is_merged);
		/*
		if (!is_merged)
			printf("sorted run is not sorted %d == %d\n", i, is_merged); 
		*/
	}

	//merge the pairs. 
	// easiest way: explicit based on eight sets of 16;
	//  take six arbitrary groups and merge into three, then two then one.
	api_par_merge(haystack, 64, 0, 7, 8, 15, 0); 
	api_par_merge(haystack, 64, 16, 23, 24, 31, 16); 
	api_par_merge(haystack, 64, 32, 39, 40, 47, 32); 
	api_par_merge(haystack, 64, 48, 55, 56, 63, 48); 

	bool is_merged = is_sorted(haystack, 0, 16); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 16, 32); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 32, 48); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 48, 64); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	//round2:

	api_par_merge(haystack, 64, 0, 15, 16, 31, 0); 
	api_par_merge(haystack, 64, 32, 47, 48, 63, 32); 

	is_merged = is_sorted(haystack, 0, 32); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 33, 64); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
	//round3:

	api_par_merge(haystack, 64, 0, 31, 32, 63, 0); 

	is_merged = is_sorted(haystack, 0, 64); 
	if (DEBUG)
	printf("\nfinal: are the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

	assert(is_merged); 
	printf("\tSUCCESS\n"); 

}


void run_book_parm() {

	printf("TEST: threaded merge; merge eight sorted arrays size eight each in three rounds.\n");
	int n = 8;
	int sets = 8;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack = malloc((sizeof *haystack)*n*sets);
	unsigned long long *result = malloc((sizeof *result)*n*sets);
	//unsigned long long *result2 = malloc((sizeof *result)*n*sets);
	//unsigned long long *result3 = malloc((sizeof *result)*n*sets);
	
	for (int i = 0; i < n*sets; i++) {
		//haystack[i] = rand(); 
		haystack[i] = rand() % 20; //make nums something i can easily see to debug. 
	}

	for (int i = 0; i < sets; i++) {
		qsort((haystack+i*n), n, sizeof(*haystack), cmpfunc); 
	}

	if (DEBUG) {
		//see if i have multiple sorted parts:
		printf("setup array is: \n");
		for (int i = 0; i < n*sets; i++) {
			if (i % n == 0) printf("\n");
			printf("%lld\n", haystack[i]);
		}
		printf("\n");
	}

	//confirm that i have sorted parts: 
	
	for (int i = 0; i < sets; i++) {
		
		//unsigned long long *start_ptr = haystack+(i*n);
		int start_idx = i*n;
		int end_idx = (i+1)*n;
		//bool is_merged = is_sorted(start_ptr, start_idx, end_idx); 
		bool is_merged = is_sorted(haystack, start_idx, end_idx); 
		assert(is_merged);
		/*
		if (!is_merged)
			printf("sorted run is not sorted %d == %d\n", i, is_merged); 
		*/
	}

	//merge the pairs. 
	// easiest way: explicit based on eight sets of 16;
	//  take six arbitrary groups and merge into three, then two then one.
	book_api_par_merge(haystack, 0, 7, 8, 15, result, 0); 
	book_api_par_merge(haystack, 16, 23, 24, 31, result, 16); 
	book_api_par_merge(haystack, 32, 39, 40, 47, result, 32); 
	book_api_par_merge(haystack, 48, 55, 56, 63, result, 48); 

	bool is_merged = is_sorted(result, 0, 16); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 16, 32); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 32, 48); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 48, 64); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	//round2:
	//before the round i need to reset haystack to have the results of the last round. i think i can copy results into it as is. i think i can leave results alone and it will be further altered. 
	for (int i = 0; i < n*sets; i++) {
		haystack[i] = result[i]; 
	}



	book_api_par_merge(haystack, 0, 15, 16, 31, result, 0); 
	book_api_par_merge(haystack, 32, 47, 48, 63, result, 32); 

	is_merged = is_sorted(result, 0, 32); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 33, 64); 
	if (DEBUG)
	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
	//round3:
	//same idea as above to start round 2. 
	for (int i = 0; i < n*sets; i++) {
		haystack[i] = result[i]; 
	}


	book_api_par_merge(haystack, 0, 31, 32, 63, result, 0); 

	if (DEBUG) {
		// is it right?
		for (int i = 0; i < n*sets; i++) {
			printf("%lld, ", result[i]);
		}
		printf("\n");
	}



	is_merged = is_sorted(result, 0, 64); 
	if (DEBUG)
	printf("\nfinal: are the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");

	assert(is_merged); 
	printf("\tSUCCESS\n"); 

}

