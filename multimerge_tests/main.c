#include"binary_search.h"
#include<stdio.h>
#include<stdlib.h>

int cmpfunc (const void * a, const void * b); 


int main(int argc, char **argv) {

	int n = 50;
	//unsigned long long needle = 1025202362;

	unsigned long long *haystack = malloc((sizeof *haystack)*n);
	
	for (int i = 0; i < n; i++) {

		haystack[i] = rand(); 
	}

	qsort(haystack, n, sizeof(*haystack), cmpfunc); 


	for (int i = 0; i < n; i++) {
	

		unsigned long long needle = haystack[i];
		int index = binary_search(needle, haystack, 0, n-1);

		printf("needle %lld is at index %d of haystack.\n", needle, index);

	}
	
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


