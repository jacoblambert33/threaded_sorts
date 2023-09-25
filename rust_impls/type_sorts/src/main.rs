use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::time::SystemTime;

use sort_utils::*;

#[derive(Debug, Clone, Eq)]
struct User {
    //active: bool,
    //username: String,
    id: u64,
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn build_user(id: u64) -> User {
    //active: bool,
    //username: String,
    //id: u64,
    User {
        //active: true,
        //username: String::from("idcareyet"),
        id: id,
    }
}

fn create_tuples_reverse_order() -> Vec<(u8, u8, u8, u8)> {
    let mut capacity = 1 << 8; // start with 256, the range of u8 values.
    println!("capacity for one u8: {}", capacity);
    capacity <<= 8 * 2; //expand the capacity to hold a tuple of three u8 values - the limit on my machine. (four u8 values - to get all the combinations.

    println!(
        "expecting i can make {} unique tuples of four values of u8s",
        capacity
    );

    let mut v = Vec::<(u8, u8, u8, u8)>::with_capacity(capacity);

    let mut t = (0, 0, 0, 0);

    for a in (0..=255).rev() {
        t.0 = a;
        for b in (0..=255).rev() {
            t.1 = b;
            for c in (0..=255).rev() {
                t.2 = c;
                /*
                for d in (0..=255).rev() {
                    t.3 = d;
                    v.push(t);
                }
                */
                v.push(t);
            }
        }
        //println!("v has {} tuples.", v.len());
    }

    /*
    println!("v has {} tuples.", v.len());

    println!("first few tuples....");
    for i in 1_000_000..1_000_010 {
        println!("\t{:?}", v[i]);
    }
    */
    //v.sort();

    v
}

fn create_tuples_random_bytes() -> Vec<(u8, u8, u8, u8)> {
    let mut capacity = 1 << 8; // start with 256, the range of u8 values.
    println!("capacity for one u8: {}", capacity);
    capacity <<= 8 * 2; //expand the capacity to hold a tuple of three u8 values - the limit on my machine. (four u8 values - to get all the combinations.

    println!(
        "expecting i can make {} unique tuples of four values of u8s",
        capacity
    );

    let mut v = Vec::<(u8, u8, u8, u8)>::with_capacity(capacity);

    let mut t = (0, 0, 0, 0);

    //TODO: figure out how to do this to make random array initialization faster. not a priority, but convenient
    //let chunk_size = 10_000;
    //v.par_chunks_mut(chunk_size)
    //.for_each_init(|| rand::thread_rng(), |rng, chunk| rng.fill(chunk)); //doesn't work bc this is not an array it's a tuple. Fill trait is for arrays.
    //.for_each_init(|| rand::thread_rng(), |rng, chunk| chunk = rng.gen_range(1..=u8::MAX));

    for _ in 0..=capacity {
        t.0 = rand::thread_rng().gen_range(1..=u8::MAX);
        t.1 = rand::thread_rng().gen_range(1..=u8::MAX);
        t.2 = rand::thread_rng().gen_range(1..=u8::MAX);
        t.3 = rand::thread_rng().gen_range(1..=u8::MAX);
        v.push(t);
    }

    v
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn t_basic_tuple_construction() {
    let mut tup: (u8, u8, u8, u8) = (20, 40, 60, 80);

    tup.0 = 100;
    tup.1 = 255; //don't expect to create a u8 over 255.

    println!("you created this tuple: {:?}", tup);
}

#[test]
fn t_vec_tups_sys_sort() {
    let start = SystemTime::now();
    let mut v: Vec<(u8, u8, u8, u8)> = create_tuples_reverse_order();
    //let mut v : Vec::<(u8, u8, u8, u8)>  = create_tuples_random_bytes();

    sort_utils::end_and_print_time(start, "filled in values...");
    //pub fn time_fn_noargs(f: &dyn Fn()) {
    //println!("have a vector with # of tuples: {}", v.len());

    /*
    println!("first few tuples....");
    for i in 1_000_000..1_000_010 {
        println!("\t{:?}", v[i]);
    }
    */

    v.sort();

    sort_utils::end_and_print_time(start, "sorted...");

    assert!(sort_utils::is_sorted(&v));
}

#[test]
fn t_vec_tups_rayon_sort() {
    let start = SystemTime::now();
    let mut v: Vec<(u8, u8, u8, u8)> = create_tuples_reverse_order();
    //let mut v : Vec::<(u8, u8, u8, u8)>  = create_tuples_random_bytes();

    sort_utils::end_and_print_time(start, "filled in values...");
    //pub fn time_fn_noargs(f: &dyn Fn()) {
    //println!("have a vector with # of tuples: {}", v.len());

    /*
    println!("first few tuples....");
    for i in 1_000_000..1_000_010 {
        println!("\t{:?}", v[i]);
    }
    */

    v.par_sort();

    sort_utils::end_and_print_time(start, "sorted...");

    assert!(sort_utils::is_sorted(&v));
}

#[test]
fn t_rayon_par_sort_struct() {
    //let n = 1_000_000_000;
    //let n = 100_000_000;
    //let n = 10_000_000;
    //let n = 5_000_000;
    //let n = 1_000_000;
    let n = 10_000;
    //let n = 8;

    let start = SystemTime::now();
    println!("start....");
    let mut v = Vec::<User>::new();

    sort_utils::end_and_print_time(start, "allocated vector...");

    for _i in 0..n {
        let id: u64 = rand::thread_rng().gen_range(1..=u64::MAX);
        let u = build_user(id);
        v.push(u);
    }

    //println!("{:?}", v);

    sort_utils::end_and_print_time(start, "filled in values...");

    let mut w = v.clone();

    sort_utils::end_and_print_time(start, "cloned...");

    w.sort();

    sort_utils::end_and_print_time(start, "serial sort...");

    //assert!(is_sorted(&w));

    sort_utils::end_and_print_time(start, "confirm serial sort...");

    //assert!(!is_sorted(&v));

    sort_utils::end_and_print_time(start, "confirm paral. NOT sorted...");

    v.par_sort();

    sort_utils::end_and_print_time(start, "parallel sort...");

    //assert!(is_sorted(&v));

    sort_utils::end_and_print_time(start, "confirm parallel sort...");
}
