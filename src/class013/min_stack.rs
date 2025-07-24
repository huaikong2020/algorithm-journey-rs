use std::collections::VecDeque;
// 测试链接 : https://leetcode.cn/problems/min-stack/
struct MinStack {
    stack: VecDeque<i32>,
    min_stack: VecDeque<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            min_stack: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push_front(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.front().unwrap() {
            self.min_stack.push_front(val);
        } else {
            self.min_stack.push_front(*self.min_stack.front().unwrap());
        }
    }

    fn pop(&mut self) {
        self.stack.pop_front();
        self.min_stack.pop_front();
    }

    fn top(&self) -> i32 {
        if self.stack.is_empty() {
            return 0;
        }
        *self.stack.front().unwrap()
    }

    fn get_min(&self) -> i32 {
        if self.min_stack.is_empty() {
            return 0;
        }
        *self.min_stack.front().unwrap()
    }
}

use std::borrow::Cow;

use url::Url;
fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    println!("key: {}, v: {}", k, v);
    // 当修改发生时，k 变成 Owned
    k.to_mut().push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
    // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
    // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}
