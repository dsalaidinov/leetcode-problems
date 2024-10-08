use core::cmp::min;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut begin = 0;
        let mut window_state = 0;
        let mut result = i32::MAX;

        for end in 0..nums.len() {
            window_state += nums[end];

            while window_state >= target {
                let mut window_size = end - begin + 1;
                result = min(result, window_size as i32);
                window_state -= nums[begin];
                begin += 1;
            }

        }

        if result == i32::MAX {
            return 0;
        } 
        result 
    }
}