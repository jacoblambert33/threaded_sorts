#ifndef P_MERGE_NEW_H
#define P_MERGE_NEW_H

// private declaration; struct meant for use only in the corresponding c file.
struct pmerge_params;

// API. a serial version of the merge procedure
void p_merge_new(unsigned long long a[], unsigned long long b[], int p, int q,
                 int r);

/* i don't think i need a threaded version exposed.
instead the threading will be under the covers instead of p_merge.
EXCEPT - for comparison reasons. so i can test each. so i will expose both, at
least for now.
*/
// API. a threaded parallel merge.
void p_merge_new_t(unsigned long long a[], unsigned long long b[], int p, int q,
                   int r, int cutoff);

#endif  // P_MERGE_NEW_H
