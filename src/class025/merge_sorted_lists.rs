// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
//https://leetcode.cn/problems/merge-k-sorted-lists/
fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut node = None;
    let mut cur = &mut node;
    let mut heap = BinaryHeap::new();
    for list in lists {
        if let Some(x) = list {
            heap.push(Reverse(x));
        }
    }
    while let Some(mut x) = heap.pop() {
        if let Some(y) = x.0.next.take() {
            heap.push(Reverse(y));
        }
        cur = &mut cur.insert(x.0).next;
    }
    node

    // let mut heap = BinaryHeap::new();
    // for node in lists {
    //     if node.is_some() {
    //         heap.push(node);
    //     }
    // }
    // if heap.is_empty() {
    //     return None;
    // }
    // let ans = heap.pop().unwrap();
    // let mut pre = ans.clone();
    // if let Some(next) = pre.clone().unwrap().next {
    //     heap.push(Some(next));
    // }
    // while let Some(cur) = heap.pop() {
    //     pre.as_mut().unwrap().next = cur.clone();
    //     pre = cur.clone();
    //     if let Some(next) = cur.clone().unwrap().next {
    //         heap.push(Some(next));
    //     }
    // }
    // ans
}
