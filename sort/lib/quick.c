#include"quick.h"
#include<assert.h>
#include"helpers_sort.h"


int partition(unsigned long long a[], int lo, int hi) {

	int i = lo;
	//int j = hi+1;
	int j = hi;

	unsigned long long pivot = a[i];

	//forever loop - must break it. 
	while(1) {
		
		while (less(a[++i], pivot))
			if (i == hi)
				break;

		while (less(pivot, a[--j]))
			if (j == lo)
				break;

		//have pointers crossed
		if (i >= j)
			break;

		exch(a, i, j);
	}		

	//put pivot at a[j]
	exch(a, lo, j); 

	// truth is a[lo ... j-1] <= a[j] <=  a[j+1 ....hi] 
	return j; 

}


void quick(unsigned long long a[], int lo, int hi) {

	//hi = hi -1;
	if (hi-1 <= lo)
		return;

	int j = partition(a, lo, hi);
	//quick(a, lo, j-1); 
	quick(a, lo, j); 
	quick(a, j+1, hi); 	
	
	assert(is_sorted(a, lo, hi));
}
