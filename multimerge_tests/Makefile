# Current method of testing relies on the following steps:
# 1. Copy the compiled object from ../sort/lib where it lives and put it into ./libs
# 2. Archive it: e.g., 
#    cd libs/
#		 ar rcs libbs.a binary_search.o
#		 cd ..
# 3. Now compile here using an -I(ncludes) for the header at ../sort/lib and -L(ibraries) at libs and use the specific file we moved into libs, e.g., libbs.a (the way you reference it is without reference to lib or .a in the name. It MUST be the last command line option to gcc - it has to come after the -o for sure. 
# The goal is to remove code duplication. Test the components where they are. I think the above solution is acceptable. I'm not positive it's the best solution. 
# The current negative to this approach is I have to manually copy the complete file to this folder before I test it. That's not cool or fun, but right now it's better to me than making the Makefile a lot more complex. 

CC=gcc
INC=-I../sort/lib/
LIB=-Llibs/
CFLAGS=-Wall -g $(INC) $(LIB)

all: bin_search test_p_merge test_pms
#all: bin_search 

bin_search: bin_search_driver.c debug.h
	$(CC) $(CFLAGS) $^ -o $@ -lbs

#test_p_merge: test_p_merge.c binary_search.o helpers_sort.o p_merge.o debug.h
test_p_merge: test_p_merge.c helpers_sort.o p_merge.o debug.h
	$(CC) $(CFLAGS) $^ -o $@ -lbs

#test_pms: test_pms.c parallel_merge_sort.o p_merge.o binary_search.o helpers_sort.o debug.h 
test_pms: test_pms.c parallel_merge_sort.o p_merge.o helpers_sort.o debug.h 
	$(CC) $(CFLAGS) $^ -o $@ -lbs

helpers_sort.o: helpers_sort.c helpers_sort.h debug.h
	$(CC) $(CFLAGS) -c $^

p_merge.o: p_merge.c p_merge.h debug.h
	$(CC) $(CFLAGS) -c $^ -lbs

parallel_merge_sort.o: parallel_merge_sort.c parallel_merge_sort.h debug.h
	$(CC) $(CFLAGS) -c $^

clean:
#	rm -rf bin_search binary_search.o test_p_merge p_merge.o test_pms parallel_merge_sort.o	
	rm -rf bin_search test_p_merge test_pms *.o
