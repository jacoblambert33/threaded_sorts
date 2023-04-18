#ifndef PARALLEL_MERGE_SORT_H
#define PARALLEL_MERGE_SORT_H

// serial version using threaded merge.
void pms(unsigned long long a[], int lo, int hi);

// threaded version with threaded merge.
void pms_t(unsigned long long a[], int lo, int hi);

// TMP - for direct testing.
void book_p_merge_sort(unsigned long long A[], int p, int r,
                       unsigned long long B[], int s);

#endif  // PARALLEL_MERGE_SORT_H
