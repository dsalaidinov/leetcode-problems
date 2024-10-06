Link: https://leetcode.com/problems/palindrome-linked-list/

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = head.clone();

        while fast.as_ref().is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next.unwrap().next;
            slow = slow.unwrap().next;
        }

        return slow;
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        prev
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return false;
        }

        let mut mid = Solution::middle_node(head.clone());
        let mut second = Solution::reverse_list(mid);
        let mut first = head;

        while let (Some(mut node1), Some(mut node2)) = (first.as_mut(), second.as_mut()) {
            if node1.val != node2.val {
                return false;
            }

            first = node1.next.take();
            second = node2.next.take();
        }

        true
    }
}