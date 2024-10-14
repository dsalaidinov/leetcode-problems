Link: https://leetcode.com/problems/reverse-nodes-in-k-group/

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn has_k_nodes(head: &Option<Box<ListNode>>, k: i32) -> bool {
            let mut count = 0;
            let mut current = head;
            while let Some(node) = current {
                count += 1;
                if count == k {
                    return true;
                }
                current = &node.next;
            }
            false
        }

        fn reverse(head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut prev = None;
            let mut current = head;
            let mut count = 0;

            while count < k {
                let next = current.as_ref().and_then(|node| node.next.clone());
                current.as_mut().map(|node| {
                    node.next = prev;
                });
                prev = current;
                current = next;
                count += 1;
            }

            (prev, current)
        }

        if head.is_none() || k < 2 || !has_k_nodes(&head, k) {
            return head;
        }

        let (reversed_head, next_head) = reverse(head, k);
        let mut result = reversed_head;

        let mut tail = result.as_mut();
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = Self::reverse_k_group(next_head, k);
                break;
            }
            tail = node.next.as_mut();
        }

        result
    }
}
