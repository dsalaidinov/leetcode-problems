// Intuition
// To solve the Two Sum problem, I initially thought about using a brute force approach, 
// which involves checking every possible pair of numbers in the array. 
// However, this would result in a time complexity of O(n²). 
// I realized that by utilizing a hash map, I could efficiently check for the required 
// complement of each number in a single pass through the array, reducing the time complexity to O(n).

// Approach
// 1. Create a hash map to store the numbers we have seen so far and their respective indices.
// 2. Iterate through the nums array:
//    - For each number, calculate its complement (the number that, when added to the current number, equals the target).
//    - Check if the complement exists in the hash map:
//      - If it does, return the indices of the current number and its complement.
//      - If it does not, insert the current number and its index into the hash map.
// 3. If no solution is found after iterating through the array, return an empty vector.

// Complexity
// Time complexity: O(n)
// Space complexity: O(n)
// Link: https://leetcode.com/problems/two-sum/


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![]
    }
}
