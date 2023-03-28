#include "fibonacci.h"

#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define DEBUG 0

signed int cutoff = 25;

long total_threads = 0;

// usleep() takes microseconds, so you will have to multiply the input by 1000
// in order to sleep in milliseconds.
unsigned int sleep_micros = 0;
int sleep_freq = 10000;
int sleep_cnt = 0;

// standard recursive fibonacci. the baseline case.
long fib(long n) {
  if (n <= 1)
    return n;
  else {
    return fib(n - 1) + fib(n - 2);
  }
}

// EXPERIMENT: effects of a delay.
long sleep_fib(long n) {
  if (n <= 1)
    return n;
  else {
    if (++sleep_cnt % sleep_freq == 0) usleep(sleep_micros);
    return sleep_fib(n - 1) + sleep_fib(n - 2);
  }
}

long zero = 0;
long one = 1;

// threaded fibonacci (pthreads) with a cutoff to serial to control threading.
void *fib_t(void *in) {
  long n = *((long *)in);

  if (DEBUG) printf("input is %ld\n", n);

  if (n == 0) return &zero;
  if (n == 1) return &one;

  long *ans = malloc(sizeof(long));

  // try cutoff to serial function.
  if (n < cutoff) {
    long cl = fib(n - 1);
    long cr = fib(n - 2);
    *ans = cl + cr;
    return ans;
  }

  pthread_t t;
  long *left = malloc(sizeof(long));
  long ll = n - 1;
  long rr = n - 2;
  pthread_create(&t, NULL, fib_t, &ll);
  total_threads++;
  long right = *((long *)fib_t(&rr));
  if (DEBUG) printf("calculated right as %ld\n", right);
  pthread_join(t, (void *)&left);
  if (DEBUG) printf("calculated left as %ld\n", *left);
  *ans = *left + right;
  if (DEBUG) printf("calculated ans as %ld\n", *ans);
  return ans;
}
