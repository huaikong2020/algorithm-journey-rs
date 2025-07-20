struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
// #[derive(Debug, Clone)]
// struct DoubleLinkedList {
//     val: i32,
//     last: Option<Box<DoubleLinkedList>>,
//     next: Option<Box<DoubleLinkedList>>,
// }

// impl DoubleLinkedList {
//     fn new(val: i32) -> Self {
//         DoubleLinkedList {
//             val,
//             last: None,
//             next: None,
//         }
//     }
// }

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut next = None;
    let mut head = head;
    while let Some(mut node) = head {
        next = node.next;
        node.next = pre;
        pre = Some(node);
        head = next;
    }
    pre
}

// fn reverse_double_list(head: Option<Box<DoubleLinkedList>>) -> Option<Box<DoubleLinkedList>> {
//     let mut pre = None;
//     let mut next = None;
//     let mut head = head;
//     while let Some(mut node) = head {
//         next = node.next;
//         let a = &next;
//         node.next = pre;
//         node.last = next;
//         pre = Some(node);
//         head = None;
//         // head = next;
//     }
//     pre
// }
