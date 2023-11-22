#![allow(dead_code)]
/*
416. 分割等和子集
中等
相关标签
相关企业
给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。



示例 1：

输入：nums = [1,5,11,5]
输出：true
解释：数组可以分割成 [1, 5, 5] 和 [11] 。
示例 2：

输入：nums = [1,2,3,5]
输出：false
解释：数组不能分割成两个元素和相等的子集。


提示：

1 <= nums.length <= 200
1 <= nums[i] <= 100
 */
struct Solution;
/*
总和必为偶数
找到若干个元素 其和为总和的一半即可划分

动态规划
dp[i][j] 表示能够在前i个数能找到一个子集，其和为j
最终所求dp[n][sum/2]

dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i]]

dp[i][0] = true
 */
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let total_sum = nums.iter().sum::<i32>();
        if total_sum & 1 == 1 {
            return false;
        }
        let target = (total_sum >> 1) as usize;
        let mut dp = vec![vec![false; target + 1]; n + 1];
        for i in 0..n + 1 {
            dp[i][0] = true;
        }

        for i in 1..=n {
            for j in 1..=target {
                let value = nums[i - 1] as usize;
                if value > j {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - value]
                }
            }
        }

        dp[n][target]
    }
}