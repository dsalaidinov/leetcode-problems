Link: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut first = head.clone();
        let mut second = &mut head;

        for _ in 0..n {
            first = first.map(|n| n.next)?;
        }
        
        while let Some(n) = first {
            first = n.next;
            second = &mut second.as_mut()?.next;
        }

        *second = second.as_mut().and_then(|n| n.next.take());

        head
    }
}