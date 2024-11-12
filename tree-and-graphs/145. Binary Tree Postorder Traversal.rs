Link: https://leetcode.com/problems/binary-tree-postorder-traversal/

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::new();

        if let Some(root_node) = root {
            stack.push(root_node);
        }

        while let Some(node) = stack.pop() {
            let node_borrowed  = node.borrow();
            result.push(node_borrowed.val);

            if let Some(left) = &node_borrowed.left {
                stack.push(left.clone());
            }

            if let Some(right) = &node_borrowed.right {
                stack.push(right.clone());
            }
        }

        result.reverse();
        result
    }
}