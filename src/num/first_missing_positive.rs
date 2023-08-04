#![allow(dead_code)]


struct Solution;
/*
41. 缺失的第一个正数
提示
困难
1.9K
相关企业
给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。


示例 1：

输入：nums = [1,2,0]
输出：3
示例 2：

输入：nums = [3,4,-1,1]
输出：2
示例 3：

输入：nums = [7,8,9,11,12]
输出：1


提示：

1 <= nums.length <= 5 * 105
-231 <= nums[i] <= 231 - 1
 */


impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        for x in nums.iter_mut() {
            if *x <= 0 {
                *x = len + 1;
            }
        }
        for i in 0..nums.len() {
            let x = nums[i].abs();
            if x <= len {
                let value = nums[(x - 1) as usize];
                nums[(x - 1) as usize] = -value.abs();
            }
        }
        for (i, &value) in nums.iter().enumerate() {
            if value > 0 {
                return (i + 1) as i32;
            }
        }
        len + 1
    }
}