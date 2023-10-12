#![allow(dead_code)]
/*
300. 最长递增子序列
中等
相关标签
相关企业
给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。


示例 1：

输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
示例 2：

输入：nums = [0,1,0,3,2,3]
输出：4
示例 3：

输入：nums = [7,7,7,7,7,7,7]
输出：1


提示：

1 <= nums.length <= 2500
-104 <= nums[i] <= 104


进阶：

你能将算法的时间复杂度降低到 O(n log(n)) 吗?
 */

use std::collections::BTreeMap;

struct Solution;

/*
使用 TreeMap 快速找到小于指定值的数
 */
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // treeMap  value:lis
        let mut map = BTreeMap::new();
        map.insert(nums[0], 1);

        let mut ans = 1;
        for i in 1..n {
            // 找到小于nums[i]的值
            let mut lis = 1;
            for (_, &value) in map.range(..nums[i]) {
                lis = std::cmp::max(lis, 1 + value);
            }
            map.insert(nums[i], lis);
            ans = std::cmp::max(ans, lis);
        }

        ans
    }
}