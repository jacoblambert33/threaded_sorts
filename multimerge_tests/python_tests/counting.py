import random
import numpy as np

BITS_PER_BYTE =   8
BITS_PER_INT  =  64   #each unsinged long long is 64 bits
R             = 256   #extended ASCII alphabet size
CUTOFF        =  15 

def counting_sort(a, R, d):
	
	#given radix R = could be the 26 letters of the alphabet or the 10 numeric digits or what else?
	#define count arr
	count = np.zeros(R+1, dtype=np.uint64)
	
	#define aux arr
	aux = np.zeros(len(a), dtype=np.uint64)


	#count frequency
	for i, v in enumerate(a):
		# v is a[i]. 
		# b is the byte array for the int64 passed in. 
		# b[d] is the element of the byte array. if d = 0, b[d] is the first element of the byte array. 
		# in this modification, we're counting the occurrences of a byte in a particular byte (d) of all the elements.
		#b = np.flip(v.tobytes())
		#b = v.tobytes()[::8]
		b = v.tobytes()
		b = b[::-1]
		#b = np.flip(b)
		#l = len(b)
		#print(f'l is {l}')
		#b = np.ascontiguousarray(v, dtype=">i4")
		print(f'b is {b}')
		print(f'{i}\t{v}')
		#count[a[i]] += 1
		count[b[d]+1] += 1
		#count[b[l-d-1]+1] += 1

	print(f'after frequency, count is {count}')

	#find positions
	for i in range(R):
		count[i+1] += count[i]

	print(f'after tallying, count is {count}')

	# computing cumulates i.e., start of each position
	for i, v in enumerate(a):
		b = v.tobytes()
		#b = v.tobytes()[::8]
		#b = np.flip(b)
		b = b[::-1]
		#b = v.tobytes('A')
		#b = np.flip(v.tobytes())
		#l = len(b)
		#b = np.ascontiguousarray(v, dtype=">i4")
		t = count[b[d]] 
		#t = count[b[l-d-1]] 
		#aux[t] = b[d]
		#aux[t] = v 
		aux[t] = a[i] 
		count[b[d]] += 1
		#count[b[l-d-1]+1] += 1


	#print(f'did i sort (aux) within the method?\n {aux}')

	#copy back
	for i in range(len(a)):
		a[i] = aux[i]

	print(f'did i sort within the method?\n {a}')
		

def msd_sort(a, aux, lo, hi, R, d) :


	if d == 8 :
		return

	if hi <= lo :
		return

	#print(f'msd_sort called with lo={lo}, hi={hi}, R={R}, d={d},')

	#given radix R = could be the 26 letters of the alphabet or the 10 numeric digits or what else?
	#define count arr
	count = np.zeros(R+2, dtype=np.uint64)
	#count = np.zeros(R+1, dtype=np.uint64)
	


	#count frequency
	for i in range(lo, hi+1) :
		#print(f'i is {i} and d is {d}')
		v = a[i]
		b = v.tobytes()
		#print(f'b is: {b}')
		b = b[::-1]
		count[b[d]+1] += 1

	#print(f'after frequency, count is {count}')

	#find positions
	for i in range(R+1):
		count[i+1] += count[i]

	#print(f'after tallying, count is {count}')

	# computing cumulates i.e., start of each position
	for i in range(lo, hi+1) :
		v = a[i]
		b = v.tobytes()
		b = b[::-1]
		t = count[b[d]] 
		aux[t] = a[i] 
		count[b[d]] += 1


	#print(f'did i sort (aux) within the method?\n {aux}')

	#copy back
	for i in range(lo, hi+1) :
		a[i] = aux[i-lo]

	#print(f'did i sort within the method?\n {a}')
	for i in range(R):
		#msd_sort(a, aux, lo, hi, R, d) :
		elo = lo + count[i]
		ehi = lo + count[i+1]-1
		msd_sort(a, aux, int(elo), int(ehi), R, d+1)
 


#https://algs4.cs.princeton.edu/51radix/MSD.java.html
#private static void sort(int[] a, int lo, int hi, int d, int[] aux) {
def msd_book(a, lo, hi, d, aux) :

	#MSD sort from a[lo] to a[hi], starting at the dth byte

	if hi <= lo + CUTOFF :
		a = a.sort()
		return


	count = np.zeros(R+2, dtype=np.uint64)
	mask = R - 1 # 0xFF
	shift = BITS_PER_INT - BITS_PER_BYTE * d - BITS_PER_BYTE	
	for i in range(lo, hi+1) :
		c = (a[i] >> shift) & mask
		count[c+1] += 1

	#transform counts to indices
	for r in range(R) :
		count[r+1] += count[r]


	#for most significant byte, 0x80-0xFF comes before 0x00-0x7F
	if d == 0 :
		half = int(R/2)
		shift1 = count[R] - count[half]
		shift2 = count[half]
		count[R] = shift1 + count[1] #to simplify recursive calls later

		for r in range(half) :
			count[r] += shift1
		for r in range(half, R) :
			count[r] -= shift2
			
	#distribute
	for i in range(lo, hi+1) :
		c = (a[i] >> shift) & mask
		aux[count[c]] = a[i]
		count[c] += 1

	#copy back
	for i in range(lo, hi+1) :
		a[i] = aux[i-lo]

	if d == 7 :
		return # no more bits

	#special case for most significant byte
	half = int(R/2)
	if d == 0 and count[half] > 0 :
		ehi = int(lo + count[half] - 1)
		msd_book(a, lo, ehi, d+1, aux)

  #special case for other bytes
	if d != 0 and count[0] > 0 :
		ehi = int(lo + count[0] - 1)
		msd_book(a, lo, ehi, d+1, aux)

  #recursively sort for each character
  # (could skip r = R/2 for d = 0 and skip r = R for d > 0)
	for r in range(R) :
		if count[r+1] > count[r] :
			elo = int(lo + count[r])
			ehi = int(lo + count[r+1]-1)
			msd_book(a, elo, ehi, d+1, aux)
	


def test_baseline() :

	sz=12000
	p = 63
	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**p, size=sz)

	#for i in a:	
		#print(i.tobytes()[::-1].hex())
		 
	aux = np.sort(a) 

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(aux)}')




def test_msd_book() :

	sz=120
	p = 63
	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**p, size=sz)

	#define aux arr
	aux = np.zeros(len(a), dtype=np.uint64)


	#for i in a:	
		#print(i.tobytes()[::-1].hex())
		 
	msd_book(a, 0, sz-1, 0, aux)
 

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(a)}')


	for i in a:
		print(i)
		
		#print(i.tobytes()[::-1].hex())

	for i in range(1, len(a)) :
		if a[i] < a[i-1] :
			print(f'mistake is at: {a[i]} OR {a[i].tobytes().hex()} and {a[i-1]} OR {a[i-1].tobytes().hex()}')



def test_msd_sort() :

	R = 256
	sz=12
	p = 6
	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**p, size=sz)

	#define aux arr
	aux = np.zeros(len(a), dtype=np.uint64)


	#for i in a:	
		#print(i.tobytes()[::-1].hex())
		 
	msd_sort(a, aux, 0, sz-1, R, 0)
 

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(a)}')


	for i in a:
		print(i)
		
		#print(i.tobytes()[::-1].hex())

	for i in range(1, len(a)) :
		if a[i] < a[i-1] :
			print(f'mistake is at: {a[i]} OR {a[i].tobytes().hex()} and {a[i-1]} OR {a[i-1].tobytes().hex()}')


def test_sort() :


	rng = np.random.default_rng(12345)
	a = rng.integers(low=0, high=2**63, size=120)

	print(a)

	for i in a:
		print(i.tobytes()[::-1].hex())

	#print(a[0])
	#print(a[0].tobytes())

	"""
	b = a[0].tobytes()
	for i in b:
		print(i)
	"""

	#print(a)
	# if my Radix is the possible byte values of an integer, then i've got 256 possible values so R = 256
	counting_sort(a, 256, 0)
 
	counting_sort(a, 256, 1)
	#print(a)

	is_sorted = lambda b: np.all(b[:-1] <= b[1:])

	print(f'did it work? {is_sorted(a)}')

	for i in a:
		print(i.tobytes().hex())

	for i in a:
		print(i)
	"""
	vhex = np.vectorize(hex)
	#vhex(a)
	print(vhex(a))
	"""

	print(f'\tEND test_sort')




def main():

	#test_sort()
	#test_msd_sort()
	test_msd_book()

	test_baseline()




if __name__ == "__main__":
    main()


