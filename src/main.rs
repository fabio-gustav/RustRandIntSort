use rand::prelude::*;



fn main() {
    println!("Sorting Program Example");

    println!("Generating Numbers to Sort");
    let mut rng = thread_rng();
    

    let mut sorting_list: Vec<i32> = Vec::new();

    for _i in 1..500 {
        sorting_list.push(rng.gen_range(1..=500));
    }

    println!("List Generated")

    
}
