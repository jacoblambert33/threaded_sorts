#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
// #include"threadpool.h"
#include "simple_queue.h"

#define DEBUG 0
#define MAX_THREADS 10
#define MAX_QUEUE 10

#define POOL_SIZE 20

pthread_t thread_pool[POOL_SIZE];

// tpool_t fib_thread_pool;

long fib(long n) {
  if (n <= 1)
    return n;
  else {
    return fib(n - 1) + fib(n - 2);
  }
}

long long_fib(long n) {
  usleep(1);
  if (n <= 1)
    return n;
  else {
    return fib(n - 1) + fib(n - 2);
  }
}

/*
void *queued_fib(void *n) {

        if (n <= 1)
                return n;
        else {
                enqueue(n-1);
                long right = fib(n-2);
                return fib(n-1) + fib(n-2);
        }
}
*/

long zero = 0;
long one = 1;

/*
void *qfib_t(void *in) {

        long n = *((long *)in);
        //long *n = (long *)in;

        //printf("input is %ld\n", n);

        if (n == 0)
                return &zero;
        if (n == 1)
                return &one;

        //pthread_t t;
        long *left = malloc(sizeof(long));
        // *left = 0;
        long *ans = malloc(sizeof(long));
        long ll = n-1;
        long rr = n-2;

        enqueue(&ll);
        enqueue(&rr);

        //printf("calculated left as %ld\n", *left);
        *ans = *left + right;
        //printf("calculated ans as %ld\n", *ans);
        return ans;

}
*/

void *fib_t(void *in) {
  long n = *((long *)in);
  // long *n = (long *)in;

  // printf("input is %ld\n", n);

  if (n == 0) return &zero;
  if (n == 1) return &one;

  long *ans = malloc(sizeof(long));

  // try cutoff to serial function.
  if (n < 44) {
    long cl = fib(n - 1);
    long cr = fib(n - 2);
    *ans = cl + cr;
    return ans;
  }

  pthread_t t;
  long *left = malloc(sizeof(long));
  //*left = 0;
  long ll = n - 1;
  long rr = n - 2;
  pthread_create(&t, NULL, fib_t, &ll);
  // int e =  tpool_add_work(fib_thread_pool, fib_t, &ll);
  long right = *((long *)fib_t(&rr));
  // printf("calculated right as %ld\n", right);
  // pthread_join(t, ((void*)left)); //this is dumb but keep it bc i thought it
  // and realize why dumb.
  pthread_join(t, (void *)&left);
  // printf("calculated left as %ld\n", *left);
  *ans = *left + right;
  // printf("calculated ans as %ld\n", *ans);
  return ans;
}

/* i can't let my threads die. they are doing the work. so i can't return from
these threads, i.e., i can't use join. so - try to use th_e serial fib function
inside the threading function. but then i have no way to return the value to the
calling function. for academic purposes, can i extend the queue to hold the
return value and the thread itself. this is all bad. what about a cutoff? let's
try a cutoff.
*/

/*
void *thread_function(void *arg) {

        while(true) {

                long *n = dequeue();
                int ans = fib(n);
                //pthread_join(
        }

}
*/

int main(int argc, char **argv) {
  if (argc != 2) {
    printf("Usage: %s <int>\n", argv[0]);
    return EXIT_FAILURE;
  }

  /*
  //TMP
  pthread_t t;
  long x = 1;
  pthread_create(&t, NULL, fib_t, &x);
  */

  // HATE this threadpool - too complex for first.
  // tpool_init(&fib_thread_pool, MAX_THREADS, MAX_QUEUE, 0);

  long input = strtol(argv[1], NULL, 10);

  int msec = 0;  //, trigger = 10; /* 10ms */

  /*
  //init threads in our pool:
  for (int i = 0; i < POOL_SIZE; i++) {
          pthread_create(&thread_pool[i], NULL, thread_function, NULL);
  }
  */

  clock_t before = clock();

  // long ans = fib(input);
  long ans = *((long *)fib_t(&input));

  clock_t difference = clock() - before;
  // msec = difference * 1000 / CLOCKS_PER_SEC;
  msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Calculated answer for threaded fib(%ld) is: %ld\n", input, ans);
  printf("Time taken %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, msec / 1000, msec % 1000);

  before = clock();

  // long ans = fib(input);
  ans = long_fib(input);

  difference = clock() - before;
  // msec = difference * 1000 / CLOCKS_PER_SEC;
  msec = difference * 1000000 / CLOCKS_PER_SEC;

  printf("Calculated answer for fib(%ld) is: %ld\n", input, ans);
  printf("Time taken %d seconds %d milliseconds %d microseconds \n",
         msec / 1000000, msec / 1000, msec % 1000);

  return EXIT_SUCCESS;
}
