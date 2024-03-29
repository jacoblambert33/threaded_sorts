use sort_utils::*;

use std::time::SystemTime;

use std::thread;

struct WrapperType(*mut Vec<u64>);
//struct RefRawPointer(&*mut Vec<u64>);
//struct SendPtr<T>(*const T)
//unsafe impl Send<T> for SendPtr<T> { }
//unsafe impl Sync<T> for SendPtr<T>  { }

unsafe impl Send for WrapperType {}
unsafe impl Sync for WrapperType {}
//unsafe impl Send for RefRawPointer {}
//unsafe impl Sync for RefRawPointer {}
//unsafe impl Send for *mut Vec<u64> {} //error raw pointers are always foreign.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

fn main() {
    //test_from_main();
    //test_ser_merge_sort_via_main();
    //test_par_merge_sort_via_main();
    //test_p_merge_small();
}

fn portion_work(part: &mut [u64]) {
    println!("received: {:?}", part);
}

/*
//START std::thread first attempt
use std::thread;


fn p_merge_t(v: &mut Vec<u64>, p: usize, q: usize, r: usize ) {

    let mut w = v.clone();
    //println!("w before p_merge_aux: {:?}", w);
    //p_merge_aux(v, p, q, q+1, r, &mut w, p);
    let pp = p as i64;
    let qq = q as i64;
    let rr = r as i64;
    p_merge_aux_ints_t(&mut v[..], pp, qq, qq+1, rr, &mut w, pp);
    //println!("w after p_merge_aux: {:?}", w);
    for i in p..=r {
        v[i] = w[i];
    }
}

//unsafe fn p_merge_aux_ints_t(v: &mut Vec<u64>, p1: i64, r1: i64, p2: i64, r2: i64, w: &mut Vec<u64>, p3: i64) {
fn p_merge_aux_ints_t(v: &mut [u64], p1: i64, r1: i64, p2: i64, r2: i64, w: &mut Vec<u64>, p3: i64) {

    //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    let mut mr2 = r2;

    if p1 > r1 && p2 > r2 { //both subarrays are empty
        //println!("mr1={mr1}, mp1={mp1}, mr2={mr2}, mp2={mp2}");
        return;
    };
    //println!("input past subarrs empty trap: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");

    if (r1 - p1) < (r2 - p2) { //second subarr is smaller than the first.
        let t1 = mp1;
        mp1 = mp2;
        mp2 = t1;
        let t2 = mr1;
        mr1 = mr2;
        mr2 = t2;
    }

    let q1 = (mp1 + mr1)/2;  // midpoint of A[p1 : r1]

    let x = v[q1 as usize]; 		// median of A[p1 : r1] is pivot x

    //fix overflow issue carried into split point:
    if mr2 == -1 {
        mr2 = 0;
    }

    //let q2 = find_split_point(v, mp2 as usize, mr2 as usize, x); // split A[p2 : r2] around x
    let q2 = find_split_point(&v.to_vec(), mp2 as usize, mr2 as usize, x); // split A[p2 : r2] around x
    let q3 = p3 + (q1 - mp1) + (q2 as i64 - mp2);  // where x belongs in B...
    /*
    if q3 == 10 {
        //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
        println!("error input: mp1={mp1}, mr1={mr1}, mp2={mp2}, mr2={mr2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
        println!("{:?}", w);
    } */
    w[q3 as usize] = x;  // ... put it there


    // doesn't need to be a vector...i know it's two each time. but first time, so using a working example...
    let mut thread_handles = vec![];

    //spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
    thread_handles.push(
            //thread::spawn(move || process_files(worklist))
                        thread::spawn(move||
                                p_merge_aux_ints_t(v, mp1, q1 - 1, mp2, q2 as i64 - 1, w, p3)
                            )
        );

    //spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
    thread_handles.push(
            //thread::spawn(move || process_files(worklist))
                        thread::spawn(move||
                                p_merge_aux_ints_t(v, q1 + 1, mr1, q2 as i64, mr2, w, q3 + 1)
                            )
        );


    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap();
    }

} //END of std::thread first attempt
*/

/*
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
"
*/

//replaced with p_merge_aux_ints due to subtract/overflow issue.
//fn p_merge_aux(

#[test]
fn test_from_main() {
    let n = 1000; //1_000_000; //1_000_000;

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

/*
//copied and modified from https://stackoverflow.com/questions/65415293/implementing-a-parallel-multithreaded-merge-sort-on-vec
fn parallel_sort(data: &mut [u64], threads: usize) {
    let chunks = std::cmp::min(data.len(), threads);
    let _ = crossbeam::scope(|scope| {
        for slice in data.chunks_mut(data.len() / chunks) {
            //scope.spawn(move |_| serial_sort(slice));
            //let n = slice.len();
            //scope.spawn(move |_| mergesort_arr_api(slice, n));
            scope.spawn(move |_| mergesort_arr_api(slice));
        }
    });
    println!("do i get here?");
    //merge(data, chunks);
    //merge(v, lo, mid, hi);
}
*/

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
    //let n = 1_000_000;
    let n = 10_000;

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
    //let n = 100_000; //1_000_000;
    let n = 1_000;

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
fn test_p_merge_small() {
    let mut v = vec![1, 3, 5, 7, 9, 2, 4, 6, 8];

    println!("{:?}", v);

    let hi = v.len() - 1;
    //let mid = hi / 2;
    let mid = 4;
    //let mid = 5;
    merge_lib::p_merge(&mut v, 0, mid, hi);

    println!("{:?}", v);

    assert!(is_sorted(&v));
}

#[test]
fn test_p_merge_medium() {
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

    //println!("{:?}", v);

    let hi = v.len() - 1;
    let mid = hi / 2;
    let start = SystemTime::now();

    merge_lib::p_merge(&mut v, 0, mid, hi);

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

use merge_lib;

#[test]
fn test_from_this_lib() {
    let x = merge_lib::can_call_me();
    assert_eq!(x, 5);
}

#[test]
fn test_ownership_rules() {
    //WRONG let *const s = String::from("hello");  // s comes into scope
    //WRONGlet s : *const String  = String::from("hello");  // s comes into scope
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        //println!("can't print s here by default: {s}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

#[test]
fn test_smallest_unsafe_rawpointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("raw pointer: {:?}", r1);
    println!("raw pointer: {:?}", r2);

    let mut p: Vec<u64> = vec![1, 2, 3];

    let p1 = &mut p as *mut Vec<u64>;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            //thread::sleep(Duration::from_millis(1));
        }
    });

    println!("vector: {:?}", p);
    println!("raw pointer: {:?}", p1);

    handle.join().unwrap();
}

//fn increment_array_elements_via_threads(v: *mut Vec<u64>, len: usize) { //doesn't work
fn increment_vector_elements_via_threads(v: WrapperType, len: usize) {
    //doesn't work
    //unsafe fn increment_array_elements_via_threads(v: *mut Vec<u64>, len: usize) { //doesn't fix it.
    let mut handle_array = vec![];

    //let v : WrapperType = v;
    //for i in v.len() {
    for i in 0..len {
        println!("{i}");
        unsafe {
            let handle = thread::spawn(|| {
                //unsafe { let handle = thread::spawn(move || {
                //let handle = thread::spawn(move || {
                //println!("in func! {:?}", v.offset(i as isize));
                //println!("in func! {:?}", v.0.offset(i as isize));
                //println!("in func! {:?}", *v.0);
                //println!("in func! {:?}", (*v.0)[i]);
                //println!("in func! {:?}", v.0[i]);
                //println!("in func! {:?}", v.0.offset(i as isize));
                //v[i] = v[i] + 1; //doesn't work.
                //v.offset(i) = v.offset(i) + 1;
            });
            handle_array.push(handle);
        }
    }

    while handle_array.len() > 0 {
        let handle = handle_array.pop();
        handle
            .expect("i'm writing unsafe code and i'm tracking indexes")
            .join()
            .unwrap();
    }
    /*
    let handle =
        thread::spawn(|| {
            for i in 1..10 {
                println!("in func!");
                //thread::sleep(Duration::from_millis(1));
            }
        });
    */
}

/*
//fn increment_array_elements_via_threads(v: *mut Vec<u64>, len: usize) { //doesn't work
fn increment_array_elements_via_threads(v: &mut [u64], len: usize) { //doesn't work
//unsafe fn increment_array_elements_via_threads(v: *mut Vec<u64>, len: usize) { //doesn't fix it.
    let mut handle_array = vec![];

        //let v : WrapperType = v;
    //for i in v.len() {
    for i in 0..len {
        println!("{i}");
        //unsafe { let handle = thread::spawn(|| { //error - fn outlives ...etc. use move it seems
        unsafe { let handle = thread::spawn(|| { //error - fn outlives ...etc. use move it seems
        //unsafe { let handle = thread::spawn(move |i| { //error - expected closure that takes zero arguments.
        //let handle = thread::spawn(move || {
            //println!("in func! {:?}", v.offset(i as isize));
            //println!("in func! {:?}", v.0.offset(i as isize));
            println!("in func! {:?}", v[0]);
            //println!("in func! {:?}", (*v.0)[i]);
            //println!("in func! {:?}", v.0[i]);
            //println!("in func! {:?}", v.0.offset(i as isize));
                        //v[i] = v[i] + 1; //doesn't work.
                        //v.offset(i) = v.offset(i) + 1;
        });
        handle_array.push(handle);
                }
    }

    while handle_array.len() > 0 {
        let handle = handle_array.pop();
        handle
            .expect("i'm writing unsafe code and i'm tracking indexes")
            .join()
            .unwrap();
    }
    /*
    let handle =
        thread::spawn(|| {
            for i in 1..10 {
                println!("in func!");
                //thread::sleep(Duration::from_millis(1));
            }
        });
    */
}
*/
