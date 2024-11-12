Link: https://leetcode.com/problems/binary-tree-preorder-traversal/
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
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
       let mut stack = Vec::new();
        let mut result = Vec::new();

        if let Some(root_node) = root {
            stack.push(root_node);
        }

        while let Some(node) = stack.pop() {
            let node_borrowed  = node.borrow();
            result.push(node_borrowed.val);

            if let Some(right) = &node_borrowed.right {
                stack.push(right.clone());
            }

            if let Some(left) = &node_borrowed.left {
                stack.push(left.clone());
            }

      
        }

        result
    }
}