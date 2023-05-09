import random
import numpy as np

# charAt - say why?
def charAt(s, d):
	if d < len(s) :
		return ord(s[d])
	else:
		return -1

# exch
def exch(a, i, j):
	print(f'exch {a[i]} and {a[j]} at positions {i} and {j}.')
	t = a[i]
	a[i] = a[j]
	a[j] = t



"""
3 way string quicksort - modification of MSD radix sort.
"""

# API
def api_sort_str(a) :
	#print(f'length of a is: {len(a)}')
	sort_str(a, 0, len(a)-1, 0)

# 3way partitioning using dth char
def sort_str(a, lo, hi, d) :
	print(f'START round {d}. lo is: {lo}, hi is: {hi}, and d is {d}')
	if (hi <= lo):
		return
	lt = lo
	gt = hi
	v = charAt(a[lo], d)
	print(f'in sort, v of {a[lo]} for d {d} is: {v}')
	i = lo + 1
	while i <= gt:
		print(f'i is {i}')
		t = charAt(a[i], d)
		if t < v:
			print(f't is {t}, v is: {v}')
			#exch(a, lt+=1, i+=1)
			exch(a, lt, i)
			lt+=1
			i+=1
		elif t > v:
			print(f't is {t}, v is: {v}')
			#exch(a, i, gt-=1)
			exch(a, i, gt)
			gt-=1
		else:
			print(f't is {t}, v is: {v}')
			i+=1

	print(f'MIDDLE round {a}')
	# now have three subarrays to sort recursively
	sort_str(a, lo, lt-1, d)
	if v >= 0:
		sort_str(a, lt, gt, d+1)
	sort_str(a, gt+1, hi, d)

	




def test_sort() :

	a = [ "Kellen", "Jacob", "Kaden", "Simeon", "Jack", "Daniel", "Wyatt", "Owen", "Ben", "Josh", "Cameron", "Logan" ]

	print(a)

	api_sort_str(a)
 
	print(a)

	print(f'\tEND test_sort')




def main():

	test_sort()



if __name__ == "__main__":
    main()


