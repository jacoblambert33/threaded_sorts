#ifndef SERIALMERGE_H
#define SERIALMERGE_H

// #include<stdint.h> // i don't think i need this if i use unsigned long long instead of uint64_t. 
// #include"helpers_sort.h"
//NOTE: the header file doesn't use helpers. the impl does. however, helpers is always required whenever this header is used. the include might not belong here, however, but in the impl file. 

//void insertion_sort(uint64_t a[], int lo, int hi); 
void smergesort(unsigned long long a[], int lo, int hi); 

#endif //SERIALMERGE_H 
