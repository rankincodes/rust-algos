use rand::Rng;
use std::time::Instant;

pub fn gen_random_array(n: usize, range_l: i32, range_r: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(rng.gen_range(range_l..range_r));
    }
    arr
}

pub fn benchmark_sort<F>(sort: F, arr: &mut [i32])
where
    F: Fn(&mut [i32]),
{
    let start = Instant::now();
    sort(arr);
    let duration = start.elapsed();
    println!("Time elapsed in sort function: {:?}", duration);
}
