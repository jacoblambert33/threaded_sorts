use std::cmp;
use std::time::SystemTime;


pub fn find_split_point(v: &Vec<u64>, p: usize, r: usize, x: u64) -> usize {

	let mut lo : usize = p;
	let mut hi : usize = r+1; 
	while lo < hi {
		let mid : usize = (lo + hi) / 2;
		if x <= v[mid] {
			hi = mid;
		}
		else {
			lo = mid + 1; 
		}
	}
	lo
}


pub fn merge(v: &mut Vec<u64>, aux: &mut Vec<u64>, lo: usize, mid: usize, hi: usize) {
    //println!("\tmerge was called...");
    for k in lo..=hi {
        aux[k] = v[k];
    }

    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..=hi {
        if i > mid {
            v[k] = aux[j];
            j = j + 1;
        } else if j > hi {
            v[k] = aux[i];
            i = i + 1;
        } else if less(aux, j, i) {
            v[k] = aux[j];
            j = j + 1;
        } else {
            v[k] = aux[i];
            i = i + 1;
        }
    }
}

pub fn bottom_up_merge(v: &mut Vec<u64>, n: usize, chunk_len: usize) {
    let mut aux = v.clone();

    let mut len = chunk_len;
    //let mut lo = 0; //this is NOT the right place to define low.
    //println!("vector len (n) is {n}");
    //println!("\tincoming v is: {:?}", v);
    while len < n {
        //println!("len is {len}");
        let mut lo = 0;
        while lo < n - len {
            //println!("\tlo is {lo}");
            let mid = lo + len - 1;
            let hi = cmp::min(lo + len + len - 1, n - 1);
            merge(v, &mut aux, lo, mid, hi);

            //println!("\teach v is: {:?}", v);
            lo = lo + len + len;
        }
        len = len * 2;
    }
}

/// sort a bunch of small arrays with threads and merge them together.
pub fn p_merge_sorted_groups(v: &mut Vec<u64>, lo: usize, hi: usize, n_threads: usize) {
    let threads = n_threads; // 4; //no of threads
    let len = hi - lo + 1;
    let chunks = std::cmp::min(len, threads);
    let _ = crossbeam::scope(|scope| {
        for slice in v.chunks_mut(len / chunks) {
            //println!("this slice is: {:?}", slice);
            //scope.spawn(move |_| insertion_sort_arr(slice, 0, slice.len()-1));
            scope.spawn(move |_| slice.sort());
        }
    });
    //merge(data, chunks);
    let chunk_len = len / chunks;
    let start = SystemTime::now();
    bottom_up_merge(v, len, chunk_len);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    //println!("final v is: {:?} for len {len}", v);
    //println!("chunks is: {chunks}");
    println!(
        "serial merge took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );
}

pub fn insertion_sort_arr(v: &mut [u64], lo: usize, hi: usize) {
    //let mut vv = v;

    //println!("outer range inclusive should be {} to {}", lo+1, hi);
    for i in lo + 1..=hi {
        //println!("i is {i}");
        //println!("inner range inclusive should be {} to {}", i, lo+1);
        for j in (lo + 1..=i).rev() {
            //println!("\tj is {j}");
            if !less_arr(v, j, j - 1) {
                break;
            }
            exch_arr(v, j, j - 1);
        }
    }

    //tmp remove:
    //assert!(is_sorted(&v));
    let s = &v[lo..=hi];
    assert!(is_sorted_slice(&s));
}

pub fn exch_arr(v: &mut [u64], i: usize, j: usize) {
    let &t: &u64 = &v[i];
    v[i] = v[j];
    v[j] = t;
}

/// less is readonly - we need references; not changing the array but reading it.
pub fn less_arr(v: &[u64], i: usize, j: usize) -> bool {
    let first: &u64 = &v[i];
    let second: &u64 = &v[j];

    match first.cmp(second) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => false,
    }
}

pub fn insertion_sort(v: &mut Vec<u64>, lo: usize, hi: usize) {
    //let mut vv = v;

    //println!("outer range inclusive should be {} to {}", lo+1, hi);
    for i in lo + 1..=hi {
        //println!("i is {i}");
        //println!("inner range inclusive should be {} to {}", i, lo+1);
        for j in (lo + 1..=i).rev() {
            //println!("\tj is {j}");
            if !less(v, j, j - 1) {
                break;
            }
            exch(v, j, j - 1);
        }
    }

    //tmp remove:
    //assert!(is_sorted(&v));
    let s = &v[lo..=hi];
    assert!(is_sorted_slice(&s));
}

/*
fn insertion_sort(Vec<u64> a, u32 lo, u32 hi) {
void insertion_sort(unsigned long long a[], int lo, int hi) {
  for (int i = lo + 1; i < hi; i++) {
    for (int j = i; j > lo && less(a[j], a[j - 1]); j--) {
      exch(a, j, j - 1);
    }
  }
  assert(is_sorted(a, lo, hi));
  return;
}
}*/

use std::cmp::Ordering;
//use std::io;

/// is_sorted confirms the vector is sorted.
pub fn is_sorted(v: &Vec<u64>) -> bool {
    for i in 1..v.len() {
        if &v[i] < &v[i - 1] {
            return false;
        }
    }
    true
}

/// is_sorted confirms the array is sorted. this version for slices.
/// TODO: refactor bc rust can surely handle one version; i'm just not sure how yet.
pub fn is_sorted_slice(v: &[u64]) -> bool {
    for i in 1..v.len() {
        if &v[i] < &v[i - 1] {
            return false;
        }
    }
    true
}

/// exch modifies the vector directly and returns nothing.
pub fn exch(v: &mut Vec<u64>, i: usize, j: usize) {
    let &t: &u64 = &v[i];
    v[i] = v[j];
    v[j] = t;
}

/// less is readonly - we need references; not changing the array but reading it.
pub fn less(v: &Vec<u64>, i: usize, j: usize) -> bool {
    let first: &u64 = &v[i];
    let second: &u64 = &v[j];

    match first.cmp(second) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rand::Rng;

    use std::time::SystemTime;

    #[test]
    fn test_less() {
        //let a : [u64] = [ 1, 2, 3 ];
        let v = vec![1, 2, 2];

        let b = less(&v, 0, 1);

        assert!(b);

        let b = less(&v, 1, 0);

        assert!(!b);

        let b = less(&v, 0, 2);

        assert!(b);

        let b = less(&v, 2, 1);

        assert!(!b);
    }

    #[test]
    fn test_exch() {
        let mut v = vec![1, 2, 3];

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);

        exch(&mut v, 0, 1);

        assert_eq!(v[0], 2);
        assert_eq!(v[1], 1);
    }

    #[test]
    fn test_is_sorted() {
        let v = vec![1, 2, 3];

        assert!(is_sorted(&v));

        let v = vec![2, 2, 2];

        assert!(is_sorted(&v));

        let v = vec![2, 1, 1];

        assert!(!is_sorted(&v));

        let v = vec![1, 1, 1, 2];

        assert!(is_sorted(&v));
    }

    #[test]
    fn test_is_sorted_slice() {
        let v: [u64; 3] = [1, 2, 3];

        assert!(is_sorted_slice(&v));

        let v: [u64; 3] = [2, 2, 2];

        assert!(is_sorted_slice(&v));

        let v: [u64; 3] = [2, 1, 1];

        assert!(!is_sorted_slice(&v));

        let v: [u64; 4] = [1, 1, 1, 2];

        assert!(is_sorted_slice(&v));
    }

    #[test]
    fn test_insertion_small() {
        let mut v = vec![4, 3, 2, 1];

        println!("{:?}", v);

        assert!(!is_sorted(&v));

        let hi = v.len() - 1;
        insertion_sort(&mut v, 0, hi);

        println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    //use rand::rng;

    #[test]
    fn test_insertion_medium() {
        let n = 10_000;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

        //println!("{:?}", v);

        assert!(!is_sorted(&v));

        let hi = v.len() - 1;

        let start = SystemTime::now();

        insertion_sort(&mut v, 0, hi);

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        //println!("it took {} seconds", duration.as_nanos());
        //println!("it took {} seconds", duration.as_micros());
        println!(
            "it took {}.{} seconds",
            duration.as_millis() / 1000,
            duration.as_millis() % 1000
        );

        let lo = v[0];
        let hi = v[n - 1];

        println!("sorted with lowest element {lo} and highest {hi}");
        /*
            let x = timeit(|| insertion_sort(&mut v, 0, hi));
            println!("x is {:?}", x);
        */

        //println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn test_p_insertion_small() {
        let mut v = vec![4, 3, 2, 1, 8, 9, 7, 6, 3, 2, 4, 1, 5];

        println!("{:?}", v);

        assert!(!is_sorted(&v));

        let hi = v.len() - 1;

        p_merge_sorted_groups(&mut v, 0, hi, 4);
        //p_insertion_sort(&mut v, 0, hi, 4);

        println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn test_p_insertion_medium() {
        let n = 1_000_000;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

        let hi = v.len() - 1;

        let start = SystemTime::now();

        p_merge_sorted_groups(&mut v, 0, hi, 256);
        //p_insertion_sort(&mut v, 0, hi, 256);

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!(
            "it took {}.{} seconds",
            duration.as_millis() / 1000,
            duration.as_millis() % 1000
        );

        let lo = v[0];
        let hi = v[n - 1];

        println!("sorted with lowest element {lo} and highest {hi}");

        assert!(is_sorted(&v));
    }

    #[test]
    fn test_find_split_point_small() {
        //let a : [u64] = [ 1, 2, 3 ];
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

				//find 4 - expect at index 3. 
        let b = find_split_point(&v, 0, v.len()-1, 4);
				assert_eq!(b, 3);

				//find 1 - expect at index 0. 
        let b = find_split_point(&v, 0, v.len()-1, 1);
				assert_eq!(b, 0);

				//find 0 - expect at index 0. 
        let b = find_split_point(&v, 0, v.len()-1, 0);
				assert_eq!(b, 0);

				//find 100 - expect at index 9, i.e., after the array length 
        let b = find_split_point(&v, 0, v.len()-1, 100);
				assert_eq!(b, 9);
    }

    #[test]
    fn test_find_split_point_loop() {

				//idea: create a random vector, sort it, then find every element using binary search one by one. 
				// limitation - doesn't test outside the bounds of the array, but that is captured in small test above.
				let n = 100;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

				v.sort();

				for i in 0..n {
					let b = find_split_point(&v, 0, v.len()-1, v[i]);
					assert_eq!(b, i);
				}
    }




}
