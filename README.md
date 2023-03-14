# multithreadedMerge

This project started with one goal in mind: implement multithreaded mergesort from CLRS chapter 27. See: http://cs403.cs.ua.edu/fall2018/CLRS%20Introduction%20to%20Algorithms%20chapter%2027.pdf

I chose Pthreads to do divide and conquer as I'd made an attempt some time ago to do this with OpenMP. I didn't see great results then (although that could be due to bad implementation of Tasks), but I wanted to have more control over the threads. 

As I tried to build this up from its component pieces, I realized a few other things that became goals as well. I'd like to see how well this implementation compares to other common sorting techniques directly - each of them written in C with or without Pthreads. I had to develop a sense for the overhead of creating threads and try to determine roughly where the dividing line might be for the minimum amount of work that might be worth creating a thread. 

The project is not complete; it needs love in many places (TODO: at least, bounds checking for memory issues in all of my implementations (valgrind), an open bug with parallel merge sort that looks to result from my binary search impl, etc. Plus, there's just general cleanup, organization, comments, and documentation.)

Incidentally, the project does an acceptable job of generating random 64 bit integers, and a decent job measuring thread overheads in the fibonacci toy example (NOTE: great problem to implement threads divide and conquer threads but there are many superior ways if your goal is to generate the numbers - DP or memoization)

How to run (for now):

```
cd sort
# options for D flag are: QSORT, MEQ, INSERT, SMS, BPMS, PMS
make clean
make ST=INSERT /* or any of the options above */
./sort <# of items>  /* e.g., ./sort 10000 */
```
Expected output (similar):
```
Reading 10000 items from your input file.
attempting to read list of integers.
how big is unsigned long long? 8
Did I sort the array? true
Time taken 0 seconds 242 milliseconds 177 microseconds 


```
