fn exist(arr: &[i32], target: i32) -> bool {
    if arr.is_empty() {
        return false;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        // let mid = (left + right) / 2;
        let mid = left + ((right - left) >> 1);
        if arr[mid] == target {
            return true;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

fn find_left(arr: &[i32], target: i32) -> i32 {
    if arr.is_empty() {
        return -1;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut ans = -1;
    while left <= right {
        let mid = left + ((right - left) >> 1);
        if arr[mid] >= target {
            ans = mid as i32;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    ans
}

fn find_peak(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return -1;
    }
    if arr.len() == 1 || arr[0] > arr[1] {
        return 0;
    }
    if arr[arr.len() - 1] > arr[arr.len() - 2] {
        return (arr.len() - 1) as i32;
    }
    let mut left = 1;
    let mut right = arr.len() - 2;
    let mut ans = -1;
    while left <= right {
        let mid = left + ((right - left) >> 1);
        if arr[mid] > arr[mid + 1] {
            ans = mid as i32;
            right = mid - 1;
        } else {
            left = mid + 1;
            ans = left as i32;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(exist(&arr, 5), true);
        assert_eq!(exist(&arr, 11), false);
    }

    #[test]
    fn test_find_left() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(find_left(&arr, 5), 4);
        assert_eq!(find_left(&arr, 11), -1);
    }

    #[test]
    fn test_find_peak() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("{:?}", find_peak(&arr));
        assert_eq!(find_peak(&arr), 9);
        let arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        println!("{:?}", find_peak(&arr));
        assert_eq!(find_peak(&arr), 0);
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9];
        println!("{:?}", find_peak(&arr));
        assert_eq!(find_peak(&arr), 9);
    }
}
