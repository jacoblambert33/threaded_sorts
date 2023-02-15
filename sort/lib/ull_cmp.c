#include"ull_cmp.h"

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



