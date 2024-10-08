Link: https://leetcode.com/problems/maximum-average-subarray-i

use core::cmp::max;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut begin = 0;
        let mut window_state = 0;
        let mut result = i32::MIN;

        for end in 0..nums.len() {
            window_state += nums[end];

            if end - begin + 1 == k as usize {
                result = max(result, window_state);
                window_state -= nums[begin];
                begin += 1;
            }
        }

        result as f64 / k as f64
    }
}