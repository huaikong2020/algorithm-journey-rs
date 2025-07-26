use std::vec;

fn merge(l: usize, m: usize, r: usize, arr: &mut [i32]) {
    let mut helper = vec![0; r - l + 1];
    let mut a = l;
    let mut b = m + 1;
    for i in 0..(r - l + 1) {
        if a > m {
            helper[i] = arr[b];
            b += 1;
        } else if b > r {
            helper[i] = arr[a];
            a += 1;
        } else if arr[a] > arr[b] {
            helper[i] = arr[b];
            b += 1;
        } else {
            helper[i] = arr[a];
            a += 1;
        }
    }
    for i in 0..(r - l + 1) {
        arr[l + i] = helper[i];
    }
}

fn merge_sort(l: usize, r: usize, arr: &mut [i32]) {
    if l < r {
        let m = (l + r) / 2;
        merge_sort(l, m, arr);
        merge_sort(m + 1, r, arr);
        merge(l, m, r, arr);
    }
}

// 测试链接 : https://www.nowcoder.com/practice/edfe05a1d45c4ea89101d936cac32469
fn merge1(l: usize, m: usize, r: usize, arr: &mut [i32]) -> i32 {
    let mut ans = 0;
    let mut sum = 0;
    let mut j = l;
    for i in m + 1..r + 1 {
        while j <= m && arr[j] <= arr[i] {
            sum += arr[j];
            j += 1;
        }
        ans += sum;
    }
    merge(l, m, r, arr);
    ans
}

fn small_sum(l: usize, r: usize, arr: &mut [i32]) -> i32 {
    if l == r {
        0
    } else {
        let m = (l + r) / 2;
        let mut ans = small_sum(l, m, arr);
        ans += small_sum(m + 1, r, arr);
        ans += merge1(l, m, r, arr);
        ans
    }
}

// 测试链接 : https://leetcode.cn/problems/reverse-pairs/
fn merge2(l: usize, m: usize, r: usize, arr: &mut [i32]) -> i32 {
    let mut ans = 0;
    let mut j = m + 1;
    for i in l..m + 1 {
        while j <= r && arr[i] as i64 > 2 * arr[j] as i64 {
            j += 1;
        }
        ans += (j - m - 1) as i32;
    }
    merge(l, m, r, arr);
    ans
}

fn reverse_pairs1(l: usize, r: usize, arr: &mut [i32]) -> i32 {
    if l == r {
        0
    } else {
        let m = (l + r) / 2;
        let mut ans = reverse_pairs1(l, m, arr);
        ans += reverse_pairs1(m + 1, r, arr);
        ans += merge2(l, m, r, arr);
        ans
    }
}

fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut arr = nums.clone();
    reverse_pairs1(0, arr.len() - 1, &mut arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [4, 2, 1, 5, 3];
        merge_sort(0, arr.len() - 1, &mut arr);
        println!("{:?}", arr);
        // assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_small_sum() {
        let mut arr = [4, 2, 1, 5, 3];
        let ans = small_sum(0, arr.len() - 1, &mut arr);
        println!("{}", ans);
    }
    #[test]
    fn test_reverse_pairs() {
        let nums = vec![
            2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647,
        ];
        let ans = reverse_pairs(nums);
        println!("{}", ans);
    }
}
