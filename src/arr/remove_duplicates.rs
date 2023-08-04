#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] != nums[i - 1] {
                nums[cur] = nums[i];
                cur += 1;
            }
        }
        cur as i32
    }
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            if nums[left] != val {
                left += 1;
                continue;
            }
            nums[left] = nums[right];
            if right == 0 {
                break;
            }
            right -= 1;
        }
        left as i32
    }
}