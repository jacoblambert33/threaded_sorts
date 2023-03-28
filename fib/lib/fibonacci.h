#ifndef FIBONACCI_H
#define FIBONACCI_H

extern int cutoff;
extern long total_threads;
extern unsigned int sleep_micros;
extern int sleep_freq;
extern int sleep_cnt;

long fib(long n);

long sleep_fib(long n);

void *fib_t(void *in);

#endif  // FIBONACCI_H
