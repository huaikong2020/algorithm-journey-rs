use rand::Rng;

fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn partition(arr: &mut [i32], low: usize, high: usize, x: i32) -> (i32, i32) {
    let mut i = low;
    let mut l = low;
    let mut r = high;
    while i <= r {
        if arr[i] == x {
            i += 1;
        } else if arr[i] < x {
            swap(arr, i, l);
            l += 1;
            i += 1;
        } else {
            swap(arr, i, r);
            r -= 1;
        }
    }
    (l as i32, r as i32)
}

fn quick_sort(arr: &mut [i32], low: i32, high: i32) {
    if low < high {
        let index = rand::thread_rng().gen_range(low..=high);
        let x = arr[index as usize];
        let (l, r) = partition(arr, low as usize, high as usize, x);
        quick_sort(arr, low, l - 1);
        quick_sort(arr, r + 1, high);
    }
}

// 测试链接 : https://leetcode.cn/problems/sort-an-array/
fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums.clone();
    let high = nums.len() - 1;
    quick_sort(&mut nums, 0, high as i32);
    nums
}

// 测试链接 : https://leetcode.cn/problems/kth-largest-element-in-an-array/
fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mut ans = 0;
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;
    let k = nums.len() as i32 - k;
    while l <= r {
        let index = rand::thread_rng().gen_range(l..=r);
        let x = nums[index as usize];
        let (low, high) = partition(&mut nums, l as usize, r as usize, x);
        if k < low {
            r = low - 1;
        } else if k > high {
            l = high + 1;
        } else {
            ans = x;
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [3, 2, 1, 4, 5];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_array() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let sorted_nums = sort_array(nums);
        assert_eq!(sorted_nums, [0, 0, 1, 1, 2, 5]);
    }

    #[test]
    fn test() {
        let a: usize = 0;
        println!("a: {}", (a as i32) - 1);
    }
}
