#![allow(dead_code)]
/*
219. 存在重复元素 II
简单
633
相关企业
给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，
满足 nums[i] == nums[j] 且 abs(i - j) <= k 。
如果存在，返回 true ；否则，返回 false 。



示例 1：

输入：nums = [1,2,3,1], k = 3
输出：true
示例 2：

输入：nums = [1,0,1,1], k = 1
输出：true
示例 3：

输入：nums = [1,2,3,1,2,3], k = 2
输出：false




提示：

1 <= nums.length <= 105
-109 <= nums[i] <= 109
0 <= k <= 105
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            if let Some(pre) = map.get_mut(&x) {
                if i - *pre <= k {
                    return true;
                }
                *pre = i;
            } else {
                map.insert(x, i);
            }
        }
        false
    }
}