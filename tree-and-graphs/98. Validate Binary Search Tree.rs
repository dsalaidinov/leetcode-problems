Link: https://leetcode.com/problems/validate-binary-search-tree/

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![(root.clone(), i64::MIN, i64::MAX)];

        while let Some((node, low, high)) = stack.pop() {
            if let Some(n) = node {
                let n_borrowed = n.borrow();
                let val = n_borrowed.val as i64;

                if val <= low || val >= high {
                    return false;
                }

                if n_borrowed.left.is_some() {
                    stack.push((n_borrowed.left.clone(), low, val));
                }

                if n_borrowed.right.is_some() {
                    stack.push((n_borrowed.right.clone(), val, high));
                }

            }
        }
        true
    }
}