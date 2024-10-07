Link: https://leetcode.com/problems/remove-duplicates-from-sorted-list/

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
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            let mut next_node = node.next.take();

            while let Some(next) = next_node.as_mut() {
                if next.val == node.val {
                    { next_node = next.next.take(); }
                } else {
                    node.next = next_node; break;
                }
            }
            current = node.next.as_mut();
        }

        head
    }
}