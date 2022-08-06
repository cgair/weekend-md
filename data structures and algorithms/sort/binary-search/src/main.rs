// Basic binary search
// version0: type is usize, and suppose nums is ordered.
pub fn binary_search0(nums: &[usize], target: usize) -> isize {
    if nums.len() == 0 {
        return -1 ;
    }
    let mut left = 0;
    let mut mid = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as isize;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

// generic type
use std::cmp::{PartialEq, PartialOrd};
pub fn binary_search1<T: PartialEq + PartialOrd>(slice: &[T], target: &T) -> isize {
    if slice.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = slice.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if &slice[mid] > target {
            right = mid - 1;
        } else if &slice[mid] < target {
            left = mid + 1;
        } else if &slice[mid] == target {
            return mid as isize;
        }
    }
    -1
}

// Binary search to find left boundary
pub fn binary_search2(nums: &[usize], target: usize) -> isize {
    if nums.len() == 0 {
        return -1;
    }
    let mut left = 0;
    let mut mid = 0;
    let mut right = nums.len() - 1;
    // let mut left_boundary = -1;
    
    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid  - 1;
        } else if nums[mid] == target {
            // left_boundary = mid as isize;
            right = mid - 1;
        }
    }
    // 或者检查出界情况: 当 target 比 nums 中所有元素都大时, 会使得索引越界
    if left >= nums.len() || nums[left] != target {
        return -1;
    }

    left as isize
    // left_boundary
}

// Binary search to find right boundary
pub fn binary_search3(nums: &[usize], target: usize) -> isize {
    if nums.len() == 0 {
        return -1;
    }
    let mut left = 0;
    let mut mid = 0;
    let mut right = nums.len() - 1;
    
    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid  - 1;
        } else if nums[mid] == target {
            left = mid + 1;
        }
    }

    // 当 target 比所有元素都小时, right会被减到 -1 检查出界情况
    if right < 0 || nums[right] != target {
        return -1;
    }
    right as isize
}

fn main() {
    // let nums = &[1, 2, 3, 4, 5, 6];
    // for i in 1..7 {
    //     let ret = binary_search0(nums, i);
    //     println!("Result = {}", ret);
    //     assert_ne!(ret, -1, "should found");   
    // }
    // assert_eq!(binary_search0(nums, usize::MAX), -1);

    // let nums = &[1, 2, 2, 2, 2, 6];
    // let ret = binary_search2(nums, 2);
    // println!("Result = {}", ret);   // 算法会返回 1: 这个 1 的含义是 nums 中小于 2 的元素有 1 个.

    let nums = &[1, 2, 3, 4, 5, 5, 5, 6];
    let ret = binary_search3(nums, 5);
    println!("Result = {}", ret);   

    let index = binary_search1(&vec!["Python", "Php", "Java", "C", "C++", "Rust"], &"Rust");
    println!("Position: {}", index);
    let index = binary_search1(&vec![25, 62, 29, 43, 77], &25);
    println!("Position: {}", index);
    let index = binary_search1(&vec![25, 62, 29, 43, 77], &855);
    println!("Position: {}", index);
}
