Link: https://leetcode.com/problems/symmetric-tree/

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(&root, &root)
    }
    
    fn is_mirror(
        left: &Option<Rc<RefCell<TreeNode>>>, 
        right: &Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val &&
                Self::is_mirror(&l.borrow().left, &r.borrow().right) &&
                Self::is_mirror(&l.borrow().right, &r.borrow().left)
            },
            (None, None) => true,
            _ => false,
        }
    }
}


// Iterative solution

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), root.clone()));

        while let Some((left, right)) = queue.pop_front() {
            match (left, right) {
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    queue.push_back((l.borrow().left.clone(), r.borrow().right.clone()));
                    queue.push_back((l.borrow().right.clone(), r.borrow().left.clone()));
                }
                (None, None) => continue,
                _ => return false,
            }
        }

        true
    }
}
