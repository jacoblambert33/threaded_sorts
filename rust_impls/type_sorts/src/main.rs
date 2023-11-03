use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::time::SystemTime;
//use std::cell::RefCell;
use std::thread;

#[derive(Clone, Copy)]
struct WrapperTwoStageOneLayer(*mut Vec<[u8; 2]>);

unsafe impl Send for WrapperTwoStageOneLayer {}
unsafe impl Sync for WrapperTwoStageOneLayer {}

#[derive(Clone, Copy)]
struct WrapperTwoStage(*mut Vec<Vec<[u8; 2]>>);

unsafe impl Send for WrapperTwoStage {}
unsafe impl Sync for WrapperTwoStage {}

#[derive(Clone, Copy)]
struct WrapperOneArr2(*mut Vec<[u8; 2]>);

unsafe impl Send for WrapperOneArr2 {}
unsafe impl Sync for WrapperOneArr2 {}

#[derive(Clone, Copy)]
struct WrapperOneArr(*mut Vec<[u8; 4]>);

unsafe impl Send for WrapperOneArr {}
unsafe impl Sync for WrapperOneArr {}

#[derive(Clone, Copy)]
//struct WrapperArrsExt(*mut Vec<Vec<Vec<Vec<bool>>>>);
struct WrapperArrsExt(*mut Vec<Vec<Vec<[bool; 256]>>>);

unsafe impl Send for WrapperArrsExt {}
unsafe impl Sync for WrapperArrsExt {}

#[derive(Clone, Copy)]
struct WrapperBag2566(*mut Vec<Vec<[u8; 6]>>);

unsafe impl Send for WrapperBag2566 {}
unsafe impl Sync for WrapperBag2566 {}

#[derive(Clone, Copy)]
struct WrapperBag2565(*mut Vec<Vec<[u8; 5]>>);

unsafe impl Send for WrapperBag2565 {}
unsafe impl Sync for WrapperBag2565 {}

#[derive(Clone, Copy)]
struct WrapperBag2562(*mut Vec<Vec<[u8; 2]>>);

unsafe impl Send for WrapperBag2562 {}
unsafe impl Sync for WrapperBag2562 {}

#[derive(Clone, Copy)]
struct WrapperBag25622(*mut Vec<Vec<Vec<[u8; 2]>>>);

unsafe impl Send for WrapperBag25622 {}
unsafe impl Sync for WrapperBag25622 {}

#[derive(Clone, Copy)]
struct WrapperBagExt(*mut Vec<Vec<Vec<[u8; 6]>>>);

unsafe impl Send for WrapperBagExt {}
unsafe impl Sync for WrapperBagExt {}

#[derive(Clone, Copy)]
struct WrapperBag(*mut Vec<Vec<Vec<[u8; 2]>>>);

unsafe impl Send for WrapperBag {}
unsafe impl Sync for WrapperBag {}

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

//e.g., static COUNTER: AtomicUsize = AtomicUsize::new(0);
static R: u8 = 31;
static M: u8 = 255;
fn modular_hash(input: [u8; 8]) -> u8 {
    //fix R at 31. fix M as 251 for the set. numbers prime. close enough to size of set.
    let mut hash: u8 = 0;
    for i in 0..8 {
        hash = (R * hash + input[i]) % M;
    }
    hash
}

/*
fn modular_hash(r: u8, m: u32, input: [u8; 8]) -> u8 {
    //fix R at 31. fix M as 257 for the set and adjust here. is prime number near the size of the set.
    let mut hash : u32 = 0;
    for i in 0..input.len() {
    hash = u32::from(u32::from(r) * hash + u32::from(input[i])) % m;
        if hash > 255 {
            hash = u32::from(modular_hash(31, 257, input));
        }
    }
    //u8::from(hash)
    hash as u8
}
*/

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

    println!("expecting i can make {} FINISH THIS STATEMENT", capacity);

    let mut v = Vec::<(u8, Vec<(u8, u8, u8)>)>::with_capacity(capacity);
    //let mut v =  [Vec::<(u8, u8, u8)>::with_capacity(capacity); 256];
    //let mut v =  Box::new([[(0, 0, 0); 65536]; 256]);
    //let mut v =  Box::new([Box::new([(0, 0, 0); 65536]); 256]);

    //let mut t = (0, 0, 0, 0);
    //let mut t: u8;

    //for a in (0..=255).rev() {
    for a in 0..=255 {
        // don't rverse this one, bc in this scheme it's the index.
        //t = a;
        let mut u: Vec<(u8, u8, u8)> = Vec::<(u8, u8, u8)>::with_capacity(capacity);
        //let mut u = Box::new([(0, 0, 0); 65536]);
        let mut x: (u8, u8, u8) = (0, 0, 0);

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
//i'm able to creat so many more tuples as an array of arrays than as one array of tuples.
fn _create_array_three_tuples_maxvm_reverse_order() -> Box<[[(u8, u8, u8); 256 * 256 * 256]; 8]> {
    //capacity <<= 8 * 2; //expand the capacity to hold a tuple of three u8 values - the limit on my machine. (four u8 values - to get all the combinations.

    //let mut v = [Vec::<(u8, u8, u8)>::with_capacity(capacity); 256 ]; trait copy not impl.
    //let mut v = [Vec::<(u8, u8, u8)>::new(); 256 ];
    //let mut v = [[(0, 0, 0); 256 * 256] ; 256 ];
    //let mut v = [[(0, 0, 0); 256 * 256 * 256] ; 256 ];
    //let mut v = Box::new([[(0, 0, 0); 256 * 256 * 256] ; 256 ]); // this what i want.
    let mut v = Box::new([[(0, 0, 0); 256 * 256 * 256]; 8]); //this hopefully feasible.

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
        //how much faster without a sort at creation?
        //created 128 arrays each with an array sized 16777216 10.623 seconds
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

fn create_array_three_tuples_sameszonearray_reverse_order(
) -> Box<[[(u8, u8, u8); 256 * 256 * 256]; 48]> {
    let mut v = Box::new([[(0, 0, 0); 256 * 256 * 256]; 48]);

    for a in 0..48 {
        let mut each_tup = (0, 0, 0);
        let mut i = 0;
        for b in (0..=255).rev() {
            each_tup.0 = b;
            for c in (0..=255).rev() {
                each_tup.1 = c;
                for d in (0..=255).rev() {
                    each_tup.2 = d;
                    v[a][i] = each_tup;
                    i = i + 1;
                }
            }
        }
        //sort in creation.
        v[a].par_sort()
    }

    v
}
/*
// create the biggest array of 2,3,4,or 5 elements that i can on my VM.
//  experimental - might not actually be the upper limit of largest, but get a sense for that.
//fn create_array_four_reverse_order() -> [[u8;4]; 256*256*256*48] {
//fn create_array_four_large() -> Box<[[u8;4]; 256*256*256*48]> {
//fn create_array_four_large() -> Box<[Box<[u8;4]>; 256*256*256*48]> {
//fn create_array_four_large() -> Box<[Vec<u8>; 256*256*256*48]> {

    //let mut arr = Box::new([[0; 4]; 256*256*256*48 ]); //this hopefully feasible.
    //let mut arr = Box::new([vec!([0; 4]); 256*256*256*48 ]); //this hopefully feasible.
    //let mut arr = Box::new(Vec::new(); 256*256*256*48 ); //this hopefully feasible.
    let mut arr = Box::new([Box::new([0; 4]); 256*256*256*48 ]); //this hopefully feasible.

        let mut i = 0;
    for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        //println!("a {} b {} c {} d {}", a, b, c, d);
                        //arr[i] = Box::new([a, b, c, d].clone());
                        arr[i] = [a, b, c, d].clone();
                        //arr[i] = BoxedArray { arr : [a, b, c, d] };
                        i = i + 1;
                    }
                }
            }
        }

        //arr.sort();
        //Box::new(arr)  //wrong place to Box.
//		arr
}
*/
fn create_vecvecvecarr() -> Vec<Vec<Vec<[u8; 2]>>> {
    let mut v = vec![vec![vec![[0; 2]; 8]; 4]; 4];
    v[3][2][1] = [0xf, 0xa];
    v
}

fn create_twostep(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 2]>> {
    let mut v = vec![vec![[0 as u8; 2]; sz_l2 * sz_l3 * sz_l4]; sz_l1];
    v
}

fn create_twoonelayer(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<[u8; 2]> {
    let mut v = vec![[0 as u8; 2]; sz_l2 * sz_l3 * sz_l4 * sz_l1];
    v
}

fn create_2566(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 6]>> {
    let mut v = vec![vec![[0 as u8; 6]; sz_l1]; sz_l3 * sz_l4 * sz_l1];
    v
}

fn create_2565(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 5]>> {
    let mut v = vec![vec![[0 as u8; 5]; sz_l1]; sz_l3 * sz_l4 * sz_l1];
    v
}

fn create_2564(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 4]>> {
    let mut v = vec![vec![[0 as u8; 4]; sz_l1]; sz_l3 * sz_l4 * sz_l1];
    v
}

fn create_2563(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 3]>> {
    //let mut v = vec![vec![[0 as u8; 3]; sz_l1];  sz_l3 * sz_l4 * sz_l1 ];
    let mut v = vec![vec![[0 as u8; 3]; sz_l2 * sz_l3 * sz_l4]; sz_l1]; // 256 bags of sz 16 mil.
    v
}

fn create_2562(sz_l1: usize, sz_l2: usize, sz_l3: usize, sz_l4: usize) -> Vec<Vec<[u8; 2]>> {
    //let mut v = vec![vec![[0 as u8; 2]; sz_l1];  sz_l3 * sz_l4 * sz_l1 ]; //16 mil bags of sz 256
    let mut v = vec![vec![[0 as u8; 2]; sz_l2 * sz_l3 * sz_l4]; sz_l1]; // 256 bags of sz 16 mil.
    v
}
// space for random variables
fn create_25622(a: usize, b: usize, c: usize, d: usize) -> Vec<Vec<Vec<[u8; 2]>>> {
    let v = vec![vec![vec![[0; 2]; a * b]; c]; d];
    v
}

//fn create_largevec(a: usize) -> Vec<[u8; a]> { error: this would need to be a constant
//fn create_largevec() -> Vec<[[u8; 4]; 256 * 256 * 256 * 256]> {
fn create_largevec() -> Vec<[u8; 4]> {
    //let v = vec![[0 as u8; 4]; 256 * 256 * 256 * 256]; //memory allocation failed
    let v = vec![[0 as u8; 4]; 128 * 256 * 256 * 256];
    v
}

// space for random variables
fn create_vecvecvecarr_variable(a: usize, b: usize, c: usize) -> Vec<Vec<Vec<[u8; 2]>>> {
    let v = vec![vec![vec![[0; 2]; a]; b]; c];
    v
}

// store the indices i write from for correctness checks/consistency/traceability. see the effects it produces on memory, sort, and search.
fn create_vecvecvecarrlong_variable(a: usize, b: usize, c: usize) -> Vec<Vec<Vec<[u8; 6]>>> {
    let v = vec![vec![vec![[0; 6]; a]; b]; c];
    v
}

//fn create_vecvecvecvec(a: usize, b: usize, c: usize, d: usize) -> Vec<Vec<Vec<Vec<bool>>>> {
fn create_vecvecvecvec(b: usize, c: usize, d: usize) -> Vec<Vec<Vec<[bool; 256]>>> {
    //let v = vec![vec![vec![vec![false; a]; b]; c]; d];
    let v = vec![vec![vec![[false; 256]; b]; c]; d];
    v
}

fn _create_vecvecvectuplearrs_variable(
    a: usize,
    b: usize,
    c: usize,
) -> Vec<Vec<Vec<([u8; 2], [u8; 4])>>> {
    let v = vec![vec![vec![([0; 2], [0; 4]); a]; b]; c];
    v
}

fn _create_vecvecvecarrarrs_variable(
    a: usize,
    b: usize,
    c: usize,
) -> Vec<Vec<Vec<Vec<([u8; 2], [u8; 4])>>>> {
    let v = vec![vec![vec![vec![([0; 2], [0; 4]); 2]; a]; b]; c];
    v
}

fn _create_little_array_four_prototype() -> Box<[[u8; 4]; 4 * 4 * 4 * 4]> {
    let mut arr = Box::new([[0; 4]; 4 * 4 * 4 * 4]); //this hopefully feasible.

    let mut i = 0;
    for a in 0..4 {
        //arr[i][0] = a;
        //for b in (0..4).rev() {
        for b in 0..4 {
            //arr[i][1] = b;
            //for c in (0..4).rev() {
            for c in 0..4 {
                //arr[i][2] = c;
                //for d in (0..8).rev() {
                for d in 0..4 {
                    //arr[i][3] = d;
                    println!("a {} b {} c {} d {}", a, b, c, d);
                    arr[i] = [a, b, c, d];
                    i = i + 1;
                }
            }
        }
    }

    //arr.sort();
    //Box::new(arr)  //wrong place to Box.
    arr
}

fn _create_array_four_reverse_order() -> Box<[[u8; 4]; 256 * 256 * 256 * 48]> {
    //fn create_array_four_reverse_order() -> Box<[Box<[u8;4]>; 256*256*256*48]> {

    //let mut arr = [[0; 4]; 256 * 256 * 256 * 48 ]; //too big for stack.
    //let mut arr = Box::new([Box::new([0; 4]); 256 * 256 * 256 * 48 ]); //this hopefully feasible.
    let mut arr = Box::new([[0; 4]; 256 * 256 * 256 * 48]); //this hopefully feasible.

    let mut i = 0;
    for a in 0..48 {
        arr[i][0] = a;
        for b in (0..=255).rev() {
            arr[i][1] = b;
            for c in (0..=255).rev() {
                arr[i][2] = c;
                for d in (0..=255).rev() {
                    arr[i][3] = d;
                    i = i + 1;
                }
            }
        }
    }

    arr.par_sort();
    //Box::new(arr)  //wrong place to Box.
    arr
}

fn _create_array_four_tuples_reverse_order() -> Box<[(u8, u8, u8, u8); 256 * 256 * 256 * 1]> {
    let mut v = Box::new([(0, 0, 0, 0); 256 * 256 * 256 * 1]); //this hopefully feasible.

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

    v.par_sort();
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

fn _create_tuples_random_bytes() -> Vec<(u8, u8, u8, u8)> {
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

    let mut rng = ChaCha8Rng::from_entropy(); //works

    for _ in 0..=capacity {
        t.0 = rng.gen_range(1..=u8::MAX);
        t.1 = rng.gen_range(1..=u8::MAX);
        t.2 = rng.gen_range(1..=u8::MAX);
        t.3 = rng.gen_range(1..=u8::MAX);
        v.push(t);
    }

    v
}

fn create_one_tuple_random_bytes() -> (u8, u8, u8, u8) {
    let mut t = (0, 0, 0, 0);

    t.0 = rand::thread_rng().gen_range(1..=u8::MAX);
    t.1 = rand::thread_rng().gen_range(1..=u8::MAX);
    t.2 = rand::thread_rng().gen_range(1..=u8::MAX);
    t.3 = rand::thread_rng().gen_range(1..=u8::MAX);

    t
}

fn main() {
    println!("Hello, world!");
    //t_search_random_tuple_in_many_arrays();
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
    let x = create_tuple_buckets_reverse_order();

    println!("x should have 256 buckets of three tuples.");
    assert_eq!(x.len(), 256);
    println!("for each first tuple of x, there should be 65536 tuples in its vector (because i can only fit two elements in my tuples, not three as i hoped. ");
    //this is NOT ideal, but a step in the right direction. it's should be a minimal expense to filter for the tuple i want, but that's something else to measure. (it would be far better to index into an array (conceptually), but 256 operators are nothing consequential...unless i'm doing something expensive that many times.

    for i in 0..256 {
        //assert_eq!(x.0jj
        //let the_one = x.filter(|i| x.0 == i);
        let the_one = &x[i];
        assert_eq!(the_one.1.len(), 256 * 256);
        println!("{} has {} elements", the_one.0, the_one.1.len());
    }

    for i in 0..10 {
        println!("x[{}] has elements: {:?}", 0, x[0].1[i]);
    }
}

fn _sort_buckets(buckets: &mut Vec<(u8, Vec<(u8, u8, u8)>)>) {
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
    let mut x = create_tuple_buckets_reverse_order();
    sort_utils::end_and_print_time(start, "time to build tuples...");

    let start = SystemTime::now();
    _sort_buckets(&mut x);
    sort_utils::end_and_print_time(start, "each bucket sorted...");

    for i in 0..256 {
        let y = &mut x[i];
        assert!(sort_utils::is_sorted(&y.1));
    }
}

fn _sort_array_buckets(buckets: &mut [(u8, u8, u8); 256 * 256]) {
    //fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 *256]; 256] {
    thread::scope(|scope| {
        //for slice in v.chunks_mut(run_size) {

        for _each in buckets {
            scope.spawn(move || {
                // requires marker Send -  T` cannot be sent between threads safely
                //each.sort(); //requires the trait Ord.
            });
        }
    });
}

//TODO: overflows its stack #[test]
fn _t_create_arr_samesz_tuples() {
    let start = SystemTime::now();
    // ideally:
    //fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 * 256 * 256]; 256] {
    //let x = create_array_three_tuples_maxvm_reverse_order();
    let x = create_array_three_tuples_sameszonearray_reverse_order();
    sort_utils::end_and_print_time(
        start,
        &format!(
            "created {} arrays each with an array sized {}",
            x.len(),
            x[0].len()
        ),
    );

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

    //for i in 0..128 {
    for i in 0..48 {
        //let y = Vec::new(x[i]);
        let z = vec![x[i]];
        assert!(sort_utils::is_sorted(&z));
    }
    /* // serial creation. a serial sort when i'm done. this is not ideal.
    created 128 arrays each with an array sized 16777216 10.279 seconds
    the array has 128 elements.
                   sorted each array 6.942 seconds
    */
    /* // this when i move sorting into array creation. creation is still serial, so that should change.
    created 128 arrays each with an array sized 16777216 13.947 seconds
    the array has 128 elements.
                   sorted each array 0.0 seconds
    test t_create_arr_tuples ... ok
    */
}

//TODO: #[test]
fn _t_create_arr_maxsz_tuples() {
    let start = SystemTime::now();
    // ideally, but biggest i can get on my VM is 128.
    //fn create_array_three_tuples_reverse_order() -> [[(u8, u8, u8); 256 * 256 * 256]; 256] {
    let x = _create_array_three_tuples_maxvm_reverse_order();
    sort_utils::end_and_print_time(
        start,
        &format!(
            "created {} arrays each with an array sized {}",
            x.len(),
            x[0].len()
        ),
    );

    println!("the array has {} elements.", x.len());

    let start = SystemTime::now();

    sort_utils::end_and_print_time(start, "sorted each array");

    for i in 0..128 {
        let z = vec![x[i]];
        assert!(sort_utils::is_sorted(&z));
    }
    /* // serial creation. a serial sort when i'm done. this is not ideal.
    created 128 arrays each with an array sized 16777216 10.279 seconds
    the array has 128 elements.
                   sorted each array 6.942 seconds
    */
    /* // this when i move sorting into array creation. creation is still serial, so that should change.
    created 128 arrays each with an array sized 16777216 13.947 seconds
    the array has 128 elements.
                   sorted each array 0.0 seconds
    test t_create_arr_tuples ... ok
    */
}

//#[test]
fn _t_create_arr_four_tuples() {
    // i can't build more than ~48 * 256^3 tuples as one array.
    //  on my VM at home.

    let start = SystemTime::now();
    //fn create_array_four_tuples_reverse_order() -> Box<[[(u8, u8, u8, u8); 256*256*256*128]> {
    //TODO: overflows its stack.
    let x = _create_array_four_tuples_reverse_order();
    sort_utils::end_and_print_time(start, &format!("created {} arrays.", x.len()));

    println!("the array has {} elements.", x.len());

    let start = SystemTime::now();
    //move sort into creation.
    //x.sort();
    //x.par_sort();
    sort_utils::end_and_print_time(start, "sorted each array");

    /*
           created 805306368 arrays. 5.581 seconds
    the array has 805306368 elements.
                   sorted each array 6.830 seconds
    test t_create_arr_four_tuples ... ok


    */
}

#[test]
fn t_get_one_random_tuple() {
    let t = create_one_tuple_random_bytes();
    println!("got a tuple: {} {} {} {}", t.0, t.1, t.2, t.3);
}

//issue with this for now abandoned strategy. #[test]
fn _t_search_random_tuple_in_one_array() {
    // limitation imposed by my VM with 8GB memory.
    let mut t = create_one_tuple_random_bytes();
    while t.0 > 47 {
        println!("got a tuple: {} {} {} {}", t.0, t.1, t.2, t.3);
        t = create_one_tuple_random_bytes();
    }

    let x = _create_array_four_tuples_reverse_order();

    let answer = x.binary_search(&t);

    match answer {
        Ok(index) => println!(
            "found {} at index {} ",
            format!("{}-{}-{}-{}", t.0, t.1, t.2, t.3),
            index
        ),
        Err(some_error) => println!("{}", some_error),
    }
}

fn _get_just_my_three_tuple(t0: usize) -> [(u8, u8, u8); 256 * 256 * 256] {
    let x = create_array_three_tuples_sameszonearray_reverse_order();

    let y = x[t0];
    y
}

//TODO: overflows its stack. #[test] //remove from test while i try to use gdb to look at the stack.
pub fn _t_search_random_tuple_in_many_arrays() {
    // limitation imposed by my VM with 8GB memory.
    let mut t = create_one_tuple_random_bytes();
    while t.0 > 47 {
        println!("got a tuple: {} {} {} {}", t.0, t.1, t.2, t.3);
        t = create_one_tuple_random_bytes();
    }

    /*
    let x = create_array_three_tuples_sameszonearray_reverse_order();

    //let x = vec!(x); //doesn't help. tried this with modified drop_save_stack.

    //first level search is an index saving many ops....or only 8 bc it's log_2 256.
    //let y = x[t.0 as usize]; //overflows the stack.
    let y = x[t.0 as usize];
    //let y = Box::new(x[t.0 as usize]);  //doesn't help.
    //let y = drop_save_stack(t.0 as usize, x);
    //println!("y is the one array i want with size {}", y.len());
    */

    //let y = get_just_my_three_tuple(t.0 as usize); //still overflows.
    //let y = Box::new(get_just_my_three_tuple(t.0 as usize));
    let y: [(u8, u8, u8); 256 * 256 * 256] = _get_just_my_three_tuple(t.0 as usize);

    println!("do i get just one array? {}", y.len());

    //let y = vec!(get_just_my_three_tuple(t.0 as usize));

    let _t0 = t.0;
    //need to split apart the tuple here...
    let _t = (t.1, t.2, t.3);
    //let answer = y.binary_search(&t);  //overflowed its stack.

    //alternative.
    //let answer = sort_utils::find_split_point(y, 0, y.len(), t);
    //let answer = my_bin_search(y, 0, y.len(), t); y.len - ownership taken by function.
    //let answer = my_bin_search(y, 0, 256*256*256, t);
    //println!("found {} at index {} ", format!("{}-{}-{}-{}", t0, t.0, t.1, t.2), answer);
    //pub fn find_split_point(v: &Vec<u64>, p: usize, r: usize, x: u64) -> usize {

    /*
    match answer {
    Ok(index) => println!("found {} at index {} ", format!("{}-{}-{}-{}", t0, t.0, t.1, t.2), index),
    Err(some_error) => println!("{}", some_error),
    }
    */
}

// not helping in these forms...
fn _drop_save_stack(
    index: usize,
    x: Box<[[(u8, u8, u8); 256 * 256 * 256]; 48]>,
) -> [(u8, u8, u8); 256 * 256 * 256] {
    //fn drop_save_stack(index: usize, x: Vec<Box<[[(u8, u8, u8); 256 *256 * 256]; 48]>>) -> Box<[(u8, u8, u8); 256 *256 * 256]> {
    x[index]
    //let x = &x[0];
    //let y = x[index];
    //drop(x);
    //Box::new(y)
    //y
}

/// binary search. TODO: compare to https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
pub fn _my_bin_search(
    v: Box<[(u8, u8, u8); 256 * 256 * 256]>,
    p: usize,
    r: usize,
    x: (u8, u8, u8),
) -> usize {
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

//TODO: overflows its stack #[test]
fn _t_create_array_four_reverse_order() {
    //too big for stack - but i can Box:
    //let arr = _create_array_four_reverse_order();
    let arr = _create_little_array_four_prototype();
    //let arr = create_array_four_large();
    println!("you created an array of arrays of length {}", arr.len());
    println!("what does it look like?");
    for i in 0..10 {
        println!("{:?}", arr[i]);
    }

    //panic!();
}

fn _create_and_sort_small() -> Vec<Vec<Vec<[u8; 2]>>> {
    let mut v = create_vecvecvecarr_variable(2, 2, 2);
    //let mut v = create_vecvecvecarr();

    println!(
        "created vec sz {} of vecs of sz {} of vec of sz {} of arrays of size {}",
        v.len(),
        v[0].len(),
        v[0][0].len(),
        v[0][0][0].len()
    );

    //now can we sort as we create?.
    v.par_iter_mut().for_each(|i| {
        i.par_iter_mut().for_each(|j| {
            for k in 0..j.len() {
                let x = rand::thread_rng().gen_range(1..=u8::MAX);
                let y = rand::thread_rng().gen_range(1..=u8::MAX);
                j[k] = [x, y]
            }
            j.sort()
        });
    });
    v
}

fn _create_and_sort_large() -> Vec<Vec<Vec<[u8; 2]>>> {
    let start = SystemTime::now();

    //let mut v = create_vecvecvecarr_variable(8,8,8);
    //let mut v = create_vecvecvecarr_variable(32,32,32);
    //let mut v = create_vecvecvecarr_variable(128,128,128);
    //let mut v = create_vecvecvecarr_variable(256*256,128,128);
    let mut v = create_vecvecvecarr_variable(256 * 256, 256, 4);
    //finishes in 38 seconds in release mode
    //let mut v = create_vecvecvecarr_variable(256 * 256, 256, 128);
    //86.05s
    //let mut v = create_vecvecvecarr();

    sort_utils::end_and_print_time(start, "created.");

    println!(
        "created vec sz {} of vecs of sz {} of vec of sz {} of arrays of size {}",
        v.len(),
        v[0].len(),
        v[0][0].len(),
        v[0][0][0].len()
    );

    let start = SystemTime::now();
    //now can we sort as we create?.
    v.par_iter_mut().for_each(|i| {
        i.par_iter_mut().for_each(|j| {
            let mut rng = ChaCha8Rng::from_entropy(); //works
            for k in 0..j.len() {
                //let x = rand::thread_rng().gen_range(1..=u8::MAX);
                //let y = rand::thread_rng().gen_range(1..=u8::MAX);
                let x = rng.gen_range(1..=u8::MAX);
                let y = rng.gen_range(1..=u8::MAX);
                j[k] = [x, y]
            }
            j.sort()
        });
    });
    sort_utils::end_and_print_time(start, "filled, sorted rand vals in parallel.");

    v
}

#[test]
fn t_create_vecvecvecarr() {
    //fn create_vecvecvecarr() -> Vec<Vec<Vec<[u8; 2]>>> {
    let mut v = create_vecvecvecarr();

    println!(
        "created vec sz {} of vecs of sz {} of vec of sz {} of arrays of size {}",
        v.len(),
        v[0].len(),
        v[0][0].len(),
        v[0][0][0].len()
    );

    /* // crossbeam probably can work if i figure it out....not this...
            let unit_step = v.len() / 4; //tmp for experiment

        let _ = crossbeam::scope(|scope| {
            for l2 in v.chunks_mut(unit_step) {
                //println!("this slice is: {:?}", slice);
                scope.spawn(move |_| l2[1][2][3] = [ 3 as u8, 4 as u8]);
            }
        });
    */

    //(0..100).into_par_iter()
    //v.into_par_iter()
    //v.par_iter()
    //v.clone().into_par_iter() // don't want to clone...
    //.for_each(|i| println!("{:?}", i[0][0]));
    //.for_each(|mut i| i[0][0] = [ 0xd, 0xe ]);
    //.for_each(|mut &mut Vec<Vec<[u8; 2]>>| i[0][0] = [ 0xd, 0xe ]);
    //.for_each(|mut &mut Vec<Vec<[u8; 2]>>| i[0][0] = [ 0xd, 0xe ]);

    /* //solid start
        v.par_iter_mut()
                .for_each(|mut i| i[0][0] = [ 0xd, 0xe ]);
    */

    /* //improvement on start
        v.par_iter_mut()
                .for_each(|mut i| {
                        i.par_iter_mut()
                            .for_each(|mut j| j[0] = [ 0xd, 0xe ]);
                });
    */

    /*
        //another improvement - all the values filled in now.
        v.par_iter_mut()
                .for_each(|mut i| {
                        i.par_iter_mut()
                            .for_each(|mut j|
                                for k in 0..j.len() {
                                    let x = rand::thread_rng().gen_range(1..=u8::MAX);
                                    let y = rand::thread_rng().gen_range(1..=u8::MAX);
                                    j[k] = [ x, y ]
                                });

                });
    */

    //now can we sort as we create?.
    v.par_iter_mut().for_each(|i| {
        i.par_iter_mut().for_each(|j| {
            for k in 0..j.len() {
                let x = rand::thread_rng().gen_range(1..=u8::MAX);
                let y = rand::thread_rng().gen_range(1..=u8::MAX);
                j[k] = [x, y]
            }
            j.sort()
        });
        //i.sort(); //wrong place.
    });

    /*
    (0..5).for_each(|i|{
        (0..5).into_par_iter().for_each_with(&sender, |sender, j|{
            sender.send(i + j).unwrap();
        });
    });
    */

    for i in 0..v.len() {
        for j in 0..v[i].len() {
            for k in 0..v[j].len() {
                println!("{:?}", v[i][j][k]);
            }
        }
    }
}

#[test]
fn t_create_and_sort_small() {
    let _v = _create_and_sort_small();
}

#[test]
fn t_create_and_sort_large() {
    let _v = _create_and_sort_large();
}

#[test]
fn t_create_cb_vecvecvecarr() {
    /* //test capacity:
    let sz_l1 = 256; //192; //256; //128; //64; //32; //16; //32; //16; //4; //128; //32 takes 131 seconds on my VM; slowest step is random creation of data.  //16 makes a total of 1.57Gb RAM used on my VM at peak.
    let sz_l2 = 256; //4; //128; //64; //16; //32; //64; //128; //256;
    let sz_l3 = 256*256; //16; //256*256;
    */

    /*
    //test correctness:
    let sz_l1 = 4;
    let sz_l2 = 4;
    let sz_l3 = 12 * 12;
        */

    //workable/testable capacity:
    let sz_l1 = 4;
    let sz_l2 = 256;
    let sz_l3 = 256 * 256;

    let start = SystemTime::now();

    // get default values of the proper size so i can fill in by index going forward. i don't want to expand the size of the vecs as i insert bc i know exactly how many and of what size i'm creating before i start.
    let mut v = create_vecvecvecarr_variable(sz_l3, sz_l2, sz_l1);

    sort_utils::end_and_print_time(start, "created data structure.");

    println!(
        "created vec sz {} of vecs of sz {} of vec of sz {} of arrays of size {}",
        v.len(),
        v[0].len(),
        v[0][0].len(),
        v[0][0][0].len()
    );

    let start = SystemTime::now();

    // wrap the data structure so i can mark it with send/sync traits. i'm telling the compiler i will use the structure between threads. Calling the structure a Bag not due to precision of description but as a rough and short name for the collection.
    let raw_v: WrapperBag = WrapperBag(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for _ in 0..sz_l1 {
            // it seems reasonable on many machines to carve up the number of work items into the outer number of arrays. this is problematic on my little VM though. tuning this for your machine might be appropriate, but it alters correctness of the algorithm so proceed carefully.
            scope.spawn(|_| {
                println!(
                    "\tDEBUG: show each thread same pointer same data structure: {:p}",
                    &raw_v
                );

                println!(
                    "\tDEBUG: each thread assigned work of level 2 times level 3 operations: {}.",
                    sz_l2 * sz_l3
                );

                //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works

                println!(
                    "\tDEBUG: show threads don't get the same random numbers: {}",
                    rng.gen_range(1..=u8::MAX)
                );

                //randomly fill (some) of the structure.
                for _ in 0..sz_l2 * sz_l3 {
                    // random values:
                    let a = rng.gen_range(1..=u8::MAX);
                    let b = rng.gen_range(1..=u8::MAX);

                    //let m = rand::thread_rng().gen_range(0..sz);
                    //random indices:
                    let i = rng.gen_range(0..sz_l1);
                    let j = rng.gen_range(0..sz_l2);
                    let k = rng.gen_range(0..sz_l3);

                    //assign random values in the Bag randomly. we don't care if we're overwriting values for this prototype.
                    unsafe {
                        (*raw_v.0)[i][j][k] = [a, b];
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    /*
    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 0..sz_l1 {
        for j in 0..sz_l2 {
            for k in 0..sz_l3{
                println!("{:x} {:x} {:x} {:x}", i, j, v[i][j][k][0], v[i][j][k][1]);
            }
        }
    }
    */

    let start = SystemTime::now();

    // here we have random values; the first two elements are sorted since we stored in an array. one goal here will be to study this method (middling method) against the obvious and extreme alternatives: create one giant vector of random values, or create vectors of single values four times (many arrays). there are cpu and memory tradeoffs for each; e.g., sort is required to search and many small sorts are extremely fast with many available threads.
    let _ =
        crossbeam::scope(|scope| {
            for l2 in 0..sz_l1 {
                // create a thread for the outer number of elements.
                scope.spawn(move |_| {

								println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v); 

                println!(
                    "\tDEBUG: move also obtains a (sort of) thread id: {:?}.",
                    l2
                );

                // the work of each thread is to sort all of the 3rd/4th values in each of the vectors of the 2d value, belonging to the values of the 1st value (assigned for each thread).
                for index in 0..sz_l2 {
                    // default/system sort expected to be good enough because the size of the innermost vector is capped at 2^16.
                    unsafe { (*raw_v.0)[l2][index].sort() }
                }
            });
            }
        });

    sort_utils::end_and_print_time(start, "sort bags.");

    /*
    // print to get a sense of sorting correctness only; i.e., are 3rd/4th values sorted? yes.
    for i in 0..sz_l1 {
        for j in 0..sz_l2 {
            for k in 0..sz_l3{
                println!("{:x} {:x} {:x} {:x}", i, j, v[i][j][k][0], v[i][j][k][1]);
            }
        }
    }
        */

    let start = SystemTime::now();

    // get the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count = 0;
    println!(
        "v.len {} v[i].len {} v[i][j].len {}",
        v.len(),
        v[0].len(),
        v[0][0].len()
    );
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            for k in 0..v[i][j].len() {
                if v[i][j][k] != [0, 0] {
                    count = count + 1;
                }
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3) as f64 * 100.0) as f64
    );

    /*
        for i in 0..sz_l1 {
            for j in 0..sz_l2 {
                for k in 0..sz_l3 {
                    println!("{:?}", v[i][j][k]);
                }
            }
        }
    */

    // parallel binary searches...
    let start = SystemTime::now();
    // search for a bunch.
    //let mut sought = 0;
    //let mut found = 0;
    //for last in 0..sz_l3 {

    let _ =
        crossbeam::scope(|scope| {
            for l2 in 0..sz_l1 {
                //
                scope.spawn(move |_| {
								println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v); 

                println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                for middle in 0..sz_l2 {
                    for _ in 0..sz_l3 {
                        let c = rng.gen_range(1..=u8::MAX);
                        let d = rng.gen_range(1..=u8::MAX);
                        let mut _answer = Ok(0); //default.
                        unsafe {
                            _answer = (*raw_v.0)[l2][middle].binary_search(&[c, d]);
                        }
                        total_sought = total_sought + 1;
                        match _answer {
                            //Ok(index) => { total_found = total_found + 1; println!("found {} at index {} ", format!("{}-{}-{}-{}", outer, middle, c, d), index) },
                            Ok(_index) => {
                                total_found = total_found + 1;
                            }
                            Err(_some_error) => (), //println!("not found. index where should be: {}", _some_error),
                        }
                    }
                }

                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    l2, total_found, total_sought
                );
                //sought = sought + total_sought;
                //found = found + total_found;
            });
            } //end outer loop
        });

    //println!("total: found {} out of searched {}", found, sought);

    sort_utils::end_and_print_time(start, "binary searches.");
}

/*
#[test]
fn t_arr_and_tuple_failed_too_much_memory() {

    //workable/testable capacity:
    let sz_l1 = 4;
    let sz_l2 = 256;
    let sz_l3 = 256 * 256;

    let start = SystemTime::now();

    let mut v = create_vecvecvectuplearrs_variable(sz_l3, sz_l2, sz_l1);

    sort_utils::end_and_print_time(start, "created data structure.");

    println!(
        "created vec sz {} containings vecs of sz {} containing vecs of tuples with arrays inside of sz {}",
        v.len(),
        v[0].len(),
        v[0][0].len(),
        //v[0][0][0].len()
    );


    let start = SystemTime::now();

    let raw_v: WrapperBagExt = WrapperBagExt(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            // it seems reasonable on many machines to carve up the number of work items into the outer number of arrays. this is problematic on my little VM though. tuning this for your machine might be appropriate, but it alters correctness of the algorithm so proceed carefully.
            //scope.spawn(|_| {
            scope.spawn(move|_| { //try move to get index.
                println!(
                                        //NOTE: must deference? somehow touch the pointer inside the closure to make it available. TODO: know exactly why.
                    "\tDEBUG: show each thread same pointer same data structure: {:p}",
                    &raw_v
                );

                println!(
                    "\tDEBUG: each thread assigned work of level 2 times level 3 operations: {}.",
                    sz_l2 * sz_l3
                );

                //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works

                println!(
                    "\tDEBUG: show threads don't get the same random numbers: {}",
                    rng.gen_range(1..=u8::MAX)
                );

                //randomly fill (some) of the structure.
                //for _ in 0..sz_l2 * sz_l3 {
                for middle in 0..sz_l2 {
                                        //for inner in 0..sz_l3 { //conceptually same as below.
                                        for inner in 0..sz_l2 {
                                            for inmost in 0..sz_l2 {
                                                // random values:
                                                let a = rng.gen_range(1..=u8::MAX);
                                                let b = rng.gen_range(1..=u8::MAX);

                                                //let m = rand::thread_rng().gen_range(0..sz);
                                                //random indices:
                                                let i = rng.gen_range(0..sz_l1); //this hacky, as i am mixing bytes and indices. realize that until i work it out completely.
                                                let j = rng.gen_range(0..sz_l2);
                                                let k = rng.gen_range(0..sz_l3);

                                                //assign random values in the Bag randomly. we don't care if we're overwriting values for this prototype.
                                                //let inner_most_index = (middle * sz_l2) + inner;
                                                let inner_most_index = (inner * sz_l2) + inmost;
                                                unsafe {
                                                        //shows completeness. all entries filled in order, to prove methodology before randomization.
                                                        //(*raw_v.0)[outer][middle][inner_most_index] = ([a, b], [outer as u8, middle as u8, inner as u8, inmost as u8]);
                                                        //randomly fill, but keep indices so i can track which is filled.
                                                        (*raw_v.0)[i][j][k] = ([a, b], [outer as u8, middle as u8, inner as u8, inmost as u8]);
                                                        //println!("another sense of the filling - what random value am i assigning? i {}, j {}, k {}", i, j, k);
                                                }
                                            }
                                        }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");
*/

#[test]
fn t_arr_long() {
    /*
        //test capacity:
       let sz_l1 = 256; //192; //256; //128; //64; //32; //16; //32; //16; //4; //128; //32 takes 131 seconds on my VM; slowest step is random creation of data.  //16 makes a total of 1.57Gb RAM used on my VM at peak.
       let sz_l2 = 256; //4; //128; //64; //16; //32; //64; //128; //256;
       let sz_l3 = 256*256; //16; //256*256;
    */

    /*
        //test correctness:
        let sz_l1 = 2;
        let sz_l2 = 8;
        let sz_l3 = 8 * 8;
    */

    //workable/testable capacity:
    //let sz_l1 = 108; //will NOT run on my VM.
    //let sz_l1 = 106; //the biggest on my VM. 10.7G out of 11 avail. 366s. //with BS: still 10.7 but goes 300+ MB into swap. 434s.
    //let sz_l1 = 99; // biggest after mods - impl tweaks using slightly more memory. 100-103 almost fit but hit swap variously in each phase.. 10.5Gb, 362s, NO swap. swap kills at least the performance of the creation of the data structure.
    //let sz_l1 = 80; // 8.47G. runs in 258s. // 8.3G runs in 241s. //8.6 in 286s.
    //let sz_l1 = 72; // 7.72G. runs in 232s. //improve sort: 7.85G 334s. restarting VM.
    //let sz_l1 = 64; // 6.76G. runs in 165s.
    //let sz_l1 = 56; // 6.15G. runs in 150s.
    //let sz_l1 = 48; // 5.25G. runs in 122s. //with BS: 5.42 140s. //improved sort: 5.56 and 163s.
    //let sz_l1 = 32; // 3.75G. runs in 78s. //with BS: 3.91 110s //improved sort: 4.05 and 102s.
    //let sz_l1 = 24; // 2.97G. runs in 57s. //with BS: 3.16 67s. //improved sort: 3.27 and 74s.
    //let sz_l1 = 16; // 2.22G. runs in 36s.
    let sz_l1 = 8; // 1.63G. runs in 16s. //improved sort: 1.74G 22s.
    let sz_l2 = 256;
    let sz_l3 = 256 * 256;

    let start = SystemTime::now();

    let mut v = create_vecvecvecarrlong_variable(sz_l3, sz_l2, sz_l1);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    let raw_v: WrapperBagExt = WrapperBagExt(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            // it seems reasonable on many machines to carve up the number of work items into the outer number of arrays. this is problematic on my little VM though. tuning this for your machine might be appropriate, but it alters correctness of the algorithm so proceed carefully.
            //scope.spawn(|_| {
            scope.spawn(move |_| {
                //try move to get index.
                /*
                println!(
                                        //NOTE: must deference? somehow touch the pointer inside the closure to make it available. disjoint capture.
                    "\tDEBUG: move changes the pointer - same data structure: {:p}",
                    &raw_v
                );
                                */
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   /*
                                                   println!(
                                                       "\tDEBUG: each thread assigned work of level 2 times level 3 operations: {}.",
                                                       sz_l2 * sz_l3
                                                   );
                                   */
                //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          /*
                                                                          println!(
                                                                              "\tDEBUG: show threads don't get the same random numbers: {}",
                                                                              rng.gen_range(1..=u8::MAX)
                                                                          );
                                                          */

                //randomly fill (some) of the structure.
                //for _ in 0..sz_l2 * sz_l3 {
                for middle in 0..sz_l2 {
                    //for inner in 0..sz_l3 { //conceptually same as below.
                    for inner in 0..sz_l2 {
                        for inmost in 0..sz_l2 {
                            // random values:
                            let a = rng.gen_range(1..=u8::MAX);
                            let b = rng.gen_range(1..=u8::MAX);

                            //let m = rand::thread_rng().gen_range(0..sz);
                            //random indices:
                            let i = rng.gen_range(0..sz_l1); //this hacky, as i am mixing bytes and indices. realize that until i work it out completely.
                            let j = rng.gen_range(0..sz_l2);
                            let k = rng.gen_range(0..sz_l3);

                            //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                            //let inner_most_index = (middle * sz_l2) + inner;
                            //let inner_most_index = (inner * sz_l2) + inmost;
                            unsafe {
                                (*raw_v.0)[i][j][k] =
                                    [a, b, outer as u8, middle as u8, inner as u8, inmost as u8];
                                //println!("another sense of the filling - what random value am i assigning? i {}, j {}, k {}", i, j, k);
                            }
                        }
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    /*
        // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
        for i in 0..sz_l1 {
            for j in 0..sz_l2 {
                for k in 0..sz_l3{
                    println!("{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}", i, j, v[i][j][k][0], v[i][j][k][1], v[i][j][k][2], v[i][j][k][3], v[i][j][k][4], v[i][j][k][5] );
                }
            }
        }
    */

    let start = SystemTime::now();

    // here we have random values; the first two elements are sorted since we stored in an array. one goal here will be to study this method (middling method) against the obvious and extreme alternatives: create one giant vector of random values, or create vectors of single values four times (many arrays). there are cpu and memory tradeoffs for each; e.g., sort is required to search and many small sorts are extremely fast with many available threads. four byte values is (beyond) the limit of values i can check on my VM; at least at the boundary, so it's a good choice to see whether any of these alternatives can get me there.
    let _ = crossbeam::scope(|scope| {
        for l2 in 0..sz_l1 {
            // create a thread for the outer number of elements.
            scope.spawn(move |_| {
                //println!("\tDEBUG: move changes the pointer - same data structure: {:p}", &raw_v);
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                /*
                                println!(
                                    "\tDEBUG: move also obtains a (sort of) thread id: {:?}.",
                                    l2
                                );
                */

                // the work of each thread is to sort all of the 3rd/4th values in each of the vectors of the 2d value, belonging to the values of the 1st value (assigned for each thread).
                for index in 0..sz_l2 {
                    // default/system sort expected to be good enough because the size of the innermost vector is capped at 2^16.
                    //unsafe { (*raw_v.0)[l2][index].sort() } //solid.
                    unsafe { (*raw_v.0)[l2][index].sort_by_key(|&[m, n, _o, _p, _q, _r]| [m, n]) }
                    //faster by ~20%?
                    //unsafe { (*raw_v.0)[l2][index].par_sort() //slower on this small dataset. }
                    //unsafe { (*raw_v.0)[l2][index].sort_by(|a, b| a[0].cmp(&b[0])) } //this method not written correctly.
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "sort bags.");

    /*
        // print to get a sense of sorting correctness only; i.e., are 3rd/4th values sorted? yes.
        for i in 0..2 {
            //for j in 0..sz_l2 {
            for j in 0..2 {
                //for k in 0..sz_l3{
                for k in 40000..40010{
                    println!("{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}", i, j, v[i][j][k][0], v[i][j][k][1], v[i][j][k][2], v[i][j][k][3], v[i][j][k][4], v[i][j][k][5]);
                }
            }
        }
    */

    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count = 0;
    println!(
        "v.len {} v[i].len {} v[i][j].len {}",
        v.len(),
        v[0].len(),
        v[0][0].len()
    );
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            for k in 0..v[i][j].len() {
                if v[i][j][k] != [0, 0, 0, 0, 0, 0] {
                    count = count + 1;
                }
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        v.len() * v[0].len() * v[0][0].len(),
        (count as f64 / (v.len() * v[0].len() * v[0][0].len()) as f64 * 100.0) as f64
    );

    /*
        for i in 0..sz_l1 {
            for j in 0..sz_l2 {
                for k in 0..sz_l3 {
                    println!("{:?}", v[i][j][k]);
                }
            }
        }
    */

    // parallel binary searches...
    let start = SystemTime::now();
    // search for a bunch.
    //let mut sought = 0;
    //let mut found = 0;
    //for last in 0..sz_l3 {

    let _ = crossbeam::scope(|scope| {
        for l2 in 0..sz_l1 {
            //
            scope.spawn(move |_| {
                //println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v);

                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                //println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                for middle in 0..sz_l2 {
                    for _ in 0..sz_l3 {
                        let c = rng.gen_range(1..=u8::MAX);
                        let d = rng.gen_range(1..=u8::MAX);
                        let mut _answer = Ok(0); //default.
                                                 //println!("\tDEBUG: searching for {}, {}, {}, {}", l2, middle, c, d);
                        unsafe {
                            //_answer = (*raw_v.0)[l2][middle].binary_search(&[c, d]);
                            _answer = (*raw_v.0)[l2][middle]
                                .binary_search_by_key(&[c, d], |&[m, n, _o, _p, _q, _r]| [m, n])
                        }
                        total_sought = total_sought + 1;
                        match _answer {
                            //Ok(index) => { unsafe {  total_found = total_found + 1; println!("found {} at index {} ", format!("{}-{}-{}-{} {}-{}-{}-{}", l2, middle, c, d, (*raw_v.0)[l2][middle][index][2],(*raw_v.0)[l2][middle][index][3],(*raw_v.0)[l2][middle][index][4],(*raw_v.0)[l2][middle][index][5]), index) } },
                            Ok(_index) => {
                                total_found = total_found + 1;
                            }
                            Err(_some_error) => (), //println!("not found. index where should be: {}", _some_error),
                        }
                    }
                }

                /*
                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    l2, total_found, total_sought
                ); */
                //sought = sought + total_sought;
                //found = found + total_found;
            });
        } //end outer loop
    });

    //println!("total: found {} out of searched {}", found, sought);

    sort_utils::end_and_print_time(start, "binary searches.");
}

#[test]
fn t_arrall() {
    /* uses only 5.63 Gb but takes about fifteen minutes on my VM.
            //test capacity:
           let sz_l1 = 256; //192; //256; //128; //64; //32; //16; //32; //16; //4; //128; //32 takes 131 seconds on my VM; slowest step is random creation of data.  //16 makes a total of 1.57Gb RAM used on my VM at peak.
           let sz_l2 = 256; //4; //128; //64; //16; //32; //64; //128; //256;
           let sz_l3 = 256; //16; //256*256;
           let sz_l4 = 256; //16; //256*256;
    */

    /*
            //test correctness:
            let sz_l1 = 1;
            let sz_l2 = 4;
            let sz_l3 = 4;
            //let sz_l4 = 4;
    */

    //workable/testable capacity:
    //let sz_l1 = 108; //
    //let sz_l1 = 106; //
    //let sz_l1 = 99; //
    //let sz_l1 = 80; //
    //let sz_l1 = 72; //
    //let sz_l1 = 64; //
    //let sz_l1 = 56; //
    //let sz_l1 = 48; //
    //let sz_l1 = 32; //
    //let sz_l1 = 24; //
    //let sz_l1 = 16; //
    //let sz_l1 = 4; //created data structure. 0.455 seconds filled Bag w/ random values. 88.276 seconds

    let sz_l1 = 2; //created data structure. 0.257 seconds filled Bag w/ random values. 54.42 seconds
    let sz_l2 = 256;
    let sz_l3 = 256;
    //let sz_l4 = 256;

    let start = SystemTime::now();

    //let mut v = create_vecvecvecvec(sz_l4, sz_l3, sz_l2, sz_l1);
    let mut v = create_vecvecvecvec(sz_l3, sz_l2, sz_l1);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    let raw_v: WrapperArrsExt = WrapperArrsExt(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            // it seems reasonable on many machines to carve up the number of work items into the outer number of arrays. this is problematic on my little VM though. tuning this for your machine might be appropriate, but it alters correctness of the algorithm so proceed carefully.
            //scope.spawn(|_| {
            scope.spawn(move |_| {
                //try move to get index.
                /*
                println!(
                                        //NOTE: must deference? somehow touch the pointer inside the closure to make it available. disjoint capture.
                    "\tDEBUG: move changes the pointer - same data structure: {:p}",
                    &raw_v
                );
                                */
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   /*
                                                   println!(
                                                       "\tDEBUG: each thread assigned work of level 2 times level 3 operations: {}.",
                                                       sz_l2 * sz_l3
                                                   );
                                   */
                //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          /*
                                                                          println!(
                                                                              "\tDEBUG: show threads don't get the same random numbers: {}",
                                                                              rng.gen_range(1..=u8::MAX)
                                                                          );
                                                          */

                //randomly fill (some) of the structure.
                //for _ in 0..sz_l2 * sz_l3 {
                for middle in 0..sz_l2 {
                    //for inner in 0..sz_l3 { //conceptually same as below.
                    for inner_one in 0..sz_l3 {
                        for innner_two in 0..256 {
                            //inmost is an array with fixed size.
                            // random values:
                            //let a = rng.gen_range(1..=u8::MAX);
                            //let b = rng.gen_range(1..=u8::MAX);

                            //let m = rand::thread_rng().gen_range(0..sz);
                            //random indices:
                            let i = rng.gen_range(0..sz_l1); //this hacky, as i am mixing bytes and indices. realize that until i work it out completely.
                            let j = rng.gen_range(0..sz_l2);
                            let k = rng.gen_range(0..sz_l3);
                            //let m = rng.gen_range(0..sz_l4);
                            let m = rng.gen_range(0..256);

                            //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                            //let inner_most_index = (middle * sz_l2) + inner;
                            //let inner_most_index = (inner * sz_l2) + inmost;
                            unsafe {
                                (*raw_v.0)[i][j][k][m] = true;
                                //[a, b, outer as u8, middle as u8, inner as u8, inmost as u8];
                                //println!("another sense of the filling - what random value am i assigning? i {}, j {}, k {}", i, j, k);
                            }
                        }
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    /*
            // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
            for i in 0..sz_l1 {
                for j in 0..sz_l2 {
                    for k in 0..sz_l3{
                                        //for m in 0..sz_l4{
                                        for m in 0..256{
                        println!("{:x} {:x} {:x} {:x} {}", i, j, k, m, v[i][j][k][m]);
                                        }
                    }
                }
            }
    */

    /* // sort not necessary. order comes from the array.
     */

    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count = 0;
    println!(
        "v.len {} v[i].len {} v[i][j].len {}",
        v.len(),
        v[0].len(),
        v[0][0].len()
    );
    for i in 0..sz_l1 {
        for j in 0..sz_l2 {
            for k in 0..sz_l3 {
                //for m in 0..sz_l4 {
                for m in 0..256 {
                    if v[i][j][k][m] {
                        count = count + 1;
                    }
                }
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * 256, //sz_l4,
        //(count as f64 / (sz_l1 * sz_l2 * sz_l3 * sz_l4) as f64 * 100.0) as f64
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * 256) as f64 * 100.0) as f64
    );

    // parallel NOTbinary searches...now they're just array lookups.
    let start = SystemTime::now();
    // search for a bunch.
    //let mut sought = 0;
    //let mut found = 0;
    //for last in 0..sz_l3 {

    let _ = crossbeam::scope(|scope| {
        for l2 in 0..sz_l1 {
            //
            scope.spawn(move |_| {
                //println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v);

                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                //println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                for m in 0..sz_l2 {
                    for inmid in 0..sz_l3 {
                        for inner in 0..256 {
                            let a = rng.gen_range(1..sz_l1);
                            let b = rng.gen_range(1..sz_l2);
                            let c = rng.gen_range(1..sz_l3);
                            let d = rng.gen_range(1..256);

                            unsafe {
                                /*
                                //search by index in order:
                                if 	(*raw_v.0)[l2][m][inmid][inner] {
                                    total_found = total_found + 1;
                                }
                                */
                                //search by random index:
                                if (*raw_v.0)[a][b][c][d] {
                                    total_found = total_found + 1;
                                }
                            }
                            total_sought = total_sought + 1;
                        }
                    }
                }

                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    l2, total_found, total_sought
                );
                //sought = sought + total_sought;
                //found = found + total_found;
            });
        } //end outer loop
    });

    //println!("total: found {} out of searched {}", found, sought);

    sort_utils::end_and_print_time(start, "binary searches.");
}

#[test]
fn t_onearr() {
    /*
            //test capacity:
           let sz_l1 = 256; //192; //256; //128; //64; //32; //16; //32; //16; //4; //128; //32 takes 131 seconds on my VM; slowest step is random creation of data.  //16 makes a total of 1.57Gb RAM used on my VM at peak.
           let sz_l2 = 256; //4; //128; //64; //16; //32; //64; //128; //256;
           let sz_l3 = 256; //16; //256*256;
           let sz_l4 = 256; //16; //256*256;
    */

    /*
            //test correctness:
            let sz_l1 = 1;
            let sz_l2 = 4;
            let sz_l3 = 4;
            let sz_l4 = 4;
    */

    // memory allocation now in allocation function bc it must be fixed there. ugly. probably just inline it now.
    //workable/testable capacity:
    //let sz_l1 = 108; //
    //let sz_l1 = 106; //
    //let sz_l1 = 99; //
    //let sz_l1 = 80; //
    //let sz_l1 = 72; //
    //let sz_l1 = 64; //
    //let sz_l1 = 56; //
    //let sz_l1 = 48; //
    //let sz_l1 = 32; //
    //let sz_l1 = 24; //
    //let sz_l1 = 16; //
    //let sz_l1 = 4; //created data structure. 0.455 seconds filled Bag w/ random values. 88.276 seconds

    //let sz_l1 = 128; // rayon goes into swap space for this size.
    let sz_l1 = 64; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    //let mut v = create_largevec();
    let mut v = vec![[0 as u8; 4]; sz_l1 * sz_l2 * sz_l3 * sz_l4];

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    let raw_v: WrapperOneArr = WrapperOneArr(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            // it seems reasonable on many machines to carve up the number of work items into the outer number of arrays. this is problematic on my little VM though. tuning this for your machine might be appropriate, but it alters correctness of the algorithm so proceed carefully.
            //scope.spawn(|_| {
            scope.spawn(move |_| {
                //try move to get index.
                /*
                println!(
                                        //NOTE: must deference? somehow touch the pointer inside the closure to make it available. disjoint capture.
                    "\tDEBUG: move changes the pointer - same data structure: {:p}",
                    &raw_v
                );
                                */
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   /*
                                                   println!(
                                                       "\tDEBUG: each thread assigned work of level 2 times level 3 operations: {}.",
                                                       sz_l2 * sz_l3
                                                   );
                                   */
                //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          /*
                                                                          println!(
                                                                              "\tDEBUG: show threads don't get the same random numbers: {}",
                                                                              rng.gen_range(1..=u8::MAX)
                                                                          );
                                                          */

                //randomly fill (some) of the structure.
                for _ in 0..sz_l1 * sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let a = rng.gen_range(1..=u8::MAX);
                    let b = rng.gen_range(1..=u8::MAX);
                    let c = rng.gen_range(1..=u8::MAX);
                    let d = rng.gen_range(1..=u8::MAX);

                    //let m = rand::thread_rng().gen_range(0..sz);
                    //random indices:
                    let i: usize = rng.gen_range(0..sz_l1 * sz_l2 * sz_l3 * sz_l4); //this hacky, as i am mixing bytes and indices. realize that until i work it out completely.

                    //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                    //let inner_most_index = (middle * sz_l2) + inner;
                    //let inner_most_index = (inner * sz_l2) + inmost;
                    unsafe {
                        (*raw_v.0)[i] = [a, b, c, d];
                        //[a, b, outer as u8, middle as u8, inner as u8, inmost as u8];
                        //println!("another sense of the filling - what random value am i assigning? i {}, j {}, k {}", i, j, k);
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 1_000_000_000..1_000_000_010 {
        println!("{:x} {:x} {:x} {:x} ", v[i][0], v[i][1], v[i][2], v[i][3]);
    }

    /* // sort necessary and expected expensive.
     */

    let start = SystemTime::now();
    v.par_sort();
    sort_utils::end_and_print_time(start, "sorted vector via rayon.");

    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 1_000_000_000..1_000_000_010 {
        println!("{:x} {:x} {:x} {:x} ", v[i][0], v[i][1], v[i][2], v[i][3]);
    }

    // how many filled...
    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count: u64 = 0;
    println!("v.len {} ", v.len(),);
    for i in 0..sz_l1 {
        for j in 0..sz_l2 {
            for k in 0..sz_l3 {
                for m in 0..sz_l4 {
                    if v[i] != [0, 0, 0, 0] {
                        count = count + 1;
                    }
                }
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * sz_l4,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * 256) as f64 * 100.0) as f64
    );

    /*
        // parallel NOTbinary searches...now they're just array lookups.
        let start = SystemTime::now();
        // search for a bunch.
        //let mut sought = 0;
        //let mut found = 0;
        //for last in 0..sz_l3 {

        let _ = crossbeam::scope(|scope| {
            for l2 in 0..sz_l1 {
                //
                scope.spawn(move |_| {
                    //println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v);

                    let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                    //println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                    let mut total_sought = 0;
                    let mut total_found = 0;
                    let mut rng = ChaCha8Rng::from_entropy(); //works

                    for m in 0..sz_l2 {
                        for inmid in 0..sz_l3 {
                                                for inner in 0..256 {

                            let a = rng.gen_range(1..sz_l1);
                            let b = rng.gen_range(1..sz_l2);
                            let c = rng.gen_range(1..sz_l3);
                            let d = rng.gen_range(1..256);

                            unsafe {
                                                            /*
                                                            //search by index in order:
                                                            if 	(*raw_v.0)[l2][m][inmid][inner] {
                                                                total_found = total_found + 1;
                                                            }
                                                            */
                                                            //search by random index:
                                                            if 	(*raw_v.0)[a][b][c][d] {
                                                                total_found = total_found + 1;
                                                            }
                            }
                            total_sought = total_sought + 1;
                                                }
                        }
                    }

                    println!(
                        "\tDEBUG: thread {} - found {} out of searched {}",
                        l2, total_found, total_sought
                    );
                    //sought = sought + total_sought;
                    //found = found + total_found;
                });
            } //end outer loop
        });

        //println!("total: found {} out of searched {}", found, sought);

        sort_utils::end_and_print_time(start, "binary searches.");
    */
}

#[test]
fn t_twostages_twolayer() {
    /*
            //test capacity:
           let sz_l1 = 256; //192; //256; //128; //64; //32; //16; //32; //16; //4; //128; //32 takes 131 seconds on my VM; slowest step is random creation of data.  //16 makes a total of 1.57Gb RAM used on my VM at peak.
           let sz_l2 = 256; //4; //128; //64; //16; //32; //64; //128; //256;
           let sz_l3 = 256; //16; //256*256;
           let sz_l4 = 256; //16; //256*256;
    */

    let sz_l1 = 256; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    //let mut v = vec![vec![[0 as u8; 2]; sz_l2  * sz_l3 * sz_l4]; sz_l1 ];
    let mut v = create_twostep(sz_l1, sz_l2, sz_l3, sz_l4);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    let raw_v: WrapperTwoStage = WrapperTwoStage(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works

                //randomly fill (some) of the structure.
                for _ in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let ind: u32 = rng.gen_range(0..256 * 256 * 256);
                    let v1 = rng.gen_range(0..u8::MAX);
                    let v2 = rng.gen_range(0..u8::MAX);

                    //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                    unsafe {
                        //(*raw_v.0)[outer][usize::from(ind)] = [v1, v2];
                        (*raw_v.0)[outer][usize::try_from(ind).unwrap()] = [v1, v2];
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 3..5 {
        //0..255 {
        for j in 1_000_000..1_000_010 {
            println!("{:x} {:x} {:x} ", i, v[i][j][0], v[i][j][1]);
        }
    }

    /* // sort necessary and expected expensive.

        let start = SystemTime::now();
            v.par_sort();
        sort_utils::end_and_print_time(start, "sorted vector via rayon.");



            // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
            for i in 1_000_000_000..1_000_000_010 {
                        println!("{:x} {:x} {:x} {:x} ", v[i][0], v[i][1], v[i][2], v[i][3]);
            }

    */

    // how many filled...
    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count: u64 = 0;
    println!("v.len {} ", v.len(),);
    for i in 0..sz_l1 {
        for j in 0..sz_l2 * sz_l3 * sz_l4 {
            if v[i][j] != [0, 0] {
                count = count + 1;
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * sz_l4,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * 256) as f64 * 100.0) as f64
    );

    /*
        // parallel NOTbinary searches...now they're just array lookups.
        let start = SystemTime::now();
        // search for a bunch.
        //let mut sought = 0;
        //let mut found = 0;
        //for last in 0..sz_l3 {

        let _ = crossbeam::scope(|scope| {
            for l2 in 0..sz_l1 {
                //
                scope.spawn(move |_| {
                    //println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v);

                    let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                    //println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                    let mut total_sought = 0;
                    let mut total_found = 0;
                    let mut rng = ChaCha8Rng::from_entropy(); //works

                    for m in 0..sz_l2 {
                        for inmid in 0..sz_l3 {
                                                for inner in 0..256 {

                            let a = rng.gen_range(1..sz_l1);
                            let b = rng.gen_range(1..sz_l2);
                            let c = rng.gen_range(1..sz_l3);
                            let d = rng.gen_range(1..256);

                            unsafe {
                                                            /*
                                                            //search by index in order:
                                                            if 	(*raw_v.0)[l2][m][inmid][inner] {
                                                                total_found = total_found + 1;
                                                            }
                                                            */
                                                            //search by random index:
                                                            if 	(*raw_v.0)[a][b][c][d] {
                                                                total_found = total_found + 1;
                                                            }
                            }
                            total_sought = total_sought + 1;
                                                }
                        }
                    }

                    println!(
                        "\tDEBUG: thread {} - found {} out of searched {}",
                        l2, total_found, total_sought
                    );
                    //sought = sought + total_sought;
                    //found = found + total_found;
                });
            } //end outer loop
        });

        //println!("total: found {} out of searched {}", found, sought);

        sort_utils::end_and_print_time(start, "binary searches.");
    */
}

#[test]
fn t_twostages_onelayer() {
    let sz_l1 = 256; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    let mut v = create_twoonelayer(sz_l1, sz_l2, sz_l3, sz_l4);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    let raw_v: WrapperTwoStageOneLayer = WrapperTwoStageOneLayer(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works

                //randomly fill (some) of the structure.
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    //let v1 = rng.gen_range(0..u8::MAX);
                    let v2 = rng.gen_range(0..u8::MAX);
                    let v3 = rng.gen_range(0..u8::MAX);

                    //let ind : usize = (usize::from(v1) * 256 * 256) + (usize::from(v2) * 256) + usize::from(v3);

                    //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                    unsafe {
                        //(*raw_v.0)[outer][usize::from(ind)] = [v1, v2];
                        //(*raw_v.0)[ind] = [v2, v3];
                        (*raw_v.0)[not_rand] = [v2, v3];
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 300_000..300_005 {
        //0..255 {
        println!("{:x} {:x} {:x} ", (i % 256), v[i][0], v[i][1]);
    }

    /*
    */
    let start = SystemTime::now();

    /*
            v.par_sort(); //too big to sort the whole thing at once on my VM.
        sort_utils::end_and_print_time(start, "sorted vector via rayon.");
    */

    /*
        let unit_step = 256*256*256; //256*256/16;

       let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(unit_step) {
                scope.spawn(move |_| {
                    //let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                    slice.sort();

                });
            }
        });

        sort_utils::end_and_print_time(start, "sort bags.");


        let unit_step = 256*256*64; //256*256/16;

       let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(unit_step) {
                scope.spawn(move |_| {
                    //let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                    slice.sort();

                });
            }
        });

        sort_utils::end_and_print_time(start, "sort bags.");


        let unit_step = 256*256*16; //256*256/16;

       let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(unit_step) {
                scope.spawn(move |_| {
                    //let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                    slice.sort();

                });
            }
        });

        sort_utils::end_and_print_time(start, "sort bags.");


        let unit_step = 256*256*8; //256*256/16;

       let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(unit_step) {
                scope.spawn(move |_| {
                    //let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                    slice.sort();

                });
            }
        });

        sort_utils::end_and_print_time(start, "sort bags.");
    */

    v.sort();
    sort_utils::end_and_print_time(start, "sorted vector via system sort.");
    //v.par_sort(); //too big to sort the whole thing at once on my VM.
    //sort_utils::end_and_print_time(start, "sorted vector via rayon.");

    // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
    for i in 300_000..300_005 {
        //0..255 {
        println!("{:x} {:x} {:x} ", (i % 256), v[i][0], v[i][1]);
    }

    /*
            // print to get a sense of correctness only; i.e., is the intended effect apparent? yes.
            for i in 1_000_000_000..1_000_000_010 {
                        println!("{:x} {:x} {:x} {:x} ", v[i][0], v[i][1], v[i][2], v[i][3]);
            }

    */

    // how many filled...
    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count: u64 = 0;
    println!("v.len {} ", v.len(),);
    for i in 0..sz_l1 * sz_l2 * sz_l3 * sz_l4 {
        if v[i] != [0, 0] {
            count = count + 1;
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * sz_l4,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * 256) as f64 * 100.0) as f64
    );

    /*
        // parallel NOTbinary searches...now they're just array lookups.
        let start = SystemTime::now();
        // search for a bunch.
        //let mut sought = 0;
        //let mut found = 0;
        //for last in 0..sz_l3 {

        let _ = crossbeam::scope(|scope| {
            for l2 in 0..sz_l1 {
                //
                scope.spawn(move |_| {
                    //println!("\tDEBUG: notice move appears to copy the pointer (different values now): {:p}", &raw_v);

                    let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                    //println!("\tDEBUG: move also obtains a (sort of) thread id: {:?}.", l2);

                    let mut total_sought = 0;
                    let mut total_found = 0;
                    let mut rng = ChaCha8Rng::from_entropy(); //works

                    for m in 0..sz_l2 {
                        for inmid in 0..sz_l3 {
                                                for inner in 0..256 {

                            let a = rng.gen_range(1..sz_l1);
                            let b = rng.gen_range(1..sz_l2);
                            let c = rng.gen_range(1..sz_l3);
                            let d = rng.gen_range(1..256);

                            unsafe {
                                                            /*
                                                            //search by index in order:
                                                            if 	(*raw_v.0)[l2][m][inmid][inner] {
                                                                total_found = total_found + 1;
                                                            }
                                                            */
                                                            //search by random index:
                                                            if 	(*raw_v.0)[a][b][c][d] {
                                                                total_found = total_found + 1;
                                                            }
                            }
                            total_sought = total_sought + 1;
                                                }
                        }
                    }

                    println!(
                        "\tDEBUG: thread {} - found {} out of searched {}",
                        l2, total_found, total_sought
                    );
                    //sought = sought + total_sought;
                    //found = found + total_found;
                });
            } //end outer loop
        });

        //println!("total: found {} out of searched {}", found, sought);

        sort_utils::end_and_print_time(start, "binary searches.");
    */
}

#[test]
fn t_interesting_flat() {
    let sz_l1 = 256; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    let mut v: Vec<[u8; 2]> = vec![[0; 2]; sz_l1 * sz_l2 * sz_l3 * sz_l4];

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    println!("v.len {} v[i].len {} ", v.len(), v[0].len(),);

    let raw_v: WrapperOneArr2 = WrapperOneArr2(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          //let rng = fastrand::Rng::new();

                //let mut rng = XorShiftRng::from_entropy();

                //randomly fill (some) of the structure.
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[2];
                    let v2 = r_bytes[3];
                    /*
                    let v1 = rng.gen_range(0..u8::MAX);
                    let v2 = rng.gen_range(0..u8::MAX);
                    */

                    let index = outer * sz_l2 * sz_l3 * sz_l4 + not_rand;

                    unsafe {
                        (*raw_v.0)[index] = [v1, v2];
                    }
                } //end not_rand
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    //nice but unnecessary step - gives a good sense at how much we're filling

    let start = SystemTime::now();

    let mut count: u64 = 0;
    for i in 0..sz_l1 * sz_l2 * sz_l3 * sz_l4 {
        if v[i] != [0, 0] {
            count = count + 1;
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * sz_l4,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * sz_l4) as f64 * 100.0) as f64
    );
    //end inspect step.

    // sort step - three natural options. par_sort, sort, and crossbeam slices.

    //v.par_sort();  //can't run this on my VM. the 9.3G grows to 10.7 and eats into my swap space until the program terminates.
    //v.sort();  //56s.

    /*
    // this is awesome to get me close, but it doesn't complete the sort.
        let start = SystemTime::now();

            let chunk_size = 256*256*256; //256*256/16;

            let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(chunk_size) {
                scope.spawn(move |_| {
                                    slice.sort();
                });
            }
        });
        sort_utils::end_and_print_time(start, "sort bags.");
    */

    let start = SystemTime::now();
    //v.sort();  //kills VM.
    //v.sort_unstable();  works, but takes an additionally 148s.
    v.par_sort_unstable();
    sort_utils::end_and_print_time(start, "complete sorted runs sort.");

    // confirm the sorting. not strictly necessary each time but worthwhile if i intend to change my methodology.
    let start = SystemTime::now();

    //for i in 0..sz_l1 * sz_l2 * sz_l3 * sz_l4 {}
    let rslt = sort_utils::is_sorted(&v);
    if !rslt {
        println!("\t\tthis isn't sorted....");
    }

    /* // this only confirms sorted slices.
            let _ = crossbeam::scope(|scope| {
            for slice in v.chunks_mut(chunk_size) {
                scope.spawn(move |_| {
                                let rslt = sort_utils::is_sorted_slice(slice); // still uses enough extra mem to be difficult.
                                if !rslt {
                                    println!("\t\tthis isn't sorted....");
                                }
                });
            }
        });
    */
    sort_utils::end_and_print_time(start, "confirmed sorted. ");

    // parallel NOTbinary searches...now they're just array lookups.
    let start = SystemTime::now();

    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                //let mut rng = XorShiftRng::from_entropy();

                //randomly look for some elements in the structure..
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[2];
                    let v2 = r_bytes[3];

                    let mut _answer = Ok(0); //default.
                    unsafe {
                        _answer = (*raw_v.0).binary_search(&[v1, v2]);
                    }
                    total_sought = total_sought + 1;
                    match _answer {
                        Ok(_index) => {
                            total_found = total_found + 1;
                        }
                        Err(_some_error) => (), //println!("not found. index where should be: {}", _some_error),
                    }
                } //end for not_rand

                /*
                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    outer, total_found, total_sought
                );
                                */
            });
        } //end outer loop
    });

    sort_utils::end_and_print_time(start, "binary searches.");
}

#[test]
fn t_interesting_one_level() {
    let sz_l1 = 256; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    //let mut v = create_2566(sz_l1, sz_l2, sz_l3, sz_l4);
    //let mut v = create_2565(sz_l1, sz_l2, sz_l3, sz_l4);
    //let mut v = create_2564(sz_l1, sz_l2, sz_l3, sz_l4);
    //let mut v = create_2563(sz_l1, sz_l2, sz_l3, sz_l4);  //on the cusp of too big...
    let mut v = create_2562(sz_l1, sz_l2, sz_l3, sz_l4);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count: u64 = 0;
    println!("v.len {} v[i].len {} ", v.len(), v[0].len(),);

    let raw_v: WrapperBag2562 = WrapperBag2562(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          //let rng = fastrand::Rng::new();

                //let mut rng = XorShiftRng::from_entropy();
                //rng.next_u64();

                //randomly fill (some) of the structure.
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    // very effective
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[0];
                    let v2 = r_bytes[1];
                    let v3 = r_bytes[2];
                    /* // not faster and NOT a better distribution. that doesn't seem right but might be due to having to use 251 as the largest prime less than 256. using 255 doesn't have the effect i expect either. might want to revisit.
                    let v1 = modular_hash((outer * not_rand).to_be_bytes());
                    let v2 = modular_hash((outer + not_rand).to_be_bytes());
                    let v3 = modular_hash((outer - not_rand).to_be_bytes());
                    */
                    /*
                                                let v1 = rng.gen_range(0..u8::MAX);
                    let v2 = rng.gen_range(0..u8::MAX);
                    let v3 = rng.gen_range(0..u8::MAX);  */
                    /*
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[0];
                    let v2 = r_bytes[1];
                    let v3 = r_bytes[2];
                    */
                    //let v1 = fastrand::u8(..);
                    //let v2 = fastrand::u8(..);
                    //let v3 = fastrand::u8(..);
                    //let ind : usize = (usize::from(v1) * 256 * 256) + (usize::from(v2) * 256) + usize::from(v3);

                    //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                    unsafe {
                        //(*raw_v.0)[outer][usize::from(ind)] = [v1, v2];
                        //(*raw_v.0)[ind] = [v2, v3];
                        (*raw_v.0)[usize::from(v1)][not_rand] = [v2, v3];
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    /*
     //nice but unnecessary step - gives a good sense at how much we're filling

        let start = SystemTime::now();
        for i in 0..sz_l1 {
                for j in 0..sz_l2 * sz_l3 * sz_l4 {
                        if v[i][j] != [0, 0] {
                            count = count + 1;
              }
                }
        }

        sort_utils::end_and_print_time(start, "inspect: how many filled.");
        println!(
            "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
            count,
            sz_l1 * sz_l2 * sz_l3 * sz_l4,
            (count as f64 / (sz_l1 * sz_l2 * sz_l3 * sz_l4) as f64 * 100.0) as f64
        );
    */
 //end inspect step.

    let start = SystemTime::now();

    let _ = crossbeam::scope(|scope| {
        //for outer in 0..sz_l1 {
        let inner_sz = 64;
        for outer in 0..4 {
            //four hyper threads on my vm.
            // create a thread for the outer number of elements.
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                for inner in 0..inner_sz {
                    let index = outer * inner_sz + inner;
                    unsafe {
                        //println!("(*raw_v.0)[outer] {}", (*raw_v.0)[outer].len());
                        //(*raw_v.0)[outer].par_sort(); //this combination appears to be a ridiculously fast sort. is it possible? crashes my VM.
                        //(*raw_v.0)[index].par_sort(); //this combination appears to be a ridiculously fast sort. is it possible? crashes my VM.
                        (*raw_v.0)[index].sort(); // still uses enough extra mem to be difficult.
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "sort bags.");

    /* // confirm the sorting. not strictly necessary each time but worthwhile if i intend to change my methodology.
        let start = SystemTime::now();
            //v.par_sort(); //too big to sort the whole thing at once on my VM.

        let _ = crossbeam::scope(|scope| {
            //for outer in 0..sz_l1 {
                    let inner_sz = 64;
            for outer in 0..4 {
                // create a thread for the outer number of elements.
                scope.spawn(move |_| {
                    let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                                    for inner in 0..inner_sz {
                                            let index = outer * inner_sz + inner;
                        unsafe {
                                                println!("attempting to sort index {}", index);
                                                let rslt = sort_utils::is_sorted(&(*raw_v.0)[index]); // still uses enough extra mem to be difficult.
                                                if !rslt {
                                                    println!("\t\tthis isn't sorted....");
                                                }
                                            }
                                    }
                });
            }
        });



        sort_utils::end_and_print_time(start, "confirmed sorted. ");

    */

    /*
     // how many filled...
        let start = SystemTime::now();

        // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
        let mut count : u64 = 0;
        println!(
            "v.len {} ",
            v.len(),
        );
        for i in 0..sz_l1 * sz_l2 * sz_l3 * sz_l4 {
                        if v[i] != [0, 0] {
                            count = count + 1;
              }
        }

        sort_utils::end_and_print_time(start, "inspect: how many filled.");
        println!(
            "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
            count,
            sz_l1 * sz_l2 * sz_l3 * sz_l4,
            (count as f64 / (sz_l1 * sz_l2 * sz_l3 * 256) as f64 * 100.0) as f64
        );

    */

    // parallel NOTbinary searches...now they're just array lookups.
    let start = SystemTime::now();

    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            //
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                //let mut rng = XorShiftRng::from_entropy();

                //randomly look for some elements in the structure..
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[0];
                    let v2 = r_bytes[1];
                    let v3 = r_bytes[2];

                    let mut _answer = Ok(0); //default.
                    unsafe {
                        _answer = (*raw_v.0)[usize::from(v1)].binary_search(&[v2, v3]);
                    }
                    total_sought = total_sought + 1;
                    match _answer {
                        Ok(_index) => {
                            total_found = total_found + 1;
                        }
                        Err(_some_error) => (), //println!("not found. index where should be: {}", _some_error),
                    }
                } //end for not_rand

                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    outer, total_found, total_sought
                );
            });
        } //end outer loop
    });

    sort_utils::end_and_print_time(start, "binary searches.");
}

#[test]
fn t_modular_hash() {
    for i in 10_000..10_010 {
        let x = modular_hash((i as u64).to_be_bytes());
        println!("x is: {}", x);
    }
}

#[test]
fn t_interesting_two_levels() {
    let sz_l1 = 256; //
    let sz_l2 = 256;
    let sz_l3 = 256;
    let sz_l4 = 256;

    let start = SystemTime::now();

    let mut v = create_25622(sz_l1, sz_l2, sz_l3, sz_l4);

    sort_utils::end_and_print_time(start, "created data structure.");

    let start = SystemTime::now();

    // give the user a sense of how many values we filled in by reporting how many nonzero values we find.
    let mut count: u64 = 0;
    println!(
        "v.len {} v[i].len {} v[i][i].len {} ",
        v.len(),
        v[0].len(),
        v[0][0].len(),
    );

    let raw_v: WrapperBag25622 = WrapperBag25622(&mut v);

    // crossbeam scoped threads let rust know the lifetime of the threads is over before main.
    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.
                                   //initialize the rng once only outside the loop - for speed.
                let mut rng = ChaCha8Rng::from_entropy(); //works
                                                          //let rng = fastrand::Rng::new();
                                                          //let mut rng = XorShiftRng::from_entropy();

                //randomly fill (some) of the structure.
                for inner in 0..sz_l2 {
                    for not_rand in 0..sz_l3 * sz_l4 {
                        // random values:
                        // very effective
                        let r = rng.next_u32();
                        let r_bytes = r.to_be_bytes();
                        let v1 = r_bytes[0];
                        let v2 = r_bytes[1];
                        let v3 = r_bytes[2];
                        let v4 = r_bytes[3];

                        let index = inner * sz_l2 + not_rand;
                        //assign random values in the Bag randomly. allows overwriting. acceptable bc the focus is on the technique of creation, sort, and search using rust and these crates.
                        unsafe {
                            (*raw_v.0)[usize::from(v1)][usize::from(v2)][not_rand] = [v3, v4];
                        }
                    } //end not_rand
                } //end inner
            });
        }
    });

    sort_utils::end_and_print_time(start, "filled Bag w/ random values.");

    //nice but unnecessary step - gives a good sense at how much we're filling

    let start = SystemTime::now();
    for i in 0..sz_l1 {
        for j in 0..sz_l2 {
            for k in 0..sz_l3 * sz_l4 {
                if v[i][j][k] != [0, 0] {
                    count = count + 1;
                }
            }
        }
    }

    sort_utils::end_and_print_time(start, "inspect: how many filled.");
    println!(
        "\tDEBUG: filled in {} values randomly out of {} possible. percent: {}",
        count,
        sz_l1 * sz_l2 * sz_l3 * sz_l4,
        (count as f64 / (sz_l1 * sz_l2 * sz_l3 * sz_l4) as f64 * 100.0) as f64
    );
    //end inspect step.

    let start = SystemTime::now();

    let _ = crossbeam::scope(|scope| {
        //for outer in 0..sz_l1 {
        let inner_sz = 64;
        for outer in 0..4 {
            //four hyper threads on my vm.
            // create a thread for the outer number of elements.
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                for inner in 0..inner_sz {
                    let index = outer * inner_sz + inner;

                    for each in 0..sz_l2 {
                        //
                        unsafe {
                            //println!("(*raw_v.0)[outer] {}", (*raw_v.0)[outer].len());
                            //(*raw_v.0)[outer].par_sort(); //this combination appears to be a ridiculously fast sort. is it possible? no. see i was only sorting four of the arrays in outer. at first, crashes my VM.
                            //(*raw_v.0)[index].par_sort(); //
                            (*raw_v.0)[index][each].sort(); // still uses enough extra mem to be difficult.
                        }
                    }
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "sort bags.");

    // confirm the sorting. not strictly necessary each time but worthwhile if i intend to change my methodology.
    let start = SystemTime::now();
    //v.par_sort(); //too big to sort the whole thing at once on my VM.

    let _ = crossbeam::scope(|scope| {
        //for outer in 0..sz_l1 {
        let inner_sz = 64;
        for outer in 0..4 {
            // create a thread for the outer number of elements.
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                for inner in 0..inner_sz {
                    let index = outer * inner_sz + inner;
                    for each in 0..sz_l2 {
                        //
                        unsafe {
                            //println!("confirming each {}", each);
                            let rslt = sort_utils::is_sorted(&(*raw_v.0)[index][each]); // still uses enough extra mem to be difficult.
                            if !rslt {
                                println!("\t\tthis isn't sorted....");
                            }
                        } // end unsafe
                    } //end each
                }
            });
        }
    });

    sort_utils::end_and_print_time(start, "confirmed sorted. ");

    // parallel NOTbinary searches...now they're just array lookups.
    let start = SystemTime::now();

    let _ = crossbeam::scope(|scope| {
        for outer in 0..sz_l1 {
            //
            scope.spawn(move |_| {
                let raw_v = raw_v; //https://users.rust-lang.org/t/unsafe-raw-pointer-in-std-thread/97705 . disjoint capture.

                let mut total_sought = 0;
                let mut total_found = 0;
                let mut rng = ChaCha8Rng::from_entropy(); //works

                //let mut rng = XorShiftRng::from_entropy();

                //randomly look for some elements in the structure..
                for not_rand in 0..sz_l2 * sz_l3 * sz_l4 {
                    // random values:
                    let r = rng.next_u32();
                    let r_bytes = r.to_be_bytes();
                    let v1 = r_bytes[0];
                    let v2 = r_bytes[1];
                    let v3 = r_bytes[2];
                    let v4 = r_bytes[3];

                    let mut _answer = Ok(0); //default.
                    unsafe {
                        _answer =
                            (*raw_v.0)[usize::from(v1)][usize::from(v2)].binary_search(&[v3, v4]);
                    }
                    total_sought = total_sought + 1;
                    match _answer {
                        Ok(_index) => {
                            total_found = total_found + 1;
                        }
                        Err(_some_error) => (), //println!("not found. index where should be: {}", _some_error),
                    }
                } //end for not_rand

                println!(
                    "\tDEBUG: thread {} - found {} out of searched {}",
                    outer, total_found, total_sought
                );
            });
        } //end outer loop
    });

    sort_utils::end_and_print_time(start, "binary searches.");
}
