use rand::prelude::*;
mod quicksort;

use std::fs;
use std::io::Write;

fn main() {
    println!("Sorting Program Example");

    println!("Generating Numbers to Sort");
    let mut rng = thread_rng();

    //creates a file of 500 unsorted integers, each ranging from 1-500 inclusive
    //feel free to comment this block out (lines 15-20 inclusive) if you have a custom list of integers that need to be sorted
    let mut input_file: fs::File = fs::File::create("unsorted.txt").expect("Could not create file");
    for _i in 0..500 {
        //generates an integer and writes it to a new line in the unsorted.txt file
        write!(input_file, "{}\n", rng.gen_range(1..=500)).expect("Could not write to file");
    }
    println!("List Generated");


    //reads a file of unsorted integers
    let mut sorting_list:Vec<i32> = Vec::new();
    for line in fs::read_to_string("unsorted.txt").unwrap().lines(){
        //converts each line (currently a string slice) to a interger and adds it to the vector of nums to sort
        sorting_list.push(line.parse::<i32>().expect("Could not parse"));
    }

    println!("Unsorted file created");

    //sorts the integers
    quicksort::sort(&mut sorting_list);

    //creates an output file
    let mut output_file: fs::File = fs::File::create("sorted.txt").expect("Could not create file");
    
    //writes the sorted integers to the file each on an individual line
    for num in &sorting_list{
        write!(output_file, "{}\n", num).expect("Could not write to file");
    }

    println!("Sorted list created!");

}
