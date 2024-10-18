Link: https://leetcode.com/problems/reorder-list/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut queue = VecDeque::new();
        let mut current = head.take();

        while let Some(mut node) = current {
            current = node.next.take();
            queue.push_back(node);
        }

        let mut new = ListNode::new(0);
        let (mut pointer, mut front) = (&mut new, true);

        while !queue.is_empty() {
            pointer.next = match front {
                true => queue.pop_front(),
                false => queue.pop_back()
            };

            front = !front;
            pointer = pointer.next.as_mut().unwrap();
        }

        *head = new.next;
    }
}