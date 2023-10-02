use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::time::SystemTime;
//use std::cell::RefCell;
use std::thread;

//use sort_utils::*;

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


fn create_tuple_buckets_reverse_order() -> Vec<(u8, Vec<(u8, u8, u8)>)> {
//fn create_tuple_buckets_reverse_order() -> [Vec<(u8, u8, u8)>; 256] {
//fn create_tuple_buckets_reverse_order() -> [[(u8, u8, u8); 65536]; 256] {
//fn create_tuple_buckets_reverse_order() -> Box<[[(u8, u8, u8); 65536]; 256]> {
//fn create_tuple_buckets_reverse_order() -> Box<[Box<[(u8, u8, u8); 65536]>; 256]> {
//the trait `Copy` is not implemented for `Box<[({integer}, {integer}, {integer}); 65536]>`
// same error as for the vector, which gives more flexibility - or at least more sugar. perhaps boxing an array is close enough to a vector that i shouldn't have expected to escape with a Box. 
    let mut capacity = 1 << 8; // start with 256, the range of u8 values.
    println!("capacity for one u8: {}", capacity);
    capacity <<= 8 * 2; //expand the capacity to hold a tuple of three u8 values - the limit on my machine. (four u8 values - to get all the combinations.

    println!(
        "expecting i can make {} FINISH THIS STATEMENT",
        capacity
    );

		let mut v = Vec::<(u8, Vec::<(u8, u8, u8)>)>::with_capacity(capacity);
		//let mut v =  [Vec::<(u8, u8, u8)>::with_capacity(capacity); 256];
		//let mut v =  Box::new([[(0, 0, 0); 65536]; 256]);
		//let mut v =  Box::new([Box::new([(0, 0, 0); 65536]); 256]);

    //let mut t = (0, 0, 0, 0);
    //let mut t: u8; 

    //for a in (0..=255).rev() {
    for a in 0..=255 { // don't rverse this one, bc in this scheme it's the index. 
        //t = a;
				let mut u : Vec::<(u8, u8, u8)> = Vec::<(u8, u8, u8)>::with_capacity(capacity);
				//let mut u = Box::new([(0, 0, 0); 65536]);
				let mut x : (u8, u8, u8) = (0, 0, 0); 
				
        for b in (0..=255).rev() {
            x.0 = b;
            for c in (0..=255).rev() {
                x.1 = c;
								/*
                for d in (0..=255).rev() {
								  //not enough ram on my VM. 
									u.push(x);
                }
								*/
                u.push(x);
                //u[((b*255)+c) as usize] = x; 
            }
        }
        //println!("v has {} tuples.", v.len());
			v.push((a, u)); 		
			//v[a] = u; 		
    }

    v
}

//fn create_array_three_tuples_reverse_order() -> [Vec<(u8, u8, u8)>; 256] {
//fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 *256]; 256] {
//fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 *256 * 256]; 256] {
//fn create_array_three_tuples_reverse_order() -> Box<[[(u8, u8, u8); 256 *256 * 256]; 256]> { //this what i want, but it requires 12 GB. so i'll test with half
fn create_array_three_tuples_reverse_order() -> Box<[[(u8, u8, u8); 256 *256 * 256]; 128]> {
    //capacity <<= 8 * 2; //expand the capacity to hold a tuple of three u8 values - the limit on my machine. (four u8 values - to get all the combinations.


    //let mut v = [Vec::<(u8, u8, u8)>::with_capacity(capacity); 256 ]; trait copy not impl. 
    //let mut v = [Vec::<(u8, u8, u8)>::new(); 256 ]; 
    //let mut v = [[(0, 0, 0); 256 * 256] ; 256 ]; 
    //let mut v = [[(0, 0, 0); 256 * 256 * 256] ; 256 ]; 
    //let mut v = Box::new([[(0, 0, 0); 256 * 256 * 256] ; 256 ]); // this what i want. 
    let mut v = Box::new([[(0, 0, 0); 256 * 256 * 256] ; 128 ]); //this hopefully feasible. 


    //for a in (0..=255).rev() {
    //for a in 0..=255 { this is right. half might be feasible on my VM. 
    for a in 0..128 {
				//let mut tup_vec = Vec::<(u8, u8, u8)>::with_capacity(capacity); 
				let mut each_tup = (0, 0, 0);
				let mut i = 0; 
        for b in (0..=255).rev() {
            each_tup.0 = b;
            for c in (0..=255).rev() {
                each_tup.1 = c;
                for d in (0..=255).rev() {
                    each_tup.2 = d;
                    //tup_vec.push(each_tup);
										// need something here...
										v[a][i] = each_tup; 
										i = i + 1; 
                }
            }
        }
        //println!("v has {} tuples.", v.len());
				//v[a] = tup_vec; 
			//v[a].sort()
			v[a].par_sort()
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


fn create_array_four_tuples_reverse_order() -> Box<[(u8, u8, u8, u8); 256*256*256*48]> {

    let mut v = Box::new([(0, 0, 0, 0); 256 * 256 * 256 * 48 ]); //this hopefully feasible. 

		let mut each_tup = (0, 0, 0, 0);
		let mut i = 0; 
    for a in 0..48 {
				each_tup.0 = a;
        for b in (0..=255).rev() {
            each_tup.1 = b;
            for c in (0..=255).rev() {
                each_tup.2 = c;
                for d in (0..=255).rev() {
                    each_tup.3 = d;
										v[i] = each_tup; 
										i = i + 1; 
                }
            }
        }
    }

    v
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

    let start = SystemTime::now();
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
    println!("some middling few tuples....");
    for i in 1_000_000..1_000_010 {
        println!("\t{:?}", v[i]);
    }
		*/

    let start = SystemTime::now();
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

#[test]
fn t_bucket_tuples() {
    let start = SystemTime::now();
    let mut v: Vec<(u8, u8, u8, u8)> = create_tuples_reverse_order();
    //let mut v : Vec::<(u8, u8, u8, u8)>  = create_tuples_random_bytes();

    sort_utils::end_and_print_time(start, "filled in values...");
    //pub fn time_fn_noargs(f: &dyn Fn()) {
    //println!("have a vector with # of tuples: {}", v.len());

		/*
    println!("some middling few tuples....");
    for i in 1_000_000..1_000_010 {
        println!("\t{:?}", v[i]);
    }
		*/

    let start = SystemTime::now();

    v.par_sort();

    sort_utils::end_and_print_time(start, "sorted...");

    assert!(sort_utils::is_sorted(&v));
}

#[test]
fn t_create_tuple_buckets() {

	let x =  create_tuple_buckets_reverse_order(); 
	 
	println!("x should have 256 buckets of three tuples."); 
	assert_eq!(x.len(), 256); 
	println!("for each first tuple of x, there should be 65536 tuples in its vector (because i can only fit two elements in my tuples, not three as i hoped. "); 
	//this is NOT ideal, but a step in the right direction. it's should be a minimal expense to filter for the tuple i want, but that's something else to measure. (it would be far better to index into an array (conceptually), but 256 operators are nothing consequential...unless i'm doing something expensive that many times. 

	for i in 0..256 {
		//assert_eq!(x.0jj
		//let the_one = x.filter(|i| x.0 == i);
		let the_one = &x[i];
		assert_eq!(the_one.1.len(), 256*256);
		println!("{} has {} elements", the_one.0, the_one.1.len());
	}

	for i in 0..10 {
		println!("x[{}] has elements: {:?}", 0, x[0].1[i]);	
	}
	
}



fn sort_buckets(buckets: &mut Vec<(u8, Vec<(u8, u8, u8)>)> ) {
//fn create_tuple_buckets_reverse_order() -> Vec<(u8, Vec<(u8, u8, u8)>)> {
/*
	for i in 0..256 {
		let y = &mut x[i]; 
		//y.1.sort(); //sort the vector that is the second element of the tuple. 
		y.1.par_sort(); //sort the vector that is the second element of the tuple. 
	}
	*/

	/*
	(0..256).into_par_iter() //(|| { let y = &mut x[i]; y.1.sort(); });
		//.for_each(|i| { let mut y = RefCell::new(x[i]); y.into_inner().1.sort(); });
		.for_each(|i| { let mut y = RefCell::new(x[i]); *y.get_mut().1.sort(); });
		//.for_each(|i| x[i].1.sort() );
	*/

/*
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
*/ 

    thread::scope(|scope| {
        //for slice in v.chunks_mut(run_size) {
				
        for each in buckets {
            scope.spawn(move || {
                // requires marker Send -  T` cannot be sent between threads safely
                each.1.sort(); //requires the trait Ord.
            });
        }
    });





}

#[test] 
fn t_sort_buckets() {

	let start = SystemTime::now();
	let mut x =  create_tuple_buckets_reverse_order(); 
	sort_utils::end_and_print_time(start, "time to build tuples...");

	let start = SystemTime::now();
	sort_buckets(&mut x); 
	sort_utils::end_and_print_time(start, "each bucket sorted...");
 
	for i in 0..256 {
		let y = &mut x[i]; 
    assert!(sort_utils::is_sorted(&y.1));
	}
	
}

fn _sort_array_buckets(buckets: &mut [(u8, u8, u8); 256 *256]) {

//fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 *256]; 256] {
    thread::scope(|scope| {
        //for slice in v.chunks_mut(run_size) {
				
        for each in buckets {
            scope.spawn(move || {
                // requires marker Send -  T` cannot be sent between threads safely
                //each.sort(); //requires the trait Ord.
            });
        }
    });





}


#[test]
fn t_create_arr_tuples() {

	let start = SystemTime::now();
//fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 *256]; 256] {
	let mut x = create_array_three_tuples_reverse_order(); 
	sort_utils::end_and_print_time(start, &format!("created {} arrays each with an array sized {}", x.len(), x[0].len()) );
	
	println!("the array has {} elements.", x.len()); 

	let start = SystemTime::now();
/* //i don't need this if i sort on creation....
 //correct sorting. serial and slow. 
	for i in 0..x.len() {
		//println!("\tthe inner array has {} elements.", x[i].len()); 
		assert_eq!(x[i].len(), 256*256*256);
		x[i].sort();
	}
*/

/* // i'm stuck here for now. 
    //thread::spawn(|| {
    thread::spawn(move|| {
        //for i in 1..x.len() {
        for i in 1..128 {
					x[i].sort();		
        }
    });
*/ 

/*
	 thread::scope(|scope| {
        //for slice in v.chunks_mut(run_size) {

			//for each in x.iter() {
			//for mut Box::new(each) in x.iter() {
			//for Box::new(mut each) in x.iter() {
			//for mut each in x.into_iter() { //overflows stack
			for Box::new(mut each) in x.into_iter() {
			//for y in 0..x.len() {
            scope.spawn(move || {
                each.sort(); 
            });
        }
    });
*/


	sort_utils::end_and_print_time(start, "sorted each array");

	for i in 0..128 {
		//let y = Vec::new(x[i]); 
		let z = vec!(x[i]); 
    assert!(sort_utils::is_sorted(&z));
	}
/*
created 128 arrays each with an array sized 16777216 10.279 seconds
the array has 128 elements.
               sorted each array 6.942 seconds
*/



}

#[test]
fn t_create_arr_all_tuples() {

	let start = SystemTime::now();
//fn create_array_four_tuples_reverse_order() -> Box<[[(u8, u8, u8, u8); 256*256*256*128]> {
	let mut x = create_array_four_tuples_reverse_order(); 
	sort_utils::end_and_print_time(start, &format!("created {} arrays.", x.len()) );
	
	println!("the array has {} elements.", x.len()); 

	let start = SystemTime::now();
	//x.sort();
	x.par_sort();
	sort_utils::end_and_print_time(start, "sorted each array");

}



