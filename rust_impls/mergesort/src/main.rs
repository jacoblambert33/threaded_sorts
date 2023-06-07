use sort_utils::*;

fn main() {
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
