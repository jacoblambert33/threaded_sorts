use std::{thread, time::Duration, time::SystemTime};

fn main() {
    println!("Hello, world!");
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

    let start = SystemTime::now();

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

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "[p_merge_parallel_merge] sorting little groups of size {unit_step} took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );


    //println!("step1\n{:?}\n", v);

    let mut len = unit_step * 2;
    //let mut start = 0;
    ////let mut end = combined;

    let start = SystemTime::now();

    while len <= power {
        //println!("len is {len}, power is {power}");
        let mut lo = 0;
        //while lo < n - len {
        //while lo < power - len {
        while lo < power - len + 1 {

            let _a = crossbeam::scope(|scope| {
                for slice in v.chunks_mut(len) {
                    //len {
                    let l = slice.len();
                    scope.spawn(move |_| sort_utils::merge_bu(slice, l)); //combined));
                }
            });

            lo = lo + len + len;
        }
        len = len * 2; // / 2;
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "[p_merge_parallel_merge] merging all the little groups in threads took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );



}





use rand::Rng;
 
#[test]
fn test_p_merge_parallel_merge() {
    //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3 ];
    //let mut v = vec![9, 7, 6, 3, 2, 4, 1, 5, 4, 3, 2, 1, 8, 3, 7, 2 ];

    let n = 1 << 24; //25;
    println!("array size: {n}");

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=50)); //=u64::MAX));
    }

    //println!("{:?}", v);

    assert!(!sort_utils::is_sorted(&v));

    let hi = v.len() - 1;

    let start = SystemTime::now();



    //p_merge_parallel_merge(&mut v, 0, hi, 4096); //22s for 1<<20
    //p_merge_parallel_merge(&mut v, 0, hi, 16384); //3.98s for 1<<20
    //p_merge_parallel_merge(&mut v, 0, hi, 262144); //1.3s for 1<<20
    //p_merge_parallel_merge(&mut v, 0, hi, 2097152); //58s for 1<<25
    p_merge_parallel_merge(&mut v, 0, hi, 4194304); //58s for 1<<25
    //p_merge_parallel_merge(&mut v, 0, hi, 8388608); //58s for 1<<25


    //p_merge_parallel_merge(&mut v, 0, hi, 8388608); //58s for 1<<25
    //p_merge_parallel_merge(&mut v, 0, hi, 16777216); //58s for 1<<25
                                                     /*
                                                     2097152
                                                     4194304
                                                     8388608
                                                     16777216
                                                     */
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "bottom up merge took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );


    //println!("{:?}", v);

    assert!(sort_utils::is_sorted(&v));
}

#[test]
fn test_baseline_sort() {

    let n = 1 << 24;//25;
    println!("array size: {n}");

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=50)); //=u64::MAX));
    }

    let start = SystemTime::now();

    v.sort();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "std vector sort took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );


    assert!(sort_utils::is_sorted(&v));
}


#[test]
fn test_just_little_sorts() {

    let n = 1 << 24;//25;
    println!("array size: {n}");

    let mut v = Vec::<u64>::new();
    for _i in 0..n {
        v.push(rand::thread_rng().gen_range(1..=50)); //=u64::MAX));
    }

        let hi = v.len() - 1;

    let start = SystemTime::now();


        //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 256);
       // sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 16384);
/*
32768
65536
131072
262144
524288
1048576
*/

    sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 16384); //works but slow
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 32768); //too big
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 65536); // too big
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 131072); //

    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 262144); //
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 2097152); //
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 8388608); ////too big on stack(?)
    //sort_utils::p_merge_sorted_groups(&mut v, 0, hi, 16777216); //too big on stack(?)
 

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "little sorted groups took {}.{} seconds",
        duration.as_millis() / 1000,
        duration.as_millis() % 1000
    );


    assert!(sort_utils::is_sorted(&v));
}
