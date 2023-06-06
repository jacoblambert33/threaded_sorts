
use sort_utils::*;


fn main() {

    println!("Hello, world!");
}


fn mergesort(v: &mut Vec<u64>, lo:usize, hi:usize, cutoff:usize) {

  let mid = lo + (hi - lo) / 2;


  if hi <= lo {// + cutoff) {
    //insertion_sort(v);
		return; 
  }  

	println!("lo is {lo}, mid is {mid}, and hi is {hi}");

	mergesort(v, lo, mid, cutoff);  
	mergesort(v, mid+1, hi, cutoff);  

	merge(v, lo, mid, hi); 

}



fn merge(a: &mut Vec<u64>, lo:usize, mid:usize, hi:usize) {

  // precondition: a[lo .. mid] and a[mid+1 .. hi] are sorted subarrays
	//println!("{:?}", &a[lo..mid]);
	//println!("{:?}", &a[mid..=hi]);
	//assert!(is_sorted_slice(&a[lo..mid]));
	//assert!(is_sorted_slice(&a[mid..=hi]));


  // copy to aux[]
	let aux = a.clone();

  // merge back to a[]
  let mut i = lo; 
	let mut j = mid;  // j = mid+1;
  // for (int k = lo; k <= hi; k++) {
	for k in lo..=hi {
		//println!("k is {k}"); 
    if i >= mid {
      a[k] = aux[j];
			j = j + 1; 
		}
    else if j > hi {
      a[k] = aux[i];
			i = i + 1; 
		}
    else if less(&aux, j, i) {
			//println!("k is {k}"); 
      a[k] = aux[j];
			j = j + 1; 
		}
    else {
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

	merge(&mut v,  0, 3, 5); 
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

	let hi = v.len()-1;
	let m = hi / 2;

	mergesort(&mut v,  0, m, hi); 

	println!("after sort: {:?}", &v);
	assert!(is_sorted(&v));
	
}
use rand::Rng;

#[test]
fn test_merge_medium() {


		let n = 1_000_000; 

		let mut v = Vec::<u64>::new();
		let mut w = Vec::<u64>::new();
		for _i in 0..n {
			v.push( rand::thread_rng().gen_range(1..=u64::MAX));
		}
		for _i in 0..n {
			w.push( rand::thread_rng().gen_range(1..=u64::MAX));
		}

		v.sort();
		w.sort();

		v.append(&mut w);

		let m = v.len() / 2; 
		let hi = v.len() - 1;

	merge(&mut v,  0, m, hi); 
	assert!(is_sorted(&v));
	
}
