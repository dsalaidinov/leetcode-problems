// Intuition
// To solve the Merge Sorted Array problem, I first thought about how to combine two sorted arrays efficiently.
// The brute force approach of merging the two arrays into a new one and then sorting it would be inefficient.
// Instead, I realized that since both arrays are already sorted, I could utilize a two-pointer technique to merge them in-place in `nums1`.
// Link: https://leetcode.com/problems/merge-sorted-array/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // Initialize pointers
        let mut i = m - 1;  // Pointer for the last element in nums1
        let mut j = n - 1;  // Pointer for the last element in nums2
        let mut k = (m + n - 1) as usize;  // Pointer for the last position in nums1

        // Merge nums2 into nums1
        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        // If there are remaining elements in nums2, copy them to nums1
        while j >= 0 {
            nums1[k] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

// Complexity
// Time complexity: O(m + n)
// Space complexity: O(1)
