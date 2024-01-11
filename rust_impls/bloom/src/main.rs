use std::thread;

use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::sync::Arc;
use std::sync::Mutex;

use std::time::SystemTime;

//use rayon::slice::ParallelSliceMut;

#[derive(Clone, Copy)]
struct WrapperBloomBits(*mut Vec<u64>);

unsafe impl Send for WrapperBloomBits {}
unsafe impl Sync for WrapperBloomBits {}

fn main() {
    t_bloom_bits();
}

//idea:
//'hash' the eight byte value from three or more starting positions and use bloom filter style
//analysis to determine if an element exists.
//no more need for data structures except to hold the filter, which is an array of boolean flags.
//aside: afraid this might not work bc the base array isn't big enough. but let's see. i can always
//make it bigger and make the hashes more complicated.
// and i think i'll need three steps: base sets flags. compare computes flags and checks if they're
// in the array created by base. if they are, save the values used to compute. then i have to go
// back to the base generation to find the values used to generate the same flags. justification: compute is
// fast and i don't have to store all the generating values this way.
// anyway, enough of the idea - here we go:
// returns a very small (hopefully) bag of possible matches.

/**************
 * ***************/

const fn get_prime_tbl_sz() -> usize {
    23
}

//#[test]
fn t_bloom_bits() {
    //let cap = 8589934609; // 8 times too big, since i'm not building a vec of bools (one byte) anymore.
    // i can have a bloom filter of bits that is 8 times bigger than one of bytes.
    // this gets confusing since the array size and unit representing a bool both scale up and down
    // simultaneously. this scheme allows me to use 1/8th the len of a data structure but get me 8
    // times more accessible memory location than that too - so effectively netting 64 times more
    // storage for the same memory size. anyway, refine this, it's a general idea. a rust bool is
    // exactly one byte.
    // e.g., if i had the usual representation that a bool takes up a byte, then eight bytes is
    // eight flags i can set. BUT if i use a u64 and a bit array method from cs.emory.edu, i have
    // 64 positions instead of eight. the new scheme allows me to, e.g., pick 100 u64 integers and
    //    index 64 positions into them for a total of 6400.
    //    http://www.cs.emory.edu/~cheung/Courses/255/Syllabus/1-C-intro/SLIDES/t19.html
    //    then i need the next largest prime after the n_bit_cap, not the n_u64s_cap
    //    https://algs4.cs.princeton.edu/34hash/
    // so, to use 8GB of RAM and get 8*64 bits that can indiciate the presence of a hash hit:
    // u64, so 8 bytes * 1B or 2**30. o
    // >>> 2**30
    // 1073741824 is a GB. 8 of them. there are 64 times the 1.07B number of fields.
    // maybe use the next largest prime here...no. need the next largest prime over my goal
    // capacity and need to adjust capacity to hold that:
    // 68_719_476_736
    // next largest prime is 68_719_476_767 which means i need one more u64 to hold those extra
    // values.
    // 1_073_741_825 instead of ..24.

    /*
    // scenario 1: 8GB table, so 1GB or 2**30 entries that are 64 bits or 8 bytes.
    //  this allows 68 billion addressable entries. i.e., n is 68B.
    //
    let n_u64s_cap = 1_073_741_825;
    let n_prime_cap = 68_719_476_767;
    */

    // scenario 2: 12GB table (biggest on VM) so 1.5GB, then
    let n_u64s_cap: usize = 1_610_612_737; //...736 plus one more to fit the extra up to prime.
    let n_prime_cap: usize = 103_079_215_111; //  103_079_215_104;

    let n_threads = 128; //2; //128; //64;//128; //2; //128; //64; //64; //32; //4; //16; //8; //4;
                         //
    println!("{} threads used.", n_threads);

    // change the fn above get_prime_tbl_sz when you want to add values
    //let prime_tbl = [ 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101 ];
    //let prime_tbl = [ 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997 ];
    //let prime_tbl = [ 23929, 41299, 23041, 28429, 43159, 17137, 31039, 20543, 10247, 16417, 42209, 47609, 28753, 18517, 48413, 19273, 36451, 12959, 39113, ];
    //pretty good.
    //let prime_tbl = [ 47507, 47963, 46933, 4783, 35117, 14767, 13103, 40529, 44087, 18787, 26099, 15299, 48353, 13171, 43913, 15647, 42611, 3517, 21277, ];
    //let prime_tbl = [ 30713, 47881, 10711, 47507, 12343, 42967, 6701, 4967, 28201, 32143, 19687, 19157, 8317, 31019, 40127, 20507, 29059, 7691, 20063, 29663, 29483, 40361, 39827 ];
    //i get the primes from https://onlinetools.com/random/generate-random-primes and use:
    // sort -n < primes.txt | sed -e 's/$/,/' | tr -d '\n'
    let prime_tbl = [
        2333, 5077, 5623, 6899, 7717, 8599, 10099, 10753, 11903, 12689, 13339, 15551, 18089, 19447,
        22783, 24281, 29641, 32621, 35999, 36761, 39157, 42937, 49789,
    ]; //23
       /*let prime_tbl = [
           2333, 5077, 6899, 7717, 8599, 12689, 15551, 18089, 19447,
           22783, 29641, 32621, 35999, 36761, 39157, 42937, 49789,
       ]; //17 */
    /*let prime_tbl = [
        2333, 5077, 7717,  12689, 18089, 19447,
        22783, 29641, 32621, 35999, 39157, 42937, 49789,
    ]; // 13 */
    /*
    let prime_tbl = [
        2333, 7717, 12689, 19447,
        22783, 29641, 32621, 42937, 49789,
    ]; //9 */
    /*let prime_tbl = [
        2333, 19447,
        22783, 32621, 49789,
    ]; //5*/
    /* let prime_tbl = [
        2333, 22783, 49789,
    ]; //3 */
    /*let prime_tbl = [
        2333,22783
    ]; //2 */

    const K_HASHES: usize = get_prime_tbl_sz();
    //let prime_tbl : Vec<u32> = vec![ 7, 11, 13, 17, 19 ];

    let bloom_filter = create_bloom_bits(n_u64s_cap, n_prime_cap, n_threads, prime_tbl);

    //println!("bloom_filter len {}, so i expect {} memory locations", bloom_filter.len(), 64* bloom_filter.len());
    let byte_range = 256_usize;
    let hash_tbl_sz = 64 * bloom_filter.len();
    let sz_set = n_threads * byte_range.pow(3);
    println!("n (hash table size): {}", hash_tbl_sz);
    println!("m (size of set): {}", sz_set);
    let c = hash_tbl_sz as f64 / sz_set as f64;
    println!("c = n/m: {}", c);
    //println!("false hit prob: {}", f64::powi(0.6185_f64, c as i32)); //.powf(c as f64));
    println!(
        "false hit prob (assuming optimal k): {}",
        f64::powf(0.6185_f64, c)
    ); //.powf(c as f64));

    let k = c * 2_f64.ln();
    println!("optimal k value (k=cln(2)) is: {}", k);
    println!("actual k value: {}", K_HASHES);

    let hits_bag = compare_to_exists_bits(&bloom_filter, n_prime_cap, n_threads, prime_tbl);
    let total_hits = hits_bag.len();

    let mut total_set_bits = 0;
    //for (i,filter_u64) in bloom_filter.iter().enumerate() {
    for filter_u64 in bloom_filter.iter() {
        let each_count = filter_u64.count_ones();
        total_set_bits += each_count;
    }
    println!("total set: {}", total_set_bits);
    println!(
        "percent hits/set_bits: {}",
        (100 * total_hits) as f64 / total_set_bits as f64
    );

    if total_hits > 0 {
        println!("these are my hits:");
        for h in &hits_bag {
            println!("\t{}", h);
        }

        //no real value as is - takes a while.
        match_hits(hits_bag, n_threads);
    }
}

// takes a while, probably due to setting bits - but could compare crossbeam vs std thread impl.
// also, need to look at performance improvements generally.
fn create_bloom_bits(
    v_len_cap: usize,
    hash_table_sz: usize,
    n_threads: usize,
    prime_tbl: [u64; get_prime_tbl_sz()],
) -> Vec<u64> {
    let start = SystemTime::now();

    let byte_range: usize = 256; //4; //128;

    // i'd like to get c = n/m = 30. where n is the size of the hash table and m is the size of teh
    // the set.
    // in the above impl, i used a full byte for each bit as that's the natural programming
    // language paradigm for the bool flag, even in rust.
    // i need k = cln2 hash functions. for c =3 that's about two
    // hash fns.
    // >>> 0.6185**3 * (256**4)
    //1016199491.2350866
    //that's terrible, as it's over a billion false positives.
    //but 30 would take about ~128/129 gb and give 2361 false positives. 40 about 160gb and 20
    //false positives.
    //20 is 288k false positives. 80gb. 14 hashes
    //25 is 26088 and 100gb. over 17 hashes

    //ok, so what i can do on the vm: prime table size just bigger than 256**4 * 3.
    //12884901893 where 12884901888 is the product.
    //with that knowledge, let's use less memory on my VM. using all of it results in atrocious
    //performance. so, 256**4 * 2 = 8589934592; the next biggest prime is: 8589934609

    // the difference: instead of a vec of bools, it's a vec of u64s. and each u64 is viewed as a
    // bit string of len 64, so i get 8 times more capacity. but i don't know if it'll be slower.
    let mut bloom_filter: Vec<u64> = vec![0; v_len_cap];

    /*
    //cheat and put in a sentinel:
    let cheat : u64 = 77777777;
    const PT_SZ: usize = get_prime_tbl_sz();
    let mut hash_tbl = [0; PT_SZ];
    for i in 0..PT_SZ {
        hash_tbl[i] =
            cheat.wrapping_mul(prime_tbl[i] as u64) % hash_table_sz as u64;
    }
    for i in 0..PT_SZ {
        bloom_filter[hash_tbl[i] as usize / 64_usize] |=
        1_u64 << (hash_tbl[i] as usize % 64_usize);
    }
     */

    let wrapped_bf: WrapperBloomBits = WrapperBloomBits(&mut bloom_filter);

    let mut threads = vec![];
    //let prime_tbl = [ 7, 11, 13, 17, 19 ];
    //const k_hashes : usize  = 5;
    for _x in 0..n_threads {
        let mut rng = ChaCha8Rng::seed_from_u64(_x as u64 + 512_u64);
        threads.push(thread::spawn({
            move || {
                let wrapped_bf = wrapped_bf;

                //let mut rng = ChaCha8Rng::from_entropy();
                //let mut rng = ChaCha8Rng::from_seed([0x0; 32]);
                for _j in 0..byte_range {
                    for _k in 0..byte_range {
                        for _l in 0..byte_range {
                            let r = rng.next_u64();

                            const PT_SZ: usize = get_prime_tbl_sz();
                            let mut hash_tbl = [0; PT_SZ];

                            for i in 0..PT_SZ {
                                hash_tbl[i] =
                                    r.wrapping_mul(prime_tbl[i] as u64) % hash_table_sz as u64;
                            }
                            // don't call set bits but do it inline.
                            unsafe {
                                for i in 0..PT_SZ {
                                    (*wrapped_bf.0)[hash_tbl[i] as usize / 64_usize] |=
                                        1_u64 << (hash_tbl[i] as usize % 64_usize);
                                }
                            }

                            /* solid, but inflexible:
                                                        //mersenne primes: 31, 127, 8191, 131071, 524287
                                                        //let h1 = r * 48017 % cap as u64;
                                                        //let h1 = r * 31 % cap as u64;
                                                        // just too big to be worth it i think:
                                                        //let h1_big : u128 = (r * 524287) as u128 % hash_table_sz as u128;
                                                        //let h1 : u64 = h1_big as u64;
                                                        let h1 = r.wrapping_mul(127) % hash_table_sz as u64;
                                                        //let h2 = r * 23003 % cap as u64;
                                                        //let h2 = r * 127 % cap as u64;
                                                        let h2 = r.wrapping_mul(131071) % hash_table_sz as u64;
                                                        //let h3 = r * 12007 % cap as u64;
                                                        let h3 = r.wrapping_mul(8191) % hash_table_sz as u64;

                                                        // don't call set bits but do it inline.
                                                        let i_h1 : u64 = h1/64;
                                                        let pos_h1 : u64 = h1 % 64;
                                                        let i_h2 : u64 = h2/64;
                                                        let pos_h2 : u64 = h2 % 64;
                                                        let i_h3 : u64 = h3/64;
                                                        let pos_h3 : u64 = h3 % 64;

                                                        unsafe {
                                                            (*wrapped_bf.0)[i_h1 as usize] |= 1_u64 << pos_h1;
                                                            (*wrapped_bf.0)[i_h2 as usize] |= 1_u64 << pos_h2;
                                                            (*wrapped_bf.0)[i_h3 as usize] |= 1_u64 << pos_h3;
                                                        }
                            */
                            /*
                            unsafe {
                                set_bit(wrapped_bf.0), h1);
                                set_bit(wrapped_bf.0), h2);
                                set_bit(wrapped_bf.0), h3);
                            } //end unsafe
                            */
                        } //end l
                    } //end k
                } //end j
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }

    //println!("total values written: hashes: {} x iterations: {} = {}", n_hashes, n_threads * byte_range.pow(3), n_hashes * n_threads * byte_range.pow(3));
    //println!("total

    sort_utils::end_and_print_time(start, "filled in values...");

    bloom_filter
}

fn compare_to_exists_bits(
    v: &Vec<u64>,
    hash_table_sz: usize,
    n_threads: usize,
    prime_tbl: [u64; get_prime_tbl_sz()],
) -> Vec<u64> {
    let start = SystemTime::now();

    //    let n_threads : usize = 2; //8; // 3; //4; // 8; //16; //432s! 32; //too long: 256; //16; //6; //4; //2 is slow 3 faster than 2!; //16;
    let byte_range: usize = 256; //4; //128;

    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let hit_counter = Arc::new(Mutex::new(0));
    let hit_vector = Vec::new();
    let hit_bag = Arc::new(Mutex::new(hit_vector));

    let _ = crossbeam::scope(|scope| {
        for _i in 0..n_threads {
            let v = v;
            let mut rng = ChaCha8Rng::seed_from_u64(_i as u64);
            //this expected to get me past borrowed after move:
            let hit_counter = Arc::clone(&hit_counter);
            let hit_bag = Arc::clone(&hit_bag);
            scope.spawn(move |_| {
                //let mut rng = ChaCha8Rng::from_entropy();
                //let mut rng = ChaCha8Rng::from_seed([0xf; 32]);
                for _j in 0..byte_range {
                    for _k in 0..byte_range {
                        for _l in 0..byte_range {
                            let r = rng.next_u64();

                            /*
                            //cheat and put in a sentinel:
                            if _i == 0 && _j == 0 && _k == 0 && _l == 0 {
                                r = 77777777;
                            }
                            */

                            const PT_SZ: usize = get_prime_tbl_sz();
                            let mut hash_tbl = [0; PT_SZ];

                            for i in 0..PT_SZ {
                                hash_tbl[i] =
                                    r.wrapping_mul(prime_tbl[i] as u64) % hash_table_sz as u64;
                            }

                            let mut all_passed = true;
                            for i in 0..PT_SZ {
                                if !test_bit(v, hash_tbl[i]) {
                                    all_passed = false;
                                    break;
                                }
                            }
                            if all_passed {
                                let mut hb = hit_bag.lock().unwrap();
                                //&*hb.push(r);
                                hb.push(r);
                                let mut data = hit_counter.lock().unwrap();
                                *data += 1;
                            }

                            /*  appears solid.
                                                     //mersenne primes: 31, 127, 8191, 131071, 524287
                                                        let h1 = r.wrapping_mul(127) % hash_table_sz as u64;
                                                        let h2 = r.wrapping_mul(131071) % hash_table_sz as u64;
                                                        let h3 = r.wrapping_mul(8191) % hash_table_sz as u64;

                                                        if test_bit(v, h1) && test_bit(v, h2) && test_bit(v, h3) {
                                                            //println!("hit!");
                                                            let mut data = hit_counter.lock().unwrap();
                                                            *data += 1;
                                                        }
                            */
                        } //end l
                    } //end k
                } //end j
            });
        } //loop 0..nthreads
    }); //end crossbeam

    let total_hits = *hit_counter.lock().unwrap();
    println!("total hits: {:?}", total_hits);

    sort_utils::end_and_print_time(start, "compared random values...");

    //let bag = *hit_bag.lock().unwrap();
    let b = Arc::try_unwrap(hit_bag).unwrap();
    let b = b.into_inner().unwrap();
    b
    //bag
}

fn match_hits(mut v: Vec<u64>, n_threads: usize) {
    let start = SystemTime::now();

    v.sort();
    let byte_range: usize = 256; //4; //128;

    thread::scope(|s| {
        for _x in 0..n_threads {
            let mut rng = ChaCha8Rng::seed_from_u64(_x as u64 + 512_u64);
            let v = &v;
            s.spawn(move || {
                //want to move to copy i to each thread, giving it an index.
                //let mut rng = ChaCha8Rng::from_entropy();
                //let mut rng = ChaCha8Rng::from_seed([0x0; 32]);
                for _j in 0..byte_range {
                    for _k in 0..byte_range {
                        for _l in 0..byte_range {
                            let r = rng.next_u64();

                            if r == 11796645308897186027 {
                                println!("true match on \t {}\n i know for 128 threads and with these random seeds there will be exactly one true match. using this to vary on false positives by tweaking my number and character of primes.\n", r);
                            }

                            /*
                            if v.contains(&r) {
                                println!("true match on \t {}", r);
                            } */
                        } //end l
                    } //end k
                } //end j
            });
        }
        //}).unwrap(); //end crossbeam
    }); //end crossbeam

    sort_utils::end_and_print_time(start, "check for matches...");
}

//careful here - returning a full byte bool to tell me the bit is checked. awkward. but don't save
//that return value anywhere! only use it to make decisions.
fn test_bit(v: &Vec<u64>, k: u64) -> bool {
    let i: u64 = k / 64;
    let pos: u64 = k % 64;

    //println!("{:#064b}", v[i as usize]);
    if v[i as usize] & 1 << pos > 0 {
        return true;
    } else {
        return false;
    }
}

// used conceptually but not directly, except in unit tests. inlined instead.
fn _set_bit(v: &mut Vec<u64>, k: u64) {
    let i: u64 = k / 64;
    let pos: u64 = k % 64;

    //println!("index {}, position {}", i, pos);
    //println!("{}, {}", v[i as usize], 1_u64 <<pos);
    v[i as usize] |= 1_u64 << pos;
    //println!("{:#064b}", v[i as usize]);
}

#[test]
fn t_basic_set_bits() {
    let mut v: Vec<u64> = vec![0; 5];
    let h1 = 1;
    let h2 = 2;
    let h3 = 3;

    //set_bit(&mut v, 1);
    for i in 0..128 {
        if i % 2 == 0 {
            _set_bit(&mut v, i);
        }
    }
}

#[test]
fn t_basic_get_bits() {
    let mut v: Vec<u64> = vec![0; 5];

    //set_bit(&mut v, 1);
    for i in 0..128 {
        if i % 2 == 0 {
            _set_bit(&mut v, i);
        }
    }

    for i in 0..128 {
        if i % 2 == 0 {
            assert!(test_bit(&v, i));
        } else {
            assert!(!test_bit(&v, i));
        }
    }
}

//basic model for sharing read only structures between threads.
#[test]
fn t_share_ro_threads() {
    let var = vec![1, 2, 3];

    //soft deprecated i thiNK:
    //crossbeam_utils::thread::scope(|s| {
    // i don't think  this is crossbeam anymore...
    thread::scope(|s| {
        //copy the reference to var to be copied onto each thread; don't want to copy var to each
        //thread.
        let var = &var;
        for i in 0..3 {
            //give var reference to three different threads.
            s.spawn(move || {
                //want to move to copy i to each thread, giving it an index.
                println!("have a child thread {} borrowing var {:?}", i, var);
            });
        }
        //}).unwrap(); //end crossbeam
    }); //end thread scope
}

// why didn't i have to use wrapping multiply here...
#[test]
fn t_hash_experiments() {
    let mut rng = ChaCha8Rng::from_seed([0x0; 32]);
    let mut r = rng.next_u64();

    let cap: u64 = 68_719_476_767;
    //let cap : u64 = 12_884_901_893;
    //let h1 = r * 48017 % cap as u64;
    //let h1 = r * 859433 % cap as u64;
    let h1 = (r * 9999999999996972593) % cap as u64;
    //let h1 = r * 31 % cap as u64;
    let h2 = r * 23003 % cap as u64;
    //let h2 = r * 127 % cap as u64;
    let h3 = r * 12007 % cap as u64;
    //let h3 = r * 8191 % cap as u64;
    println!("r is {}, hash is: {} {} {} ", r, h1, h2, h3);
}

/*
 * The functions below represent a space inefficient bloom filter. This approach is easier to get
 * working first:
 *
 * */

#[derive(Clone, Copy)]
struct WrapperBloom(*mut Vec<bool>);

unsafe impl Send for WrapperBloom {}
unsafe impl Sync for WrapperBloom {}

#[test]
fn t_create_bloom_base() {
    //let bloom_filter = create_arc_bloom_base();
    let bloom_filter = _create_bloom_base();

    println!("bloom_filter len {}", bloom_filter.len());

    let th = _compare_to_exists(&bloom_filter);

    let mut total_not_set = 0;
    let mut total_set = 0;
    for (i, hash) in bloom_filter.iter().enumerate() {
        if *hash == true {
            //println!("index not filled: {}", i);
            total_set += 1;
        }
    }
    println!("total set: {}", total_set);
    println!("percent hits: {}", th as f64 / total_set as f64);
}

//fn create_arc_bloom_base() -> Vec<bool>  {
fn _create_bloom_base() -> Vec<bool> {
    // `Vec<String>` is wrapped inside a `Mutex` and `Arc`.
    // `Mutex` provides synchronization, `Arc` provides lifetime so each
    // thread participates in ownership over the `Mutex<Vec<String>>`

    let start = SystemTime::now();

    let n_threads: usize = 3; //4; // 8; //16; //432s! 32; //too long: 256; //16; //6; //4; //2 is slow 3 faster than 2!; //16;
    let byte_range: usize = 256; //4; //128;

    //terrible idea. c is too small
    //let cap = 4294967311; //first prime past 2^32 (or 256^4)

    // i'd like to get c = n/m = 30. where n is the size of the hash table and m is the size of teh
    // the set. but if the set is 4gb, since unfortunately i think i have to use one byte per bit (unless
    // i used string bits, maybe?) i can't try more than c = 2, possibly 3, on this vm.
    // but that makes for very few ks. i need k = cln2 hash functions. for c =3 that's about two
    // hash fns.
    // >>> 0.6185**3 * (256**4)
    //1016199491.2350866
    //that's terrible, as it's over a billion false positives.
    //but 30 would take about ~128/129 gb and give 2361 false positives. 40 about 160gb and 20
    //false positives.
    //20 is 288k false positives. 80gb. 14 hashes
    //25 is 26088 and 100gb. over 17 hashes

    //ok, so what i can do on the vm: prime table size just bigger than 256**4 * 3.
    //12884901893 where 12884901888 is the product.

    let cap = 12884901893;

    let n_hashes = 3;

    //realize: don't need an arc/mutex bc i'm only ever writing true. who cares if there's a race,
    //the outcome is the same if two are writing. these is an excellent condition.
    //let bloom_filter = Arc::new(Mutex::new(vec![false; cap]));
    let mut bloom_filter = vec![false; cap];
    let wrapped_bf: WrapperBloom = WrapperBloom(&mut bloom_filter);

    //can't do this anymore
    //println!("bloom_filter has size: {}", bloom_filter.len());

    let mut threads = vec![];
    for _x in 0..n_threads {
        threads.push(thread::spawn({
            //let clone = Arc::clone(&bloom_filter);

            move || {
                let wrapped_bf = wrapped_bf;

                let mut rng = ChaCha8Rng::from_entropy();
                for _j in 0..byte_range {
                    for _k in 0..byte_range {
                        for _l in 0..byte_range {
                            let r = rng.next_u64();

                            //let h1 = r * 48017 % cap as u64;
                            //let h1 = r * 31 % cap as u64;
                            let h1 = r * 524287 % cap as u64;
                            //let h2 = r * 23003 % cap as u64;
                            //let h2 = r * 127 % cap as u64;
                            let h2 = r * 131071 % cap as u64;
                            //let h3 = r * 12007 % cap as u64;
                            let h3 = r * 8191 % cap as u64;

                            unsafe {
                                (*wrapped_bf.0)[h1 as usize] = true;
                                (*wrapped_bf.0)[h2 as usize] = true;
                                (*wrapped_bf.0)[h3 as usize] = true;
                            } //end unsafe
                        } //end l
                    } //end k
                } //end j
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }

    println!(
        "total values written: hashes: {} x iterations: {} = {}",
        n_hashes,
        n_threads * byte_range.pow(3),
        n_hashes * n_threads * byte_range.pow(3)
    );
    sort_utils::end_and_print_time(start, "filled in values...");

    //1
    //bloom_filter.downcast::<Vec<bool>>()
    //2
    //let lock = Arc::try_unwrap(bloom_filter);
    //lock.into_inner()
    //3
    //let clone = Arc::clone(&bloom_filter);
    //clone.lock().unwrap()
    //4
    //let bf = Arc::try_unwrap(bloom_filter).unwrap();
    //let bf = bf.into_inner().unwrap();
    //bf
    //
    bloom_filter
}

fn _compare_to_exists(v: &Vec<bool>) -> usize {
    let start = SystemTime::now();

    let n_threads: usize = 3; //4; // 8; //16; //432s! 32; //too long: 256; //16; //6; //4; //2 is slow 3 faster than 2!; //16;
    let byte_range: usize = 256; //4; //128;

    let cap: u64 = 12884901893;

    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    //let data = Arc::new(Mutex::new(0));
    let hit_counter = Arc::new(Mutex::new(0));

    let _ = crossbeam::scope(|scope| {
        for _i in 0..n_threads {
            let v = v;
            //this expected to get me past borrowed after move:
            let hit_counter = Arc::clone(&hit_counter);
            //let (data, tx) = (Arc::clone(&data), tx.clone());
            scope.spawn(move |_| {
                let mut rng = ChaCha8Rng::from_entropy();
                for _j in 0..byte_range {
                    for _k in 0..byte_range {
                        for _l in 0..byte_range {
                            let r = rng.next_u64();

                            //let h1 = r * 48017 % cap as u64;
                            //let h1 = r * 31 % cap as u64;
                            let h1 = r * 524287 % cap as u64;
                            //let h2 = r * 23003 % cap as u64;
                            //let h2 = r * 127 % cap as u64;
                            let h2 = r * 131071 % cap as u64;
                            //let h3 = r * 12007 % cap as u64;
                            let h3 = r * 8191 % cap as u64;
                            if v[h1 as usize] && v[h2 as usize] && v[h3 as usize] {
                                //println!("hit!");
                                //let mut data = clone.lock().unwrap();
                                //count = (*data + 1);
                                //let mut data = data.lock().unwrap();
                                let mut data = hit_counter.lock().unwrap();
                                *data += 1;
                            }
                        } //end l
                    } //end k
                } //end j
            });
        } //loop 0..nthreads
    }); //end crossbeam

    //rx.recv().unwrap();
    //println!("total hit: {:?}", rx.recv().unwrap());
    //println!("total hit! {}", count);
    println!("total hit: {:?}", *hit_counter.lock().unwrap());

    /*
        let mut threads = vec![];
        for x in 0..n_threads {
            threads.push(thread::spawn({

                move || {
                    //let wrapped_bf = wrapped_bf;

                    let mut rng = ChaCha8Rng::from_entropy();
                    let mut write_cheat = true;
                    for j in 0..byte_range {
                        for k in 0..byte_range {
                            for l in 0..byte_range {
                                let r = rng.next_u64();
                                let r_bytes = r.to_be_bytes();
                                let s = rng.next_u32();
                                let s_bytes = r.to_be_bytes();
                                let v9 = r_bytes[0];
                                let v10 = r_bytes[1];
                                let v11 = r_bytes[2];
                                let v12 = r_bytes[3];

                                //let index: usize = v1 as usize;

                                let h1 = r * 13 % cap as u64;
                                let h2 = r * 19 % cap as u64;
                                let h3 = r * 31 % cap as u64;

                                //if &v[h1 as usize] && &v[h2 as usize] && &v[h3 as usize] {
                                if v[h1 as usize] && v[h2 as usize] && v[h3 as usize] {
                                    println!("hit!");
                                }

                            } //end l
                        } //end k
                    } //end j
                }
            }));
        }
        for t in threads {
            t.join().unwrap();
        }
    */
    sort_utils::end_and_print_time(start, "compared random values...");

    let total_hits = *hit_counter.lock().unwrap();
    total_hits
}

#[test]
fn find_overlap_rngs() {
    const LEN: usize = 1_000_000_000;

    let v: Vec<u64> = Vec::new();
    for i in 11..100 {
        let mut rng = ChaCha8Rng::seed_from_u64(i);
        for _ in 0..LEN {
            let r = rng.next_u64();
            /*
            if r % 100_000_000 == 0 {
                println!("found something memorable: {}", r);
            }*/
            if r == 559670132500000000 {
                println!("\n\tfound gold\n");
            }
            v.push(r);
        }
    }

    assert_eq!(v.len(), LEN);

    //v.par_sort();
}
