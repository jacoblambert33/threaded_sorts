#include"binary_search.h"
#include"p_merge.h"
#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>

int cmpfunc (const void * a, const void * b); 

bool is_sorted(unsigned long long a[], int lo, int hi); 

void run_basic_merge(); 

void run_many_parts_merge(); 

void run_many_parts_par_merge(); 



int main(int argc, char **argv) {

	//run_basic_merge(); 
	//run_many_parts_merge();
	run_many_parts_par_merge();

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
	printf("setup array is: \n");
	for (int i = 0; i < n*2; i++) {
		printf("%lld\n", haystack[i]);
	}
	printf("\n");


	//invoke p_merge once
	//parallel_merge(haystack, result, 0, 49, 50, 99, 0); 
	api_par_merge(haystack, 0, 49, 50, 99, 0); 

	
	//see if i have two sorted arrays:
	printf("\nfinished:\n\n");
	for (int i = 0; i < n*2; i++) {
		//printf("%lld\n", result[i]);
		printf("%lld\n", haystack[i]);
	}
	printf("\n");

	bool is_merged = is_sorted(result, 0, 2*n); 
	//printf("is_merged == %d\n", is_merged); 

	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
}

void run_many_parts_merge() {

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
	printf("setup array is: \n");
	for (int i = 0; i < n*sets; i++) {
		if (i % n == 0) printf("\n");
		printf("%lld\n", haystack[i]);
	}
	printf("\n");

	//confirm that i have sorted parts: 
	
	for (int i = 0; i < sets; i++) {
		
		//unsigned long long *start_ptr = haystack+(i*n);
		int start_idx = i*n;
		int end_idx = (i+1)*n;
		//bool is_merged = is_sorted(start_ptr, start_idx, end_idx); 
		bool is_merged = is_sorted(haystack, start_idx, end_idx); 
		if (!is_merged)
			printf("sorted run is not sorted %d == %d\n", i, is_merged); 
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
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 16, 32); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 32, 48); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result, 48, 64); 
	//printf("sorted run == %d\n", is_merged); 

	//round2:
	// i think i need a new results array to do another round....

	parallel_merge(result, result2, 0, 15, 16, 31, 0); 
	parallel_merge(result, result2, 32, 47, 48, 63, 32); 

	is_merged = is_sorted(result2, 0, 32); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result2, 33, 64); 
	//printf("sorted run == %d\n", is_merged); 

	//round3:
	// i think i need a new results array to do another round....

	parallel_merge(result2, result3, 0, 31, 32, 63, 0); 

	is_merged = is_sorted(result3, 0, 64); 
	printf("sorted run == %d\n", is_merged); 


	//recursive	


	//parallel_merge(haystack, result, 0, 49, 50, 99, 0); 
/*
	//make two sorted halves
	for (int i = 0; i < n*2; i++) {

		if (i < n)
			haystack[i] = haystack1[i];
		else
			haystack[i] = haystack2[i-n];	
	}
	
	//see if i have two sorted arrays:
	printf("setup array is: \n");
	for (int i = 0; i < n*2; i++) {
		printf("%lld\n", haystack[i]);
	}
	printf("\n");


	//invoke p_merge once
	//parallel_merge(haystack, result, 0, 49, 50, 99, 0); 

	//parallel_merge(haystack, result, 0, 25, 25, 50, 0); 
	//parallel_merge(haystack, result, 50, 75, 75, 100, 50); 

	
	//see if i have two sorted arrays:
	printf("\nfinished:\n\n");
	for (int i = 0; i < n*2; i++) {
		printf("%lld\n", result[i]);
	}
	printf("\n");

	bool is_merged = is_sorted(result, 0, 2*n); 
	printf("is_merged == %d\n", is_merged); 

	printf("\nare the two pieces merged? %s \n\n", (is_merged) ? "yes" : "no");
	
*/ 

}

void run_many_parts_par_merge() {

	int n = 8;
	int sets = 8;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack = malloc((sizeof *haystack)*n*sets);
	//unsigned long long *result = malloc((sizeof *result)*n*sets);
	//unsigned long long *result2 = malloc((sizeof *result)*n*sets);
	//unsigned long long *result3 = malloc((sizeof *result)*n*sets);
	
	for (int i = 0; i < n*sets; i++) {
		//haystack[i] = rand(); 
		haystack[i] = rand() % 20; 
	}

	for (int i = 0; i < sets; i++) {
		qsort((haystack+i*n), n, sizeof(*haystack), cmpfunc); 
	}

	//see if i have multiple sorted parts:
	printf("setup array is: \n");
	for (int i = 0; i < n*sets; i++) {
		if (i % n == 0) printf("\n");
		printf("%lld\n", haystack[i]);
	}
	printf("\n");

	//confirm that i have sorted parts: 
	
	for (int i = 0; i < sets; i++) {
		
		//unsigned long long *start_ptr = haystack+(i*n);
		int start_idx = i*n;
		int end_idx = (i+1)*n;
		//bool is_merged = is_sorted(start_ptr, start_idx, end_idx); 
		bool is_merged = is_sorted(haystack, start_idx, end_idx); 
		if (!is_merged)
			printf("sorted run is not sorted %d == %d\n", i, is_merged); 
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
	api_par_merge(haystack, 0, 7, 8, 15, 0); 
	api_par_merge(haystack, 16, 23, 24, 31, 16); 
	api_par_merge(haystack, 32, 39, 40, 47, 32); 
	api_par_merge(haystack, 48, 55, 56, 63, 48); 

	bool is_merged = is_sorted(haystack, 0, 16); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 16, 32); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 32, 48); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(haystack, 48, 64); 
	//printf("sorted run == %d\n", is_merged); 

	//round2:
	// i think i need a new results array to do another round....
/*
	parallel_merge(result, result2, 0, 15, 16, 31, 0); 
	parallel_merge(result, result2, 32, 47, 48, 63, 32); 

	is_merged = is_sorted(result2, 0, 32); 
	//printf("sorted run == %d\n", is_merged); 

	is_merged = is_sorted(result2, 33, 64); 
	//printf("sorted run == %d\n", is_merged); 

	//round3:
	// i think i need a new results array to do another round....

	parallel_merge(result2, result3, 0, 31, 32, 63, 0); 

	is_merged = is_sorted(result3, 0, 64); 
	printf("sorted run == %d\n", is_merged); 
*/
}
