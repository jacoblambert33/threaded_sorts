# concrete sorting algorithms

This project started with one goal in mind: implement multithreaded mergesort from CLRS chapter 27. See: http://cs403.cs.ua.edu/fall2018/CLRS%20Introduction%20to%20Algorithms%20chapter%2027.pdf. (in the 4th edition - it's chapter 26!)

However, the project evolved to become a study of sorting with Pthreads. It now has some lasting value in this regard. 

I chose Pthreads to do divide and conquer as I'd made an attempt some time ago to do multimerge with OpenMP. I didn't see great results then (that could be my bad implementation of Tasks and is something I hope to look at in the future), but I wanted to take more control over the threads anyway. 

As I tried to build up multithread merge sort from its component pieces, I realized a few other things that became goals as well. One, how well does this implementation compare to other common sorting techniques directly, where each is written in C with and without Pthreads. I had to develop a sense for the overhead of creating threads and try to determine roughly where the dividing line might be for the minimum amount of work that might be worth creating a thread. 

And again the project would evolve. I realized a need for many other types of sorts, to compare different methods against one another directly with a certain baseline. For complicated algorithms, I felt correctness was sometimes best first established in Python, and so Python examples exist. Similarly, the promises of Rust as a systems programming language mean direct comparions are also valuable. 

The project is not complete; it needs love in many places (TODO: at least, confirm bounds checking is complete for memory issues in all my C implementations (valgrind), removal of some of my half-completed or abandoned attempts, additional Python and especially Rust implementations. Plus, there's just general cleanup, organization, comments, and documentation.)

One limitation I expect not to address in the near term is that I only sort large positive integers. It sometimes seems there are countless possible variables to consider in sorting implementations, and focusing only on positive integers felt like an acceptable start - even possibly an acceptable final destination. 

Incidentally, the project does an acceptable job of generating random 64 bit integers, and a decent job measuring thread overheads in the fibonacci toy example (NOTE: great problem to implement threads divide and conquer threads but there are many superior ways if your goal is to generate the numbers - e.g., DP or memoization)

There are currently five groups within this project. 
* Fibonacci (./fib), a direct comparison of the standard implementation versus one in Pthreads. 
* Random number generation (./mersenne), create a file of 64-bit integers to use elsewhere as sorting inputs. 
* Component tests (./tests), a place to stage components and test them individually before moving them to the ./sort folder. Python implementations are here for now as well. 
* Rust implementations (./rust_impls), an initial effort at comparing serial and threaded versions of the C sorts against a modern systems language.  
* Sort (./sort), a collection of common sorting methods implemented in C and Pthreads. 

How to run Sort (for now):

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
