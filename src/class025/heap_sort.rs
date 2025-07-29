fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn heapify(arr: &mut [i32], size: usize, i: usize) {
    let mut l = 2 * i + 1;
    let mut best = l;
    let mut i = i;
    while l < size {
        if l + 1 < size && arr[l + 1] > arr[l] {
            best = l + 1;
        } else {
            best = l;
        }
        if arr[best] <= arr[i] {
            break;
        }
        swap(arr, i, best);
        i = best;
        l = 2 * i + 1;
    }
}

fn heap_insert(arr: &mut [i32], i: usize) {
    let mut i = i;
    while arr[i] > arr[((i as i32 - 1) / 2) as usize] {
        swap(arr, i, ((i as i32 - 1) / 2) as usize);
        i = ((i as i32 - 1) / 2) as usize;
    }
}

fn heap_sort(arr: &mut [i32]) {
    let size = arr.len();
    for i in (0..size).rev() {
        heapify(arr, size, i);
    }
    for i in (1..size).rev() {
        swap(arr, 0, i);
        heapify(arr, i, 0);
    }
}

// 测试链接 : https://leetcode.cn/problems/sort-an-array/
fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    heap_sort(&mut nums);
    nums
}

use std::collections::BinaryHeap;
fn heap_sort1(arr: &mut [i32]) {
    let mut heap = BinaryHeap::new();
    let size = arr.len();
    for i in arr.as_ref() {
        heap.push(*i);
    }
    for i in (0..size).rev() {
        arr[i] = heap.pop().unwrap();
    }
}
use std::cmp::Reverse;
fn heap_sort2(arr: &mut [i32]) {
    let mut heap = BinaryHeap::new();
    let size = arr.len();
    for i in arr.as_ref() {
        heap.push(Reverse(*i));
    }
    for i in (0..size).rev() {
        arr[i] = heap.pop().unwrap().0;
    }
}

// https://leetcode.cn/problems/divide-intervals-into-minimum-number-of-groups/
fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let n = intervals.len();
    let mut intervals = intervals;
    intervals.sort_by_key(|x| x[0]);
    // println!("intervals: {:?}", intervals);
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..n {
        while !heap.is_empty() && heap.peek().unwrap().0 < intervals[i][0] {
            heap.pop();
        }
        heap.push(Reverse(intervals[i][1]));
        ans = ans.max(heap.len() as i32);
        // println!("heap: {:?}, ans: {}", heap, ans);
    }
    ans
}

//https://leetcode.cn/problems/minimum-operations-to-halve-array-sum/
fn halve_array(nums: Vec<i32>) -> i32 {
    // let values = nums.iter().map(|x| (*x as i64) << 20).collect::<Vec<i64>>();
    let mut sum = 0;
    let mut heap = BinaryHeap::from(
        nums.iter()
            .map(|x| {
                let y = (*x as i64) << 20;
                sum += y;
                y
            })
            .collect::<Vec<i64>>(),
    );
    let mut ans = 0;
    sum = sum / 2;
    while sum > 0 {
        let x = heap.pop().unwrap() >> 1;
        ans += 1;
        sum -= x;
        heap.push(x);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        // heap_sort(&mut arr);
        heap_sort1(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
    }

    #[test]
    fn test_sort_reverse() {
        let mut arr = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        heap_sort2(&mut arr);
        assert_eq!(arr, [16, 14, 10, 9, 8, 7, 4, 3, 2, 1]);
    }

    #[test]
    fn test() {
        let mut a: usize = 10;
        for i in 0..10 {
            println!("{:?}", a);
            a = ((a as i32 - 1) / 2) as usize;
        }
    }

    #[test]
    fn test_min_groups() {
        let intervals = vec![
            vec![5, 10],
            vec![6, 8],
            vec![1, 5],
            vec![2, 3],
            vec![1, 10],
            vec![9, 12],
        ];
        assert_eq!(min_groups(intervals), 3);
    }
}
