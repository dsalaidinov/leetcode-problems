Link: https://leetcode.com/problems/remove-duplicates-from-sorted-array/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 1;

        for j in 1..nums.len() {
            if nums[j] != nums[j-1] {
                nums[i] = nums[j];
                i += 1;
            }

        }

        return i as i32;
    }
}