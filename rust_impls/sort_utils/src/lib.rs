use std::cmp;
use std::time::SystemTime;
//use std::thread;

use rayon::prelude::*;

use std::{thread, time::Duration};

//fn main() {
fn experiment_scoped_threads() {
    //let mut vec = vec![1, 2, 3, 4, 5];
    let mut vec = vec![5, 4, 3, 2, 1];
    //let l = vec.len();

    thread::scope(|scope| {
        //for e in &mut vec {
        for slice in vec.chunks_mut(3) {
            //for slice in vec.chunks_mut(vec.len() / 2) {
            scope.spawn(move || {
                thread::sleep(Duration::from_secs(1));
                //*e += 1;
                println!("{:?}", slice);
                slice.sort();
            });
        }
    });

    println!("{:?}", vec);
}

/* //START std::thread first attempt

pub fn p_merge_t(v: &mut Vec<u64>, p: usize, q: usize, r: usize ) {

    let mut w = v.clone();
    //println!("w before p_merge_aux: {:?}", w);
    //p_merge_aux(v, p, q, q+1, r, &mut w, p);
    let pp = p as i64;
    let qq = q as i64;
    let rr = r as i64;
    p_merge_aux_ints_t(v, pp, qq, qq+1, rr, &mut w, pp);
    //println!("w after p_merge_aux: {:?}", w);
    for i in p..=r {
        v[i] = w[i];
    }
}

fn p_merge_aux_ints_t(v: &mut Vec<u64>, p1: i64, r1: i64, p2: i64, r2: i64, w: &mut Vec<u64>, p3: i64) {

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

    let q2 = find_split_point(v, mp2 as usize, mr2 as usize, x); // split A[p2 : r2] around x
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
                        thread::spawn(||
                                p_merge_aux_ints_t(v, mp1, q1 - 1, mp2, q2 as i64 - 1, w, p3)
                            )
        );

    //spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
    thread_handles.push(
            //thread::spawn(move || process_files(worklist))
                        thread::spawn(||
                                p_merge_aux_ints_t(v, q1 + 1, mr1, q2 as i64, mr2, w, q3 + 1)
                            )
        );


    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap();
    }

}


*/
 //END of std::thread first attempt

/*
P-MERGE.A; p; q; r/
1 let B[p : r] be a new array // allocate scratch array
2 P-MERGE-AUX (A; p; q; q + 1; r; B; p) // merge from A into B
3 parallel for i = p to r // copy B back to A in parallel
4 A[i] = B[i]
*/

pub fn p_merge(v: &mut Vec<u64>, p: usize, q: usize, r: usize) {
    let mut w = v.clone();
    //println!("w before p_merge_aux: {:?}", w);
    //p_merge_aux(v, p, q, q+1, r, &mut w, p);
    let pp = p as i64;
    let qq = q as i64;
    let rr = r as i64;
    p_merge_aux_ints(v, pp, qq, qq + 1, rr, &mut w, pp);
    //parallel_merge_aux_ints(v, pp, qq, qq+1, rr, &mut w, pp);

    //println!("w after p_merge_aux: {:?}", w);
    for i in p..=r {
        v[i] = w[i];
    }
}

fn parallel_merge_aux_ints(
    v: &mut Vec<u64>,
    p1: i64,
    r1: i64,
    p2: i64,
    r2: i64,
    w: &mut Vec<u64>,
    p3: i64,
) {
    //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    let mut mr2 = r2;

    if p1 > r1 && p2 > r2 {
        //both subarrays are empty
        //println!("mr1={mr1}, mp1={mp1}, mr2={mr2}, mp2={mp2}");
        return;
    };
    //println!("input past subarrs empty trap: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");

    if (r1 - p1) < (r2 - p2) {
        //second subarr is smaller than the first.
        let t1 = mp1;
        mp1 = mp2;
        mp2 = t1;
        let t2 = mr1;
        mr1 = mr2;
        mr2 = t2;
    }

    let q1 = (mp1 + mr1) / 2; // midpoint of A[p1 : r1]

    let x = v[q1 as usize]; // median of A[p1 : r1] is pivot x

    //fix overflow issue carried into split point:
    if mr2 == -1 {
        mr2 = 0;
    }

    let q2 = find_split_point(v, mp2 as usize, mr2 as usize, x); // split A[p2 : r2] around x
    let q3 = p3 + (q1 - mp1) + (q2 as i64 - mp2); // where x belongs in B...
                                                  /*
                                                  if q3 == 10 {
                                                      //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
                                                      println!("error input: mp1={mp1}, mr1={mr1}, mp2={mp2}, mr2={mr2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
                                                      println!("{:?}", w);
                                                  } */
    w[q3 as usize] = x; // ... put it there

    //spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
    p_merge_aux_ints(v, mp1, q1 - 1, mp2, q2 as i64 - 1, w, p3);
    //spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
    p_merge_aux_ints(v, q1 + 1, mr1, q2 as i64, mr2, w, q3 + 1);

    /*
        thread::scope(|scope| {
            for e in &mut vec {
                scope.spawn(move || {
                    thread::sleep(Duration::from_secs(1));
                    *e += 1;
                });
            }
        });
    */

    /*
    //roughly follow this but can't use directly:
            let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(len / chunks) {
                //println!("this slice is: {:?}", slice);
                //scope.spawn(move |_| insertion_sort_arr(slice, 0, slice.len()-1));
                scope.spawn(move |_| slice.sort());
                //scope.spawn(move |_| println!("{:?}", slice));
            }
        });
    */
}

fn p_merge_aux_ints(
    v: &mut Vec<u64>,
    p1: i64,
    r1: i64,
    p2: i64,
    r2: i64,
    w: &mut Vec<u64>,
    p3: i64,
) {
    //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    let mut mr2 = r2;

    if p1 > r1 && p2 > r2 {
        //both subarrays are empty
        //println!("mr1={mr1}, mp1={mp1}, mr2={mr2}, mp2={mp2}");
        return;
    };
    //println!("input past subarrs empty trap: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}");

    if (r1 - p1) < (r2 - p2) {
        //second subarr is smaller than the first.
        let t1 = mp1;
        mp1 = mp2;
        mp2 = t1;
        let t2 = mr1;
        mr1 = mr2;
        mr2 = t2;
    }

    let q1 = (mp1 + mr1) / 2; // midpoint of A[p1 : r1]

    let x = v[q1 as usize]; // median of A[p1 : r1] is pivot x

    //fix overflow issue carried into split point:
    if mr2 == -1 {
        mr2 = 0;
    }

    let q2 = find_split_point(v, mp2 as usize, mr2 as usize, x); // split A[p2 : r2] around x
    let q3 = p3 + (q1 - mp1) + (q2 as i64 - mp2); // where x belongs in B...
                                                  /*
                                                  if q3 == 10 {
                                                      //println!("input: p1={p1}, r1={r1}, p2={p2}, r2={r2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
                                                      println!("error input: mp1={mp1}, mr1={mr1}, mp2={mp2}, mr2={mr2}, p3={p3}, q1={q1}, q2={q2}, q3={q3}");
                                                      println!("{:?}", w);
                                                  } */
    w[q3 as usize] = x; // ... put it there

    //spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
    p_merge_aux_ints(v, mp1, q1 - 1, mp2, q2 as i64 - 1, w, p3);
    //spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
    p_merge_aux_ints(v, q1 + 1, mr1, q2 as i64, mr2, w, q3 + 1);
}

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

fn p_merge_aux(
    v: &mut Vec<u64>,
    p1: usize,
    r1: usize,
    p2: usize,
    r2: usize,
    w: &mut Vec<u64>,
    p3: usize,
) {
    // bounds of each array mutable bc the first could be considered the second and vice versa - bigger is first.
    let mut mp1 = p1;
    let mut mp2 = p2;
    let mut mr1 = r1;
    let mut mr2 = r2;

    if p1 > r1 && p2 > r2 {
        //both subarrays are empty
        println!("mr1={mr1}, mp1={mp1}, mr2={mr2}, mp2={mp2}");
        return;
    };

    if (r1 < p1) || (r2 < p2) {
        //second subarr is smaller than the first.
        println!("isolating subtract with overflow: r1={r1}, p1={p1}, r2={r2}, p2={p2}");
        return;
    }
    if (r1 - p1) < (r2 - p2) {
        //second subarr is smaller than the first.
        let t1 = mp1;
        mp1 = mp2;
        mp2 = t1;
        let t2 = mr1;
        mr1 = mr2;
        mr2 = t2;
    }

    let q1 = (mp1 + mr1) / 2; // midpoint of A[p1 : r1]

    let x = v[q1]; // median of A[p1 : r1] is pivot x

    let q2 = find_split_point(v, mp2, mr2, x); // split A[p2 : r2] around x
    let q3 = p3 + (q1 - mp1) + (q2 - mp2); // where x belongs in B...
    w[q3] = x; // ... put it there

    // bc using usize, need a special case for underflow. if either q1 or q2 was zero, we'd have underflow.
    //  if BOTH were 0 so that -1 was sent as the r, we'd always have the base case and we'd return, so we'll do it here before we try to subtract and underflow.
    //  POTENTIAL issue though - we don't return unless both sides are smaller; here i'm only doing left recursion if neither side underflows. let's see what happens.
    if q1 == 0 || q2 == 0 || mr1 < mp1 || mr2 < mp2 {
        //if (r1 - p1) < (r2 - p2) { //second subarr is smaller than the first.
        println!("q1={q1}, q2={q2}, mr1={mr1}, mp1={mp1}, mr2={mr2}, mp2={mp2}");
    } else {
        //spawn P-MERGE-AUX(A; p1; q1 - 1; p2; q2 - 1; B; p3)
        p_merge_aux(v, mp1, q1 - 1, mp2, q2 - 1, w, p3);
    }
    //spawn P-MERGE-AUX(A; q1 + 1; r1; q2; r2; B; q3 + 1)
    //if mr1 < mp1 || mr2 < mp2 {
    p_merge_aux(v, q1 + 1, mr1, q2, mr2, w, q3 + 1);
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

    let mut aux = v.clone();
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
    fn test_find_split_point_small() {
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
            let b = find_split_point(&v, 0, v.len() - 1, v[i]);
            assert_eq!(b, i);
        }
    }

    #[test]
    fn test_p_merge_small() {
        let mut v = vec![1, 3, 5, 7, 9, 2, 4, 6, 8];

        println!("{:?}", v);

        let hi = v.len() - 1;
        //let mid = hi / 2;
        let mid = 4;
        //let mid = 5;
        p_merge(&mut v, 0, mid, hi);

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

        p_merge(&mut v, 0, mid, hi);

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
    fn test_merge_medium() {
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
    fn test_rayon_par_sort_baseline() {
        let n = 10_000_000;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        }

        //v.sort();

        //println!("{:?}", v);

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
    fn test_my_parallel_merge() {
        let n = 10_000_000;

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
    fn test_p_merge_parallel_merge() {
        //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3 ];
        //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3, 7, 2 ];

        let n = 1 << 25;

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
    fn test_experiment_scoped_threads() {
        experiment_scoped_threads();
    }

    #[test]
    fn test_baseline_sort() {
        let n = 1 << 25;

        let mut v = Vec::<u64>::new();
        for _i in 0..n {
            v.push(rand::thread_rng().gen_range(1..=50)); //=u64::MAX));
        }

        v.sort();

        assert!(is_sorted(&v));
    }
}
