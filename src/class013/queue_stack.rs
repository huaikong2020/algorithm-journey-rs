use std::collections::VecDeque;

struct MyCircularQueue {
    queue: VecDeque<i32>,
    size: i32,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: VecDeque::new(),
            size: k,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue.push_front(value);
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.queue.pop_back();
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        *self.queue.back().unwrap()
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        *self.queue.front().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn is_full(&self) -> bool {
        self.queue.len() as i32 == self.size
    }
}

struct MyQueue {
    stack: VecDeque<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            -1
        } else {
            self.stack.pop_front().unwrap()
        }
    }

    fn peek(&self) -> i32 {
        if self.stack.is_empty() {
            -1
        } else {
            *self.stack.front().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_front(x)
    }

    fn pop(&mut self) -> i32 {
        if self.queue.is_empty() {
            -1
        } else {
            self.queue.pop_front().unwrap()
        }
    }

    fn top(&self) -> i32 {
        if self.queue.is_empty() {
            -1
        } else {
            *self.queue.front().unwrap()
        }
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

struct MyCircularDeque {
    deque: VecDeque<i32>,
    size: i32,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            deque: VecDeque::new(),
            size: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.deque.push_front(value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.deque.push_back(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.deque.pop_front();
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.deque.pop_back();
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        *self.deque.front().unwrap()
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        *self.deque.back().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    fn is_full(&self) -> bool {
        self.deque.len() as i32 == self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecdeque() {
        let mut q = VecDeque::new();
        q.push_back(1);
        q.push_back(2);
        q.push_back(3);

        assert_eq!(q.len(), 3);
        assert_eq!(q.front(), Some(&1));
        assert_eq!(q.back(), Some(&3));
        assert_eq!(q.pop_front(), Some(1));
        assert_eq!(q.pop_back(), Some(3));
        assert_eq!(q.pop_front(), Some(2));
        assert_eq!(q.pop_front(), None);
        assert_eq!(q.is_empty(), true);
        // println!("{}", *q.front().unwrap());
    }
}
