Link: https://leetcode.com/problems/rotate-array/

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;

        if k == 0 || n == 0 {
            return;
        }

        fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
            let mut left = start;
            let mut right = end;

            while left < right {
                nums.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }
}