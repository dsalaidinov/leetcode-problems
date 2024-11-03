Link: https://leetcode.com/problems/path-sum/

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let remaining_sum = target_sum - node.val;

                if node.left.is_none() && node.right.is_none() {
                    return remaining_sum == 0;
                }

                Solution::has_path_sum(node.left.clone(), remaining_sum)
                    || Solution::has_path_sum(node.right.clone(), remaining_sum)
            }
            None => false,
        }
    }
}