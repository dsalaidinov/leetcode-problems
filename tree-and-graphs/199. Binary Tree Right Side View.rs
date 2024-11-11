Link: https://leetcode.com/problems/binary-tree-right-side-view/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result = vec![];

        if let Some(root_node) = root {
            queue.push_back(root_node);
        }

        while !queue.is_empty() {
            let level_size = queue.len();

            for i in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node_borrowed = node.borrow();

                    if i == level_size - 1 {
                        result.push(node_borrowed.val);
                    }

                    if let Some(left) = &node_borrowed.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &node_borrowed.right {
                        queue.push_back(right.clone());
                    }
                }
            }
        }

        result
    }
}