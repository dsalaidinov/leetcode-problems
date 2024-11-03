Link: https://leetcode.com/problems/invert-binary-tree/description

// Recourseive solution

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();

            let left = Solution::invert_tree(node.left.take());
            let right = Solution::invert_tree(node.right.take());

            node.left = right;
            node.right = left;
        }
        root
    }
}

// Iterative solution

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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        if let Some(node) = root.clone() {
            queue.push_back(node);
        }

        while let Some(node) = queue.pop_front() {
            let mut node = node.borrow_mut();

            let left = node.left.take();
            node.left = node.right.take();
            node.right = left;

            if let Some(left_node) = node.left.clone() {
                queue.push_back(left_node);
            }

            if let Some(right_node) = node.right.clone() {
                queue.push_back(right_node);
            }
        }
        root
    }
}




