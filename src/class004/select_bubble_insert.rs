fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    for i in (0..n).rev() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                swap(arr, j, j + 1);
            }
        }
    }
}

fn insert_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    for i in 1..n {
        for j in (1..=i).rev() {
            if arr[j] >= arr[j - 1] {
                break;
            }
            swap(arr, j, j - 1);
        }
    }
}

fn select_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    let mut min_idx = 0;
    for i in 0..n - 1 {
        min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            swap(arr, i, min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [4, 2, 1, 5, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_sort() {
        let mut arr = [4, 2, 1, 5, 3];
        insert_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_select_sort() {
        let mut arr = [4, 2, 1, 5, 3];
        select_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
