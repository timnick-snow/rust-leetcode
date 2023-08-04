#![allow(dead_code)]
/*
35. 搜索插入位置
简单
2.1K
相关企业
给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 O(log n) 的算法。



示例 1:

输入: nums = [1,3,5,6], target = 5
输出: 2
示例 2:

输入: nums = [1,3,5,6], target = 2
输出: 1
示例 3:

输入: nums = [1,3,5,6], target = 7
输出: 4  // left right-1


提示:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums 为 无重复元素 的 升序 排列数组
-104 <= target <= 104
 */
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let mid = (left + right) >> 1;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod test {
    use crate::arr::search_insert::Solution;

    #[test]
    pub fn t1() {
        let i = Solution::search_insert(vec![1, 3, 5, 6], 0);
        assert_eq!(i, 0);
    }
}