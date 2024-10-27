Link: https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
use core::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth = Solution::max_depth(node.borrow().left.clone());
            let right_depth = Solution::max_depth(node.borrow().right.clone());

            return max(left_depth, right_depth) + 1
        } else {
            0
        }
    }
}

// ---------------------------------------------------------

//Solution 2: using stack
use core::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut stack = vec![(root.clone(), 1)];
        let mut max_depth = 0;

        while let Some((node, depth)) = stack.pop() {
            if let Some(n) = node {
                max_depth = max_depth.max(depth);

                let n_borrowed = n.borrow();
                if n_borrowed.left.is_some() {
                    stack.push((n_borrowed.left.clone(), depth + 1));
                }
                if n_borrowed.right.is_some() {
                    stack.push((n_borrowed.right.clone(), depth + 1));
                }
            }
        }

        max_depth
    }
}