import random
import numpy as np

CUTOFF = 4096 #128 #15

# charAt - say why?
def charAt(b, d):
	s = b.tobytes() #('C') (unnecessary; already default)
	s = s[::-1]
	#print(f'integer in bytes is: {s.hex()}. size is {len(s)}')
	if d < len(s) :
		return s[d]
	else:
		print('yikes!')
		return -1

# exch
def exch(a, i, j):
	#print(f'exch {a[i]} and {a[j]} at positions {i} and {j}.')
	t = a[i]
	a[i] = a[j]
	a[j] = t



"""
3 way string quicksort - modification of MSD radix sort.
"""

# API
def api_sort_int64(a) :
	#print(f'length of a is: {len(a)}')
	sort_int64(a, 0, len(a)-1, 0)

# 3way partitioning using dth char
def sort_int64(a, lo, hi, d) :
	#print(f'START round {d}. lo is: {lo}, hi is: {hi}, and d is {d}')
	#if (hi <= lo):
		#return
	if hi <= lo + CUTOFF :
		a = a.sort()
		return

	lt = lo
	gt = hi
	v = charAt(a[lo], d)
	#print(f'\tv of {a[lo]} for d {d} is: {v}')
	i = lo + 1
	while i <= gt:
		#print(f'i is {i}')
		t = charAt(a[i], d)
		if t < v:
			#print(f't is {t}, v is: {v}')
			exch(a, lt, i)
			lt+=1
			i+=1
		elif t > v:
			#print(f't is {t}, v is: {v}')
			exch(a, i, gt)
			gt-=1
		else:
			#print(f't is {t}, v is: {v}')
			i+=1

	#print(f'MIDDLE round {a}')
	# now have three subarrays to sort recursively
	sort_int64(a, lo, lt-1, d)
	if v >= 0:
		sort_int64(a, lt, gt, d+1)
	sort_int64(a, gt+1, hi, d)

	




def test_sort() :

	sz=120000
	p =63

	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**p, size=sz)

	#print(a)

	api_sort_int64(a)
 
	#print(a)

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(a)}')

	print(f'\tEND test_sort')

def test_baseline() :

	sz=120000
	p = 63
	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**p, size=sz)

	#for i in a:	
		#print(i.tobytes()[::-1].hex())
		 
	aux = np.sort(a) 

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(aux)}')




def main():

	test_sort()
	test_baseline()



if __name__ == "__main__":
    main()


