// mod utils;
mod dandc;

fn main() {
    // let mut arr = utils::gen_random_array(100000, 0, 100000);
    // utils::benchmark_sort(utils::selection_sort, &mut arr);
    // utils::benchmark_sort(utils::merge_sort, &mut arr);
    let mut a = vec![1, 2, 3, 4];
    let mut b = vec![1, 2, 3, 4];
    let mut c = vec![0; 4];
    dandc::matmul_recursive(&mut a, &mut b, &mut c, 2);
    println!("{:?}", c);
    // let mut a = vec![1, 2, 3, 4];
    // let mut b = vec![5, 6, 7, 8];
    // let mut c = vec![0; 4];
    // dandc::matmul_strassen(&mut a, &mut b, &mut c, 2);
    // println!("{:?}", c);
    // let mut a = vec![1, 2, 3, 4];
    // let mut b = vec![5, 6, 7, 8];
    // let mut c = vec![0; 4];
    // dandc::matmul_strassen_parallel(&mut a, &mut b, &mut c, 2);
    // println!("{:?}", c);
    // let mut a = vec![1, 2, 3, 4];
    // let mut b = vec![5, 6, 7, 8];
    // let mut c = vec![0; 4];
    // dandc::matmul_strassen_parallel2(&mut a, &mut b, &mut c, 2);
    // println!("{:?}", c);
    // let mut a = vec![1, 2, 3, 4];
    // let mut b = vec![5, 6, 7, 8];
    // let mut c = vec![0; 4];
    // dandc::matmul_strassen_parallel3(&mut a, &mut b, &mut c, 2);
    // println!("{:?}", c);
    // let mut a = vec![1, 2, 3, 4];
    // let mut
}
