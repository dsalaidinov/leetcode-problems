Link: https://leetcode.com/problems/merge-two-sorted-lists/

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>, 
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut current = dummy.as_mut();

        let mut pointer1 = list1;
        let mut pointer2 = list2;

        while pointer1.is_some() && pointer2.is_some() {
            if let (Some(node1), Some(node2)) = (pointer1.clone().as_mut(), pointer2.clone().as_mut()) {
                if node1.val < node2.val {
                    current.as_mut().unwrap().next = pointer1;
                    pointer1 = node1.next.take();
                } else {
                    current.as_mut().unwrap().next = pointer2;
                    pointer2 = node2.next.take();
                }
                current = current.unwrap().next.as_mut();
            }
        }

        if pointer1.is_some() {
            current.as_mut().unwrap().next = pointer1;
        }
        if pointer2.is_some() {
            current.as_mut().unwrap().next = pointer2;
        }

        dummy.unwrap().next
    }
}
