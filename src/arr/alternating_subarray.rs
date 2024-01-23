#![allow(dead_code)]
/*
2765. 最长交替子数组
简单
相关标签
相关企业
提示
给你一个下标从 0 开始的整数数组 nums 。如果 nums 中长度为 m 的子数组 s 满足以下条件，我们称它是一个 交替子数组 ：

m 大于 1 。
s1 = s0 + 1 。
下标从 0 开始的子数组 s 与数组 [s0, s1, s0, s1,...,s(m-1) % 2] 一样。也就是说，s1 - s0 = 1 ，s2 - s1 = -1 ，s3 - s2 = 1 ，s4 - s3 = -1 ，以此类推，直到 s[m - 1] - s[m - 2] = (-1)m 。
请你返回 nums 中所有 交替 子数组中，最长的长度，如果不存在交替子数组，请你返回 -1 。

子数组是一个数组中一段连续 非空 的元素序列。



示例 1：

输入：nums = [2,3,4,3,4]
输出：4
解释：交替子数组有 [3,4] ，[3,4,3] 和 [3,4,3,4] 。最长的子数组为 [3,4,3,4] ，长度为4 。
示例 2：

输入：nums = [4,5,6]
输出：2
解释：[4,5] 和 [5,6] 是仅有的两个交替子数组。它们长度都为 2 。


提示：

2 <= nums.length <= 100
1 <= nums[i] <= 104
 */
struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 1;
        let mut left = 0;
        while left < n {
            let mut right = left + 1;
            let mut d = 1;
            while right < n && nums[right] - nums[right - 1] == d {
                right += 1;
                d *= -1;
            }
            ans = ans.max((right - left) as i32);
            if d == 1 {
                left = right;
            } else {
                left = right - 1;
            }
        }
        if ans > 1 {
            ans
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::alternating_subarray::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![2, 3, 4, 3, 4];
        let ans = Solution::alternating_subarray(nums);
        println!("{}", ans);
    }
}