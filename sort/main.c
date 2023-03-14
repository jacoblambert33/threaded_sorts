// #define  _POSIX_C_SOURCE 200809L

#include<stdlib.h>
#include<stdio.h>
#include<stdint.h>
#include<string.h>
#include<time.h>
#include"insertion.h"
#include"serial_merge.h"
#include"basic_par_merge.h"
#include"parallel_merge_sort.h"
#include"quick.h"
#include"helpers_sort.h"
#include"ull_cmp.h"

#define DEBUG 1

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

           //size_t ret = fread(buffer, ARRAY_SIZE(buffer), sizeof(*buffer), fp);

void write_data_to_file(char *filename, unsigned long long *x, long items_to_read ); 


int main(int argc, char **argv) {

	if (argc != 2) {
		printf("Tell the program how many lines to read from your input file. i.e., ./sort 8\n"); 
		return 1;
	}

	long items_to_read;

	items_to_read = strtol(argv[1], NULL, 10);
	printf("Reading %ld items from your input file.\n", items_to_read);	

	//size_t items_to_read = 8;
	//size_t items_to_read = 10000;
  FILE *myfile;
  int j = 0;
  unsigned long long x[items_to_read];
  //uint64_t x[8];
    char * line = NULL;
    char * endptr = NULL;
    size_t len = 0;
	size_t read;
	size_t base = 10;
	unsigned long long val;

	int msec = 0;//, trigger = 10; /* 10ms */


  //int numread;

  if ( (myfile = fopen("doc/rand64_list.txt", "r")) == NULL) {
    printf("sorry, I can't open your file\n");
		return(1);
  }

  printf("attempting to read list of integers.\n");

  printf("how big is unsigned long long? %ld\n", sizeof(unsigned long long));

  //numread = fread(x, sizeof(unsigned long long), 8, myfile);
  //numread = fread(x, ARRAY_SIZE(x), sizeof(*x), myfile);


    while ((read = getline(&line, &len, myfile)) != -1 && j < items_to_read ) {
        //printf("Retrieved line of length %zu:\n", read);
        //printf("%s", line);
				val = strtoull(line, &endptr, base);
				//printf("%20llu\n", val);
				x[j++] = val;

    }

	fclose(myfile);

	clock_t before = clock();

	char *output_fn;
#if defined(default)
	perror("Error. Enter a sort method. So far we have QSORT, INSERT, SMS, BPMS");
#elif defined(QSORT)
	qsort(x, items_to_read, sizeof(unsigned long long), cmpfunc);
	output_fn = "QSORT"; 
#elif defined(MEQ)
	quick(x, 0, items_to_read); 
	output_fn = "MEQ"; 
#elif defined(INSERT)
	insertion_sort(x, 0, items_to_read); 
	output_fn = "INSERT"; 
#elif defined(SMS)
	smergesort(x, 0, items_to_read); 
	output_fn = "SMS"; 
#elif defined(BPMS)
	basic_p_ms(x, 0, items_to_read); 
	output_fn = "BPMS"; 
#elif defined(PMS)
	pms(x, 0, items_to_read); 
	output_fn = "PMS"; 
#endif




	
	clock_t difference = clock() - before;
  //msec = difference * 1000 / CLOCKS_PER_SEC;
  msec = difference * 1000000 / CLOCKS_PER_SEC;


	if(DEBUG) {
		write_data_to_file(output_fn, x, items_to_read );
	}

	printf("Did I sort the array? %s\n", is_sorted(x, 0, items_to_read) ? "true" : "false");
	printf("Time taken %d seconds %d milliseconds %d microseconds \n",
  msec/1000000, msec/1000, msec%1000 );

/*
  printf("numread is: %d.\n", numread);

  for(j = 0; j < numread; j++) {
    //printf("%02lld\n", x[j]);
    printf("%20llu\n", x[j]);
    //fprintf(mine, "%20llu\n", x[j]);

  }
  printf("\n");
*/  

  exit(0);
 
}

void read_data_from_file() {

}

void write_data_to_file(char *filename, unsigned long long *x, long items_to_read ) {

	FILE *output;	
	//char fn[100];
	char basefn[100] = "results/sorted_by_";
	strcat(basefn, filename); 
  if ( (output = fopen(basefn, "w")) == NULL) {
    printf("sorry, I can't open your file\n");
  }


		for(int j = 0; j < items_to_read; j++) {
			//printf("%20llu\n", x[j]);
			fprintf(output, "%20llu\n", x[j]);
		}

	fclose(output);

}

