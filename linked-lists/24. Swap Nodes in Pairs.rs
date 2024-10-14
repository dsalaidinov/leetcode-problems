Link: https://leetcode.com/problems/swap-nodes-in-pairs/

Time Complexity: O(n)
Space Complexity: O(1)

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut current = dummy.as_mut();

        while let Some(node) = current {
            let mut first = match node.next.take() {
                Some(first) => first,
                None => break,
            };

            let mut second = match first.next.take() {
                Some(second) => second,
                None => {
                    node.next = Some(first);
                    break;
                }
            };

            first.next = second.next.take();
            second.next = Some(first);

            node.next = Some(second);

            current = node.next.as_mut()?.next.as_mut();
        }

        dummy.unwrap().next
    }
}

Time Complexity: O(n)
Space Complexity: O(n)

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut first = head.unwrap();
        let mut second = first.next.take().unwrap();

        first.next = Self::swap_pairs(second.next);

        second.next = Some(first);

        Some(second)
    }
}



