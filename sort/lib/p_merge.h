#ifndef PARALLLEL_MERGE_H
#define PARALLLEL_MERGE_H

// private declaration; struct meant for use only in the corresponding c file.
struct pmerge_params;

// API. a serial version of the merge procedure
void parallel_merge(unsigned long long haystack[], int p1, int r1, int p2,
                    int r2, unsigned long long result[], int p3);

// API. a threaded parallel merge.
void parallel_merge_t(unsigned long long haystack[], int p1, int r1, int p2,
                      int r2, unsigned long long result[], int p3);

#endif  // PARALLLEL_MERGE_H
