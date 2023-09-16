#![allow(dead_code)]
/*
229. 多数元素 II
中等
相关标签
相关企业
提示
给定一个大小为 n 的整数数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。



示例 1：

输入：nums = [3,2,3]
输出：[3]
示例 2：

输入：nums = [1]
输出：[1]
示例 3：

输入：nums = [1,2]
输出：[1,2]


提示：

1 <= nums.length <= 5 * 104
-109 <= nums[i] <= 109


进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1)的算法解决此问题。
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let min_count = (nums.len() / 3) as i32;
        let mut map = HashMap::new();
        for x in nums.into_iter() {
            let count: &mut i32 = map.entry(x).or_default();
            *count += 1;
        }
        let mut ans = Vec::new();
        for (key, val) in map.into_iter() {
            if val > min_count {
                ans.push(key);
            }
        }
        ans
    }
}