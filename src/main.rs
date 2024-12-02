use rand::prelude::*;
mod quicksort;



fn main() {
    println!("Sorting Program Example");

    println!("Generating Numbers to Sort");
    let mut rng = thread_rng();
    

    let mut sorting_list: Vec<i32> = Vec::new();

    for _i in 1..500 {
        sorting_list.push(rng.gen_range(1..=500));
    }

    
    println!("List Generated");
    let mut sorting_list1: Vec<i32> = vec![6,5,4,3,2,1];
    quicksort::sort(&mut sorting_list1);
    

    for i in sorting_list{
        println!("{}",i);
    }
}


