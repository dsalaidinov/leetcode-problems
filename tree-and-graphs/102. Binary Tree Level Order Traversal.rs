Link: https://leetcode.com/problems/binary-tree-level-order-traversal/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut result = vec![];

        if let Some(root_node) = root {
            queue.push_back(root_node);
        }

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = vec![];

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node_borrowed = node.borrow();
                    current_level.push(node_borrowed.val);

                    if let Some(left) = &node_borrowed.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &node_borrowed.right {
                        queue.push_back(right.clone());
                    }
                }
            }
            result.push(current_level);
        }

        result
    }
}
