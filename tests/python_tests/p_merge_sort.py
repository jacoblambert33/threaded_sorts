import random
import numpy as np

"""
FIND-SPLIT-POINT(A; p; r; x)
1 low = p // low end of search range 
2 high = r + 1 // high end of search range 
3 while low < high // more than one element?
4 mid = (low + high)2 // midpoint of range 
5 if x <= A[mid] // is answer q <= mid?
6 high = mid // narrow search to A[low : mid]
7 else low = mid + 1 // narrow search to A[mid + 1 : high]
8 return low
"""
def find_split_point(a, p, r, x): 

	low = p; 
	high = r + 1; 
	while low < high: 

		mid = (low + high) // 2; 
		if x <= a[mid]:
			high = mid;
		else:
			low = mid + 1;
	
	return low; 


def test_sp(arr):
	#for x in range(len(arr)):
	for x in arr:
		sp = find_split_point(arr, 0, 16, x)
		print(f'{x} is at {sp}')

	print(f'\tEND test_sp')


"""
P-MERGE-AUX(A; p1; r1; p2; r2; B; p3)
1 if p1 > r1 and p2 > r2 // are both subarrays empty?
2 return
3 if r1 - p1 < r2 - p2 // second subarray bigger?
4 exchange p1 with p2 // swap subarray roles 
5 exchange r1 with r2
6 q1 = (p1 + r1)/2 // midpoint of A[p1 : r1]
7 x = A[q1]		// median of A[p1 : r1] is pivot x
8 q2 = FIND-SPLIT-POINT(A; p2; r2; x) // split A[p2 : r2] around x
9 q3 = p3 + (q1 - p1) + (q2 - p2)  // where x belongs in B...
10 B[q3] = x // ... put it there 
11 // Recursively merge A[p1 : q1 - 1] and A[p2 : q2 - 1] into B[p3 : q3 - 1]. 
12 spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
13 // Recursively merge A[q1 + 1 : r1] and A[q2 : r2] into B[q3 + 1 : r3]. 
14 spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
15 sync // wait for spawns
"""
def p_merge_aux(a, p1, r1, p2, r2, b, p3): 

	if ((p1 > r1) and (p2 > r2)) :
		return

	if ((r1 - p1) < (r2 - p2)) :
		#exch(a, p1, p2);
		#exch(a, r1, r2);  
		t1 = p1 
		p1 = p2
		p2 = t1
		t2 = r1
		r1 = r2 
		r2 = t2 
	

	q1 = (p1 + r1) // 2 

	x = a[q1] 

	q2 = find_split_point(a, p2, r2, x)
	
	q3 = p3 + (q1 - p1) + (q2 - p2)

	b[q3] = x
 
	p_merge_aux(a, p1, q1-1, p2, q2-1, b, p3)
	p_merge_aux(a, q1+1, r1, q2, r2, b, q3+1)



def test_pmergeaux(arr) :

	a2 = [ 5, 15, 7, 9, 13, 13, 13, 19, 14, 8, 5, 11, 2, 9, 9, 4]
	a2 = np.sort(a2)

	combined = np.concatenate((arr, a2), axis=None)
	print(combined)

	b = np.zeros(32) 
	p_merge_aux(combined, 0, 15, 16, 31, b, 0)
	
	print(b)

	print(f'\tEND test_pmergeaux')

"""
P-MERGE(A; p; q; r)
1 let B[p : r] be a new array // allocate scratch array 
2 P-MERGE-AUX (A; p; q; q + 1; r; B; p) // merge from A into B
3 parallel for i = p to r // copy B back to A in parallel 
4 A[i] = B[i]
"""
def p_merge(a, p, q, r) : 

	length = r-p+1
	print(f'p_merge: length is {length}')

	#aux = np.zeros(length+10)
	#aux = np.zeros(len(a))
	aux = np.zeros(r+1)

	p_merge_aux(a, p, q, q+1, r, aux, p)
	
	#print(f'p_merge: aux is {aux}')

	# works but overkill?
	#for i in range(length) :
	for i in range(p, r+1) :
		a[i] = aux[i]  

	#print(f'p_merge: a is {a}')

	return a
	


def test_pmerge(a) :

	a2 = [ 5, 15, 7, 9, 13, 13, 13, 19, 14, 8, 5, 11, 2, 9, 9, 4]
	a2 = np.sort(a2)

	combined = np.concatenate((a, a2), axis=None)
	
	l = len(combined)
	#print(f'length is {l}')
	q = (l-1) // 2
	#print(f'q is {q}')
	#print(f'combined is {combined}')
	n = p_merge(combined, 0, q, l-1)
 
	print(n)

	print(f'\tEND test_pmerge')



def p_merge_me(a, b, p, q, r) : 

	p_merge_aux(a, p, q, q+1, r, b, p)
	
	#print(f'p_merge: aux is {aux}')

	for i in range(p, r+1) :
		a[i] = b[i]  

	#print(f'p_merge: a is {a}')

	return a
	
"""
P-MERGE-SORT.A; p; r/
1 if p >= r // zero or one element?
2 return
3 q = (p + r) / 2 // midpoint of A[p : r]
4 // Recursively sort A[p : q] in parallel. 
5 spawn P-MERGE-SORT(A; p; q)
6 // Recursively sort A[q + 1 : r] in parallel. 
7 spawn P-MERGE-SORT(A; q + 1; r)
8 sync // wait for spawns 
9 // Merge A[p : q] and A[q + 1 : r] into A[p : r]. 
10 P-MERGE(A; p; q; r)

"""
#public API
def p_merge_sort(a,	p, r) :

	# if the size of the array is 0 or 1 elements: 
	if (p >= r) :
		return
 
	# get midpoint index. 
	q = (p + r) // 2

	p_merge_sort(a, p, q)
	p_merge_sort(a, q+1, r)
	# debug:
	#print(f'input to p_merge(<arr> (len is {len(a)}), {p}, {q}, {r})')
	print(f'input to p_merge({a}, {p}, {q}, {r})')
	p_merge(a, p, q, r)


def test_pmergesort() :

	#arr_num = np.random.randint(20, size=16)
	arr_num = [19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4]
	print(f'test_pmergesort: input is: {arr_num}')
	
	r = len(arr_num)-1
	print(f'test_pmergesort: r is: {r}')
	#arr_num = p_merge_sort(arr_num, 0, r)
	p_merge_sort(arr_num, 0, r)
	print(f'test_pmergesort: answer is: {arr_num}')
 
	print(f'\tEND test_pmergesort')



def p_merge_sort_me(a, b, p, r) :

	# if the size of the array is 0 or 1 elements: 
	if (p >= r) :
		return
 
	# get midpoint index. 
	q = (p + r) // 2

	p_merge_sort_me(a, b, p, q)
	p_merge_sort_me(a, b, q+1, r)
	# debug:
	#print(f'input to p_merge(<arr> (len is {len(a)}), {p}, {q}, {r})')
	#print(f'input to p_merge({a}, {p}, {q}, {r})')
	p_merge_me(a, b, p, q, r)


def my_api_pms(a, p, r) :

	length = r-p+1
	print(f'p_merge: length is {length}')

	#aux = np.zeros(length+10)
	#aux = np.zeros(len(a))
	aux = np.zeros(r+1)

	#n = p_merge_sort_me(a, aux, p, r)
	p_merge_sort_me(a, aux, p, r)

	#print(a)



def main():
	#arr_num = np.random.randint(20, size=16)
	arr_num = [17, 19,  7,  2,  8,  5, 13,  9, 11, 19, 18, 11,  6,  0, 12,  6]

	print(arr_num)
	arr_num = np.sort(arr_num)
	print(arr_num)


	#sp = find_split_point(arr_num, 0, 16, 7)
	#print(sp)
	test_sp(arr_num);

	test_pmergeaux(arr_num)

	test_pmerge(arr_num)

	#print(f'answer is:\n{a}')

	test_pmergesort()


	arr_num = np.random.randint(10000, size=100000)
	A = np.sort(arr_num)	
	my_api_pms(arr_num, 0, len(arr_num)-1)
	B = arr_num
	
	is_sorted = (A==B).all()
	print(f'did you sort the array? {is_sorted}')

	


if __name__ == "__main__":
    main()


