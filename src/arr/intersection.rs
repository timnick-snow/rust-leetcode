#![allow(dead_code)]
/*
349. 两个数组的交集
简单
相关标签
相关企业
给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。



示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
示例 2：

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
解释：[4,9] 也是可通过的


提示：

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 1000
 */
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            let temp = nums1;
            nums1 = nums2;
            nums2 = temp;
        }
        let set = nums1.into_iter().collect::<HashSet<_>>();
        let mut ans = HashSet::new();
        for x in nums2.into_iter() {
            if set.contains(&x) {
                ans.insert(x);
            }
        }
        ans.into_iter().collect()
    }
}