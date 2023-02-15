#include"p_merge.h"
#include"binary_search.h"
#include<assert.h>
#include<stdbool.h>
#include<stdio.h>
#include<stdlib.h>
#include<pthread.h>

//private declaration so impl is meant to go in c file. 
struct pmerge_params {
    unsigned long long *input;
		unsigned long long *result; 
		int beg_left; 
		int end_left;
		int beg_right; 
		int end_right;
		int beg_reslt;
};

int count = 0; 

bool is_sorted(unsigned long long a[], int lo, int hi); 

// public API 
void api_par_merge(unsigned long long haystack[], int p1, int r1, int p2, int r2, int p3) {
		int total_len = (r1- p1) + 1 + (r2 - p2)+1; 
		//unsigned long long *result = malloc(total_len * sizeof(*result)); 
		unsigned long long *result = malloc(total_len * sizeof(unsigned long long)); 
		if (!result) {
			perror("Unable to allocate array");
			return;
		}

		struct pmerge_params p = { .input = haystack, .result = result, .beg_left = p1, .end_left = r1, .beg_right = p2, .end_right = r2, .beg_reslt = p3};

		par_merge((void*)&p); 

		for(int i = p3; i < total_len; i++) {
			haystack[i] = result[i];
		}

		free(result);

}

void *par_merge(void *params) {

	struct pmerge_params left = *((struct pmerge_params*) params);
	struct pmerge_params right = left; 


	//precondition - halves are sorted:
	assert(is_sorted(left.input, left.beg_left, left.end_left)); 
	assert(is_sorted(left.input, left.beg_right, left.beg_right));


	int len_left = left.end_left - left.beg_left + 1;
	int len_right = left.end_right - left.beg_right + 1;


	//treat the longer portion (if one is longer) as the first half:
	if (len_left < len_right) {
		//exchange values, explicitly here:
		int tmp = left.beg_right;
		left.beg_right = left.beg_left; 
		left.beg_left = tmp; 
		tmp = left.end_right;
		left.end_right = left.end_left;
		left.end_left = tmp;
		tmp = len_right; 
		len_right = len_left;
		len_left = tmp;
	}
	//assert precondition - half one is >= half two
	//if both halves are empty then return
	if (len_left == 0)
		return NULL;

	//do parallel merge:

	int mid_left = (left.beg_left + left.end_left) / 2;	
	int mid_in_right = binary_search(left.input[mid_left], left.input, left.beg_right, left.end_right); 
	//calculate the index - where does the midpoint element go in the result.
	int mid_in_result = left.beg_reslt + (mid_left - left.beg_left) + (mid_in_right - left.beg_right); 
	//put mid element into place in the result. 
	left.result[mid_in_result] = left.input[mid_left]; 


	//set up the structures:
	right.beg_left = mid_left+1;
	right.end_left = left.end_left; // unchanged 
	right.beg_right = mid_in_right; 
	right.end_right = left.end_right; //unchanged
	right.beg_reslt = mid_in_result+1;

	left.beg_left = left.beg_left; // unchanged
	left.end_left = mid_left-1; 
	left.beg_right = left.beg_right; //unchanged;
	left.end_right = mid_in_right-1;
	left.beg_reslt = left.beg_reslt; // unchanged; 


	//this becomes parallel. 
	
	par_merge(&left); 
	par_merge(&right); 

	return NULL;
}


void parallel_merge(unsigned long long input_arr[], unsigned long long result_arr[], int start_half1_indx, int end_half1_indx, int start_half2_indx, int end_half2_indx, int start_result_indx) {


	//precondition - halves are sorted:
	is_sorted(input_arr, start_half1_indx, end_half1_indx); 
	is_sorted(input_arr, start_half2_indx, end_half2_indx);


	int len_half1 = end_half1_indx - start_half1_indx + 1;
	int len_half2 = end_half2_indx - start_half2_indx + 1;

	//treat the longer portion (if one is longer) as the first half:
	if (len_half1 < len_half2) {
		//exchange values, explicitly here:
		int tmp = start_half2_indx; 
		start_half2_indx = start_half1_indx;
		start_half1_indx = tmp;
		tmp = end_half2_indx;
		end_half2_indx = end_half1_indx;
		end_half1_indx = tmp; 
		tmp = len_half2;
		len_half2 = len_half1;
		len_half1 = tmp;	
	}
	//assert precondition - half one is >= half two
	//if both halves are empty then return
	if (len_half1 == 0)
		return;

	//do parallel merge:
	printf("current count non-length 0 portions: %d\n", ++count); 

	int mid_half1 = (start_half1_indx + end_half1_indx) / 2;	
	int mid_indx_in_half2 = binary_search(input_arr[mid_half1], input_arr, start_half2_indx, end_half2_indx); 
	//calculate the index - where does the midpoint element go in the result.
	int indx_mid_in_result = start_result_indx + (mid_half1 - start_half1_indx) + (mid_indx_in_half2 - start_half2_indx); 
	//put mid element into place in the result. 
	result_arr[indx_mid_in_result] = input_arr[mid_half1]; 

	//this becomes parallel. 
	parallel_merge(input_arr, result_arr, start_half1_indx, mid_half1-1, start_half2_indx, mid_indx_in_half2-1, start_result_indx); 

	parallel_merge(input_arr, result_arr, mid_half1+1, end_half1_indx, mid_indx_in_half2, end_half2_indx, indx_mid_in_result+1); 
	

	

}
// is v < w ?
//bool less(uint64_t v, uint64_t w) {
bool less(unsigned long long v, unsigned long long w) {
		return (v < w) ? true : false;
}

	//binary_search(unsigned long long needle, unsigned long long haystack[], int lo_indx, int hi_indx); 

// is the array a[lo..hi) sorted
//bool is_sorted(uint64_t a[], int lo, int hi) {
bool is_sorted(unsigned long long a[], int lo, int hi) {
		for (int i = lo + 1; i < hi; i++)
				//if (less(a[i], a[i-1])) return false;
				if (less(a[i], a[i-1])) {
					printf("found %lld before %lld\n", a[i], a[i-1]);	
					return false;
				}
		return true;
}
