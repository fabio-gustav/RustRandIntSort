use std::isize;

pub fn sort(array: &mut [i32]) {
    let start = 0;
    let end = array.len() - 1;
    quick_sort_part(array,start,end as isize)
}


fn quick_sort_part(array: &mut [i32],start:isize,end:isize) {
    if start < end && end - start >= 1 {
        let pivot = part(array, start as isize, end as isize);
        quick_sort_part(array, start,pivot as isize - 1);
        quick_sort_part(array, start + 1, end);
    }
}

fn part(array: &mut [i32], left: isize, right: isize) -> isize {
    let pivot = array[left as usize];
    let mut i = left - 1;
    let mut j = right;

    loop {

        loop    {
            i+=1;
            if array[i as usize] >= pivot { break; }  
        }

        loop    {
            j-=1;
            if array[j as usize] <= pivot { break; }
        }

        if i > j { break; }

        array.swap(i as usize, j as usize);

    }

    return i + 1;
}