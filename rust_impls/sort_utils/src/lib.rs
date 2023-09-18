use rand::Rng;
use rayon::prelude::*;
use std::cmp;
use std::cmp::min;
use std::cmp::Ord;
use std::fmt::Debug;
use std::marker::Send;
use std::thread;
use std::time::SystemTime;


/// another timer function for no arguments...
pub fn time_fn_noargs(f: &dyn Fn()) {
    let start = SystemTime::now();

    f();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    println!(
        "it took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );

}



/// timer function
pub fn time_fn<T: PartialOrd + Ord + Send + Debug>(
    v: &mut Vec<T>,
    run_size: usize,
    f: &dyn Fn(&mut Vec<T>, usize, bool) -> &mut Vec<T>,
) {
    let start = SystemTime::now();

    let _v = f(v, run_size, false);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    //			println!("{:?}", v);

    //println!("it took {} seconds", duration.as_micros());
    println!(
        "it took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );
}

/// utility function to quickly fill a test vector. not generic.
pub fn fill_vec_u64(num_elems: usize, chunk_size: usize) -> Vec<u64> {
    let mut v: Vec<u64> = vec![0; num_elems];

    v.par_chunks_mut(chunk_size)
        .for_each_init(|| rand::thread_rng(), |rng, chunk| rng.fill(chunk));

    println!(
        "data generated. v is size: {} with capacity: {}",
        v.len(),
        v.capacity()
    );

    v
}

pub fn confirm_sorted_runs<T: PartialOrd + Ord + Send + Debug>(
    v: &mut Vec<T>,
    run_size: usize,
) -> bool {
    let n = v.len();
    let n_grps = n / run_size;
    println!("\tnumber of groups is {}", n_grps);

    for i in 0..=n_grps {
        let start = i * run_size;
        let end = min(start + run_size, v.len());
        println!("\tstart is {}. end is {}", start, end);
        let is_good = is_sorted_slice(&v[start..end]);
        if !is_good {
            //assert!(false);
            return false;
        }
    }
    return true;
}

///
pub fn create_sorted_runs<T: PartialOrd + Ord + Send + Debug>(
    v: &mut Vec<T>,
    run_size: usize,
    do_print_each: bool,
) -> &mut Vec<T> {
    thread::scope(|scope| {
        for slice in v.chunks_mut(run_size) {
            scope.spawn(move || {
                // requires marker Send -  T` cannot be sent between threads safely
                if do_print_each {
                    println!("before: {:?}", slice);
                }
                slice.sort(); //requires the trait Ord.
                if do_print_each {
                    println!("after:  {:?}", slice);
                }
            });
        }
    });
    v
}

pub fn find_split_point(v: &Vec<u64>, p: usize, r: usize, x: u64) -> usize {
    //println!("split-point input: p={p}, r={r}, x={x}");

    let mut lo: usize = p;
    let mut hi: usize = r + 1;
    while lo < hi {
        let mid: usize = (lo + hi) / 2;
        if x <= v[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

pub fn merge_slices(v: &mut [u64], aux: &mut [u64], lo: usize, mid: usize, hi: usize) {
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
        } else if less_arr(aux, j, i) {
            v[k] = aux[j];
            j = j + 1;
        } else {
            v[k] = aux[i];
            i = i + 1;
        }
    }
}

pub fn merge_bu(v: &mut [u64], len: usize) {
    //println!("\tmerge_bu was called with length {len}...");

    //let mut aux = v.clone();
    let mut aux = vec![0; len];
    aux.clone_from_slice(v);

    //println!("[merge_bu] cloned v is: {:?}", aux);

    let lo = 0;
    let hi = len - 1;
    let mid = hi / 2;

    //println!("v lo: {lo}, mid: {mid}, hi: {hi}");

    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..=hi {
        if i > mid {
            v[k] = aux[j];
            j = j + 1;
        } else if j > hi {
            v[k] = aux[i];
            i = i + 1;
        } else if less(&aux, j, i) {
            v[k] = aux[j];
            j = j + 1;
        } else {
            v[k] = aux[i];
            i = i + 1;
        }
    }
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

/// parallel merge - this would be bottom up but this an incremental step....
pub fn p_merge_parallel_merge(v: &mut Vec<u64>, lo: usize, hi: usize, unit_step: usize) {
    let n = hi - lo + 1;

    let mut power = 1;
    while power < n {
        power = power * 2;
    }

    //let mut aux = v.clone();
    //let mut aux = &v[..];

    //let threads = n_threads; // 4; //no of threads
    //let len = hi - lo + 1;
    //let chunks = std::cmp::min(len, threads);
    let _ = crossbeam::scope(|scope| {
        //for slice in v.chunks_mut(len / chunks) {
        for slice in v.chunks_mut(unit_step) {
            //println!("this slice is: {:?}", slice);
            //scope.spawn(move |_| insertion_sort_arr(slice, 0, slice.len()-1));
            scope.spawn(move |_| slice.sort());
            //scope.spawn(move |_| println!("{:?}", slice));
        }
    });

    println!("step1\n{:?}\n", v);

    let mut len = unit_step * 2;
    //let mut start = 0;
    ////let mut end = combined;

    while len <= power {
        //n { //*2 { //> 0 { // < n {
        println!("len is {len}, power is {power}");
        let mut lo = 0;
        //while lo < n - len {
        //while lo < power - len {
        while lo < power - len + 1 {
            //println!("\tlo is {lo}");
            //let mid = lo + len - 1;
            //let hi = cmp::min(lo + len + len - 1, n - 1);
            //merge(v, &mut aux, lo, mid, hi);

            let _a = crossbeam::scope(|scope| {
                for slice in v.chunks_mut(len) {
                    //len {
                    let l = slice.len();
                    scope.spawn(move |_| merge_bu(slice, l)); //combined));
                }
            });

            //println!("\teach v is: {:?}", v);
            lo = lo + len + len;
        }
        len = len * 2; // / 2;
    }
    /*
        let _a = crossbeam::scope(|scope| {
                    //let mut destination : Vec<&mut [u64]> = aux.chunks_mut(combined).collect();
                    let mut destination : Vec<&mut [u64]> = aux.chunks_mut(combined).collect();

                    //let mut i = 0;
            for slice in v.chunks_mut(combined) {
                println!("this slice is: {:?}", slice);
                            //let this_slice = &mut aux[start..end];
                            //let this_slice = &destination[i];
                            //let l = suuuuuu
                //scope.spawn(move |_| merge_slices(slice, &mut destination[i], 0, unit_step, combined));
                //scope.spawn(move |_| merge_slices(slice, *this_slice, 0, unit_step, combined));
                            let l = slice.len();
                scope.spawn(move |_| merge_bu(slice, l)); //combined));
                            //i = i + 1;
                //merge(v, &mut aux, lo, mid, hi);
                            //start = end;
                            //end = end + combined;
            }
        });
    */
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
pub fn is_sorted<T: PartialOrd + Ord + Send + Debug>(v: &Vec<T>) -> bool {
    //pub fn is_sorted(v: &Vec<u64>) -> bool {
    for i in 1..v.len() {
        if &v[i] < &v[i - 1] {
            return false;
        }
    }
    true
}

/// is_sorted confirms the array is sorted. this version for slices.
pub fn is_sorted_slice<T: PartialOrd + Ord + Send + Debug>(v: &[T]) -> bool {
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

    use std::time::SystemTime;

    /// 14 is a good number for fast unit tests. 24 is a great number to compare tests that you want to have a meaningfully long run without waiting too long.
    fn get_shared_n_els() -> usize {
        1 << 26
    }

    /// experimented with sizes 12-19 and 16 *seems* best (so far).
    fn get_shared_run_size() -> usize {
        1 << 16
    }

    #[test]
    fn t_less() {
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
    fn t_exch() {
        let mut v = vec![1, 2, 3];

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);

        exch(&mut v, 0, 1);

        assert_eq!(v[0], 2);
        assert_eq!(v[1], 1);
    }

    #[test]
    fn t_is_sorted() {
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
    fn t_is_sorted_slice() {
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
    fn t_insertion_small() {
        let mut v = vec![4, 3, 2, 1];

        println!("{:?}", v);

        assert!(!is_sorted(&v));

        let hi = v.len() - 1;
        insertion_sort(&mut v, 0, hi);

        println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_insertion_medium() {
        let n = 1_000; //10_000;

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
        println!(
            "it took {}.{} seconds",
            duration.as_millis() / 1000,
            duration.as_millis() % 1000
        );

        let lo = v[0];
        let hi = v[n - 1];

        println!("sorted with lowest element {lo} and highest {hi}");

        //println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_p_insertion_small() {
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
    fn t_p_insertion_medium() {
        let n = 10_000; //1_000_000;

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
    fn t_find_split_point_small() {
        //let a : [u64] = [ 1, 2, 3 ];
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        //find 4 - expect at index 3.
        let b = find_split_point(&v, 0, v.len() - 1, 4);
        assert_eq!(b, 3);

        //find 1 - expect at index 0.
        let b = find_split_point(&v, 0, v.len() - 1, 1);
        assert_eq!(b, 0);

        //find 100 - expect at index 9, i.e., after the array length
        let b = find_split_point(&v, 0, v.len() - 1, 100);
        assert_eq!(b, 9);
    }

    #[test]
    fn t_find_split_point_loop() {
        //idea: create a random vector, sort it, then find every element using binary search one by one.
        // limitation - doesn't test outside the bounds of the array, but that is captured in small test above.
        let n = 100;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

        v.sort();

        for i in 0..n {
            let b = find_split_point(&v, 0, v.len() - 1, v[i]);
            assert_eq!(b, i);
        }
    }

    #[test]
    fn t_merge_medium() {
        let n = 10_000; //_000;

        let mut v = Vec::<u64>::new();
        let mut w = Vec::<u64>::new();
        for _i in 0..n {
            //v.push(rand::thread_rng().gen_range(1..=u64::MAX));
            v.push(rand::thread_rng().gen_range(1..=20));
            w.push(rand::thread_rng().gen_range(1..=20));
        }

        v.sort();
        w.sort();

        v.append(&mut w);
        //w.clear();  makes a length 0 array....not what i want.
        let mut w = v.clone();

        //println!("{:?}", v);

        let hi = v.len() - 1;
        //let mid = hi / 2;
        let start = SystemTime::now();

        merge(&mut v, &mut w, 0, n - 1, hi);

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        //println!("it took {} seconds", duration.as_nanos());
        //println!("it took {} seconds", duration.as_micros());
        println!(
            "it took {}.{} seconds",
            duration.as_millis() / 1000,
            duration.as_millis() % 1000
        );

        //println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_rayon_par_sort() {
        let n = get_shared_n_els();
        println!("n is: {}", n);

        let mut v: Vec<u64> = fill_vec_u64(n, 10_000);

        let start = SystemTime::now();

        v.par_sort();

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!(
            "it took {}.{} seconds",
            duration.as_millis() / 1000,
            duration.as_millis() % 1000
        );

        //println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_my_parallel_merge() {
        let n = 100_000; //10_000_000;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

        let hi = v.len() - 1;

        let start = SystemTime::now();

        p_merge_sorted_groups(&mut v, 0, hi, 4);
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
    fn t_p_merge_parallel_merge() {
        //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3 ];
        //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3, 7, 2 ];

        let n = 1 << 18; //25;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=50)); //=u64::MAX));
        }

        println!("{:?}", v);

        assert!(!is_sorted(&v));

        let hi = v.len() - 1;

        //p_merge_parallel_merge(&mut v, 0, hi, 4096); //22s for 1<<20
        //p_merge_parallel_merge(&mut v, 0, hi, 16384); //3.98s for 1<<20
        //p_merge_parallel_merge(&mut v, 0, hi, 262144); //1.3s for 1<<20
        //p_merge_parallel_merge(&mut v, 0, hi, 2097152); //58s for 1<<25
        //p_merge_parallel_merge(&mut v, 0, hi, 8388608); //58s for 1<<25
        p_merge_parallel_merge(&mut v, 0, hi, 16777216); //58s for 1<<25
                                                         /*
                                                         2097152
                                                         4194304
                                                         8388608
                                                         16777216
                                                         */
        //p_insertion_sort(&mut v, 0, hi, 4);

        println!("{:?}", v);

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_create_sorted_runs_small() {
        let mut v = vec![5, 4, 3, 2, 1];
        println!("{:?}", v);
        let run_size: usize = 2;
        let v = create_sorted_runs(&mut v, run_size, true);
        println!("{:?}", v);

        assert!(confirm_sorted_runs(v, run_size));
    }

    #[test]
    fn t_create_sorted_runs_medium() {
        let n = 1 << 6; //25;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=100)); //=u64::MAX));
        }

        println!("{:?}", v);
        let run_size: usize = 6;
        let v = create_sorted_runs(&mut v, run_size, true);
        println!("{:?}", v);

        assert!(confirm_sorted_runs(v, run_size));
        /*
                let n_grps = n / run_size;
                println!("\tnumber of groups is {}", n_grps);

                for i in 0..=n_grps {
                    let start = i * run_size;
                    let end = min(start + run_size, v.len());
                    println!("\tstart is {}. end is {}", start, end);
                    let is_good = is_sorted_slice(&v[start..end]);
                    if !is_good {
                        assert!(false);
                    }
                }
        */
    }

    #[test]
    fn t_create_sorted_runs_large() {
        let n = get_shared_n_els();
        println!("n is: {}", n);

        let run_size: usize = get_shared_run_size();
        println!("run_size is: {}", run_size);

        let mut v: Vec<u64> = fill_vec_u64(n, run_size);

        let _v = create_sorted_runs(&mut v, run_size, false);

    }

    #[test]
    fn t_baseline_sort() {
        let n = get_shared_n_els();
        println!("n is: {}", n);

        let mut v: Vec<u64> = fill_vec_u64(n, 10_000);

        v.sort();

        assert!(is_sorted(&v));
    }

    #[test]
    fn t_time_fn() {
        let n = get_shared_n_els();
        println!("n is: {}", n);

        let run_size: usize = get_shared_run_size();
        println!("run_size is: {}", run_size);

        let mut v: Vec<u64> = fill_vec_u64(n, run_size);

        //        let _v = create_sorted_runs(&mut v, run_size, false);
        //time_fn(&mut v, run_size, create_sorted_runs(&mut v, run_size, false));
        time_fn(&mut v, run_size, &create_sorted_runs);
    }

	#[test]
	fn t_time_fn_noargs() {

		//time_fn_noargs(&t_exch); 
		//time_fn_noargs(&t_baseline_sort); 
		time_fn_noargs(&t_create_sorted_runs_large); 


	}
}
