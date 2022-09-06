pub mod timers;
pub mod sorting_algo;
use crate::timers::*;
use crate::sorting_algo::*;

fn main() {
    // let result = current_time_as_micros();
    //     println!("{:?}", result);
    //     let result = current_time_as_secs();
    //     println!("{:?}", result);
    //     let result = current_time_as_micros();
    //     println!("{:?}", result);
    let mut vecs = vec![4,3,2,1];
    let cur = hill_sort(&mut vecs, true).to_vec();
    println!("{:?}", &cur);
    println!("{:?}", &vecs);
}
