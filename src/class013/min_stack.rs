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
