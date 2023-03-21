#ifndef PARALLLEL_MERGE_H
#define PARALLLEL_MERGE_H

struct pmerge_params;

void *par_merge(void *params);

void book_api_par_merge(unsigned long long haystack[], int p1, int r1, int p2,
                        int r2, unsigned long long result[], int p3);

void api_par_merge_output(unsigned long long haystack[],
                          unsigned long long output[], int h_len, int p1,
                          int r1, int p2, int r2, int p3);

void api_par_merge(unsigned long long haystack[], int h_len, int p1, int r1,
                   int p2, int r2, int p3);

void parallel_merge(unsigned long long input_arr[],
                    unsigned long long result_arr[], int start_half1_indx,
                    int end_half1_indx, int start_half2_indx,
                    int end_half2_indx, int start_result_indx);

#endif  // PARALLLEL_MERGE_H
