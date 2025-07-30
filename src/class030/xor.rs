// 测试链接 : https://leetcode.cn/problems/single-number-iii/
fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut xor1 = 0;
    for num in &nums {
        xor1 ^= num;
    }
    let right_most_bit = xor1 & (-xor1);
    let mut xor2 = 0;
    for num in nums {
        if num & right_most_bit == 0 {
            xor2 ^= num;
        }
    }
    vec![xor1 ^ xor2, xor2]
}

//https://leetcode.cn/problems/single-number-ii/
fn single_number1(nums: Vec<i32>) -> i32 {
    find(nums, 3)
}

fn find(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnts = [0; 32];
    for num in nums {
        for i in 0..32 {
            cnts[i] += (num >> i) & 1;
        }
    }
    let mut res = 0;
    for i in 0..32 {
        if cnts[i] % k != 0 {
            res |= 1 << i;
        }
    }
    res
}
