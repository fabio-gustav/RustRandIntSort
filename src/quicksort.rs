use std::isize;

//Public function to start a sort
//Returns the sorted array
pub fn sort(array: &mut [i32]) {
    let start = 0;
    let end = array.len() - 1;
    quick_sort_part(array,start,end as isize)
}

// Quicksort partition function
fn quick_sort_part(array: &mut [i32],start:isize,end:isize) {
    if start < end {
        let pivot = part(array, start as isize, end as isize);
        quick_sort_part(array, start,pivot-1);
        quick_sort_part(array, pivot+1, end);
    }
}

//Rest of the quicksort partition
fn part(array: &mut [i32], left: isize, right: isize) -> isize {
    let pivot = right; //Pivot at right end
    let mut i = left as isize - 1;
    

    for j in left..=right-1 {

        if array[j as usize] < array[pivot as usize] {
            i+=1;
            array.swap(i as usize, j as usize);
        }
    }


    array.swap((i+1) as usize , pivot as usize);

    return i+1;

}
