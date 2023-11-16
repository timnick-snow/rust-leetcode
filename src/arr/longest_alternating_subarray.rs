#![allow(dead_code)]
/*
2760. 最长奇偶子数组
简单
相关标签
相关企业
提示
给你一个下标从 0 开始的整数数组 nums 和一个整数 threshold 。

请你从 nums 的子数组中找出以下标 l 开头、下标 r 结尾 (0 <= l <= r < nums.length) 且满足以下条件的 最长子数组 ：

nums[l] % 2 == 0
对于范围 [l, r - 1] 内的所有下标 i ，nums[i] % 2 != nums[i + 1] % 2
对于范围 [l, r] 内的所有下标 i ，nums[i] <= threshold
以整数形式返回满足题目要求的最长子数组的长度。

注意：子数组 是数组中的一个连续非空元素序列。



示例 1：

输入：nums = [3,2,5,4], threshold = 5
输出：3
解释：在这个示例中，我们选择从 l = 1 开始、到 r = 3 结束的子数组 => [2,5,4] ，满足上述条件。
因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。
示例 2：

输入：nums = [1,2], threshold = 2
输出：1
解释：
在这个示例中，我们选择从 l = 1 开始、到 r = 1 结束的子数组 => [2] 。
该子数组满足上述全部条件。可以证明 1 是满足题目要求的最大长度。
示例 3：

输入：nums = [2,3,4,5], threshold = 4
输出：3
解释：
在这个示例中，我们选择从 l = 0 开始、到 r = 2 结束的子数组 => [2,3,4] 。
该子数组满足上述全部条件。
因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。


提示：

1 <= nums.length <= 100
1 <= nums[i] <= 100
1 <= threshold <= 100
 */
struct Solution;

/*
动态规划
滑动窗口
 */
impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let mut dp = if nums[0] & 1 == 0 && nums[0] <= threshold { 1 } else { 0 };
        let mut ans = dp;

        for i in 1..n {
            if nums[i] > threshold {
                dp = 0;
                continue;
            }
            if nums[i - 1] <= threshold && (nums[i] ^ nums[i - 1]) & 1 == 1 {
                // 与前一个数奇偶性不同
                dp += 1;
                ans = ans.max(dp);
            } else {
                // 前一个数超标或者与当前数奇偶性相同
                dp = if nums[i] & 1 == 0 { 1 } else { 0 };
                ans = ans.max(dp);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::longest_alternating_subarray::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![10, 1, 10];
        println!("{}", Solution::longest_alternating_subarray(nums,3));
    }
}