# multithreadedMerge

This project started with one goal in mind: implement multithreaded mergesort from CLRS chapter 27. See: http://cs403.cs.ua.edu/fall2018/CLRS%20Introduction%20to%20Algorithms%20chapter%2027.pdf

However, the project evolved to become a study of sorting with Pthreads. It now has some lasting value in this regard. 

I chose Pthreads to do divide and conquer as I'd made an attempt some time ago to do multimerge with OpenMP. I didn't see great results then (that could be my bad implementation of Tasks and is something I will look at in the future), but I wanted to take more control over the threads anyway. 

As I tried to build up multithread merge sort from its component pieces, I realized a few other things that became goals as well. One, how well does this implementation compare to other common sorting techniques directly, where each is written in C with and without Pthreads. I had to develop a sense for the overhead of creating threads and try to determine roughly where the dividing line might be for the minimum amount of work that might be worth creating a thread. 

The project is not complete; it needs love in many places (TODO: at least, bounds checking for memory issues in all of my implementations (valgrind), an open bug with parallel merge sort that looks to result from my binary search impl, etc. Plus, there's just general cleanup, organization, comments, and documentation.)

Incidentally, the project does an acceptable job of generating random 64 bit integers, and a decent job measuring thread overheads in the fibonacci toy example (NOTE: great problem to implement threads divide and conquer threads but there are many superior ways if your goal is to generate the numbers - e.g., DP or memoization)

There are currently four groups within this project. 
* Fibonacci (./fib), a direct comparison of the standard implementation versus one in Pthreads. 
* Random number generation (./mersenne), create a file of 64-bit integers to use elsewhere as sorting inputs. 
* Component tests (./multimerge_tests), a place to stage components and test them individually before moving them to the ./sort folder. 
* Sort (./sort), a collection of common sorting methods implemented in C and Pthreads. 

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
