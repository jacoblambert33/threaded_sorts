use sort_utils::*;
use std::cmp::Ordering;
use std::time::SystemTime;

use rayon::prelude::*;

fn main() {
    //test_from_main();
    //test_ser_merge_sort_via_main();
    //test_par_merge_sort_via_main();
    test_rayon_par_sort_baseline();
}
//first step to parallelize array work without locks....slices
fn portion_work(part: &mut [u64]) {
    println!("received: {:?}", part);
}

fn test_rayon_par_sort_baseline() {
    let n = 100_000_000; //1_000_000_000;

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=u64::MAX));
    }

    let mut w = v.clone();

    let start = SystemTime::now();

    w.sort();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "sort took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );
    //v.sort();

    //println!("{:?}", v);

    let start = SystemTime::now();

    v.par_sort();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "par_sort took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );

    //println!("{:?}", v);

    assert!(is_sorted(&v));
}

fn test_ser_merge_sort_via_main() {
    let n = 10_000_000;

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

    //println!("{:?}", v);

    let hi = v.len() - 1;
    let mid = hi / 2;

    let mut aux: Vec<u64> = v.clone();

    let start = SystemTime::now();
    sort_utils::merge(&mut v, &mut aux, 0, mid, hi);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    //println!("{:?}", v);
    println!(
        "it took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );

    assert!(is_sorted(&v));
}

fn test_par_merge_sort_via_main() {
    let n = 10_000_000;

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

    //println!("{:?}", v);

    let hi = v.len() - 1;
    let mid = hi / 2;

    let start = SystemTime::now();
    p_merge(&mut v, 0, mid, hi);

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    //println!("{:?}", v);
    println!(
        "it took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );

    assert!(is_sorted(&v));
}

fn test_from_main() {
    let n = 1_000_000; //1_000_000;

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        //v.push( rand::thread_rng().gen_range(1..=15));
    }

    println!("length of v {}", v.len());

    let hi = v.len() - 1;
    //let cutoff = 255;//<5 secs for 1 mil.
    //let cutoff = 511;//<3 secs for 1 mil.
    //let cutoff = 1023;//~2 secs for 1 mil.
    let cutoff = 2047; //~2 secs for 1 mil.

    mergesort(&mut v, 0, hi, cutoff);
    assert!(is_sorted(&v));
    let worked = is_sorted(&v);

    println!("did it sort? {}", worked);
}

/// exch modifies the vector directly and returns nothing.
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

//copied and modified from https://stackoverflow.com/questions/65415293/implementing-a-parallel-multithreaded-merge-sort-on-vec
fn parallel_sort(data: &mut [u64], threads: usize) {
    let chunks = std::cmp::min(data.len(), threads);
    let _ = crossbeam::scope(|scope| {
        for slice in data.chunks_mut(data.len() / chunks) {
            //scope.spawn(move |_| serial_sort(slice));
            //let n = slice.len();
            //scope.spawn(move |_| mergesort_arr_api(slice, n));
            //scope.spawn(move |_| mergesort_arr_api(slice));
        }
    });
    println!("do i get here?");
    //merge(data, chunks);
    //merge(v, lo, mid, hi);
}

/*
// TODO: can't get this to work so far. saving to show the dead end.
// replaced this idea with the crossbeam crate and a bottom up merge on sorted pieces.
fn mergesort_arr_api(v: &mut [u64]) {
    //fn mergesort_arr_api(v: &mut [u64], len: usize) {

    // copy to aux[]
        // next line - v.clone() - doesn't work.
    //let mut w = v.clone();
    //dst.copy_from_slice(&src[2..]);
    //let len = w.len();
        //tmp workaround to build
    let len = v.len();
    let hi = len - 1;
    /*
    let w : [u64; len] = [0; len];
    for i in 0..len {
        w[i] = v[i];
    }
    */
    let cutoff = 15;
    //mergesort_arr(&mut v, &mut w, 0, hi, cutoff);
    mergesort_arr(&mut v, &mut v, 0, hi, cutoff);
}*/

//for now, get it to work via different types.
fn mergesort_arr(v: &mut [u64], aux: &mut [u64], lo: usize, hi: usize, cutoff: usize) {
    /*
    if hi <= lo {
          return;
      }
      */
    //println!("{:?}", &v);
    if hi <= (lo + cutoff) {
        insertion_sort_arr(v, lo, hi);
        return;
    }

    let mid = lo + (hi - lo) / 2;
    //println!("inside ms: lo is {lo}, mid is {mid}, and hi is {hi}");

    mergesort_arr(v, aux, lo, mid, cutoff);
    mergesort_arr(v, aux, mid + 1, hi, cutoff);

    merge_arr(v, aux, lo, mid, hi);
}

fn merge_arr(a: &mut [u64], aux: &mut [u64], lo: usize, mid: usize, hi: usize) {
    // precondition: a[lo .. mid] and a[mid+1 .. hi] are sorted subarrays
    //println!("{:?}", &a[lo..=mid]);
    //println!("{:?}", &a[mid+1..=hi]);
    //assert!(is_sorted_slice(&a[lo..=mid]));
    //assert!(is_sorted_slice(&a[mid+1..=hi]));

    // merge back to a[]
    let mut i = lo;
    let mut j = mid + 1; // j = mid+1;
                         // for (int k = lo; k <= hi; k++) {
    for k in lo..=hi {
        //println!("k is {k}");
        if i > mid {
            a[k] = aux[j];
            j = j + 1;
        } else if j > hi {
            a[k] = aux[i];
            i = i + 1;
        } else if less_arr(&aux, j, i) {
            //println!("k is {k}");
            a[k] = aux[j];
            j = j + 1;
        } else {
            a[k] = aux[i];
            i = i + 1;
        }
    }

    //println!("{:?}", &a);
    //assert!(is_sorted(&a));
}

fn mergesort(v: &mut Vec<u64>, lo: usize, hi: usize, cutoff: usize) {
    /*
    if hi <= lo {
          return;
      }
      */
    //println!("{:?}", &v);
    if hi <= (lo + cutoff) {
        insertion_sort(v, lo, hi);
        return;
    }

    let mid = lo + (hi - lo) / 2;
    //println!("inside ms: lo is {lo}, mid is {mid}, and hi is {hi}");

    mergesort(v, lo, mid, cutoff);
    mergesort(v, mid + 1, hi, cutoff);

    merge(v, lo, mid, hi);
}

fn merge(a: &mut Vec<u64>, lo: usize, mid: usize, hi: usize) {
    // precondition: a[lo .. mid] and a[mid+1 .. hi] are sorted subarrays
    //println!("{:?}", &a[lo..=mid]);
    //println!("{:?}", &a[mid+1..=hi]);
    //assert!(is_sorted_slice(&a[lo..=mid]));
    //assert!(is_sorted_slice(&a[mid+1..=hi]));

    // copy to aux[]
    let aux = a.clone();

    // merge back to a[]
    let mut i = lo;
    let mut j = mid + 1; // j = mid+1;
                         // for (int k = lo; k <= hi; k++) {
    for k in lo..=hi {
        //println!("k is {k}");
        if i > mid {
            a[k] = aux[j];
            j = j + 1;
        } else if j > hi {
            a[k] = aux[i];
            i = i + 1;
        } else if less(&aux, j, i) {
            //println!("k is {k}");
            a[k] = aux[j];
            j = j + 1;
        } else {
            a[k] = aux[i];
            i = i + 1;
        }
    }

    //println!("{:?}", &a);
    //assert!(is_sorted(&a));
}

#[test]
fn test_merge_small() {
    let mut v = Vec::<u64>::new();
    v.push(2);
    v.push(4);
    v.push(6);
    v.push(1);
    v.push(3);
    v.push(5);

    let hi = v.len() - 1;
    let mid = hi / 2;
    println!("before: lo is 0, mid is {mid}, and hi is {hi}");
    merge(&mut v, 0, mid, hi);
    assert!(is_sorted(&v));
}

use rand::Rng;

#[test]
fn test_merge_medium() {
    let n = 1_000_000;

    let mut v = Vec::<u64>::new();
    let mut w = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=u64::MAX));
    }
    for _i in 0..n {
        w.push(rand::thread_rng().gen_range(1..=u64::MAX));
    }

    v.sort();
    w.sort();

    println!("length of v {} and w {}", v.len(), w.len());
    v.append(&mut w);
    println!("length of v {} and w {}", v.len(), w.len());

    let hi = v.len() - 1;
    let m = hi / 2;

    merge(&mut v, 0, m, hi);
    assert!(is_sorted(&v));
}

#[test]
fn test_mergesort_small() {
    let mut v = Vec::<u64>::new();
    v.push(2);
    v.push(8);
    v.push(4);
    v.push(1);
    v.push(3);
    v.push(5);
    v.push(2);
    v.push(9);
    v.push(6);

    println!("before sort: {:?}", &v);

    println!("length of v {}", v.len());
    let hi = v.len() - 1;
    let cutoff = 15;

    println!("before: lo is 0 and hi is {hi}");
    mergesort(&mut v, 0, hi, cutoff);

    println!("after sort: {:?}", &v);
    assert!(is_sorted(&v));
}

#[test]
fn test_mergesort_medium() {
    //let n = 24; //10_000; //1_000_000;
    let n = 100_000; //1_000_000;

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        //v.push( rand::thread_rng().gen_range(1..=15));
    }

    println!("length of v {}", v.len());

    let hi = v.len() - 1;
    let cutoff = 15;

    mergesort(&mut v, 0, hi, cutoff);
    assert!(is_sorted(&v));
}

#[test]
fn test_divide_array() {
    let n = 8;

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=15));
    }

    println!("length of v {}", v.len());
    println!("v {:?}", v);

    let mut i = 0;
    while i < n {
        println!("i is {i}");
        i = i + 2;
        portion_work(&mut v[i - 2..i]);
    }

    let mut width = 1;
    let mut lo = 0;

    while width < n {
        while lo < n - width {
            lo = lo + width + width;
        }
        width = width * 2;
    }

    /*
        for (int len = 1; len < n; len *= 2) {
                for (int lo = 0; lo < n-len; lo += len+len) {
                    int mid  = lo+len-1;
                    int hi = Math.min(lo+len+len-1, n-1);
                    merge(a, aux, lo, mid, hi);
                }
            }
    */
}

#[test]
fn test_parallel_merge() {}
