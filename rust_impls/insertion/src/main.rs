use rand::Rng;
use std::time::SystemTime;

use sort_utils::*;

fn main() {
    let n = 100_000_000; //can't do 100 mil on my home ubuntu setup

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=u64::MAX));
        //v.push(rand::thread_rng().gen_range(1..=20));
    }

    //println!("{:?}", v);

    assert!(!is_sorted(&v));

    let hi = v.len() - 1;

    let start = SystemTime::now();

    //v.sort();  //14.4s for 100 mil
    //insertion_sort(&mut v, 0, hi);
    //p_merge_sorted_groups(&mut v, 0, hi, 32); //19s
    //p_merge_sorted_groups(&mut v, 0, hi, 64); //4.xs -- best for 100mil records (??)
    p_merge_sorted_groups(&mut v, 0, hi, 128); //4.xs
                                               //p_merge_sorted_groups(&mut v, 0, hi, 256); //4.xs
                                               //p_merge_sorted_groups(&mut v, 0, hi, 512); //0.844s
                                               //p_merge_sorted_groups(&mut v, 0, hi, 1024); //1.9s
                                               //p_merge_sorted_groups(&mut v, 0, hi, 2048); //1.3s   16.x
                                               //p_merge_sorted_groups(&mut v, 0, hi, 4096); //1.3s   16.x
                                               //p_merge_sorted_groups(&mut v, 0, hi, 8192); //1.3s
                                               //p_merge_sorted_groups(&mut v, 0, hi, 16384); //1.3s
                                               //p_merge_sorted_groups(&mut v, 0, hi, 32768); // stack overflow

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

    //println!("{:?}", v);

    assert!(is_sorted(&v));

    //println!("first elements:\n{:?}", &v[0..1000]);
    //let b = n - 1000;
    //println!("last elements:\n{:?}", &v[b..n]);
}

/*
use std::time::SystemTime;

fn insertion_sort(v: &mut Vec<u64>, lo:usize, hi:usize) {

    //let mut vv = v;

    //println!("outer range inclusive should be {} to {}", lo+1, hi);
    for i in lo+1..=hi {
        //println!("i is {i}");
        //println!("inner range inclusive should be {} to {}", i, lo+1);
        for j in (lo+1..=i).rev() {
            //println!("\tj is {j}");
            if !less(v, j, j-1) {
                break;
            }
            exch(v, j, j-1);
        }
    }

    assert!(is_sorted(&v));
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
fn is_sorted(v: &Vec<u64>) -> bool {

    for i in 1..v.len() {
        if &v[i] < &v[i-1] {
            return false;
        }
    }
    true
}


/// exch modifies the vector directly and returns nothing.
fn exch(v: &mut Vec<u64>, i:usize, j:usize) {

    let &t: &u64 = &v[i];
    v[i] = v[j];
    v[j] = t;

}

/// less is readonly - we need references; not changing the array but reading it.
fn less(v: &Vec<u64>, i:usize, j:usize) -> bool {

        let first: &u64 = &v[i];
        let second: &u64 = &v[j];

    match first.cmp(second) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => false,
    }


}

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
fn test_insertion_small() {

    let mut v = vec![4, 3, 2, 1];

    println!("{:?}", v);

    assert!(!is_sorted(&v));

    let hi = v.len()-1;
    insertion_sort(&mut v, 0, hi);

    println!("{:?}", v);

    assert!(is_sorted(&v));


}

use rand::Rng;

#[test]
fn test_insertion_medium() {

    let n = 10_000;

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push( rand::thread_rng().gen_range(1..=u64::MAX));
    }


    //println!("{:?}", v);

    assert!(!is_sorted(&v));

    let hi = v.len()-1;


    let start = SystemTime::now();

    insertion_sort(&mut v, 0, hi);

  let end = SystemTime::now();
  let duration = end.duration_since(start).unwrap();
  //println!("it took {} seconds", duration.as_nanos());
  //println!("it took {} seconds", duration.as_micros());
  println!("it took {}.{} seconds", duration.as_millis()/1000, duration.as_millis()%1000);

    let lo = v[0];
    let hi = v[n-1];

    println!("sorted with lowest element {lo} and highest {hi}");

    //println!("{:?}", v);

    assert!(is_sorted(&v));


}
*/
