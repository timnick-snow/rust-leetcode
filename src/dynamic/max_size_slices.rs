#![allow(dead_code)]
/*
1388. 3n 块披萨
提示
困难
138
相关企业
给你一个披萨，它由 3n 块不同大小的部分组成，现在你和你的朋友们需要按照如下规则来分披萨：

你挑选 任意 一块披萨。
Alice 将会挑选你所选择的披萨逆时针方向的下一块披萨。
Bob 将会挑选你所选择的披萨顺时针方向的下一块披萨。
重复上述过程直到没有披萨剩下。
每一块披萨的大小按顺时针方向由循环数组 slices 表示。

请你返回你可以获得的披萨大小总和的最大值。



示例 1：



输入：slices = [1,2,3,4,5,6]
输出：10
解释：选择大小为 4 的披萨，Alice 和 Bob 分别挑选大小为 3 和 5 的披萨。然后你选择大小为 6 的披萨，Alice 和 Bob 分别挑选大小为 2 和 1 的披萨。你获得的披萨总大小为 4 + 6 = 10 。
示例 2：



输入：slices = [8,9,8,6,1,1]
输出：16
解释：两轮都选大小为 8 的披萨。如果你选择大小为 9 的披萨，你的朋友们就会选择大小为 8 的披萨，这种情况下你的总和不是最大的。


提示：

1 <= slices.length <= 500
slices.length % 3 == 0
1 <= slices[i] <= 1000
 */

struct Solution;

/*
本题可以转化成如下问题：
给一个长度为 3n 的环状序列，你可以在其中选择 n 个数，并且任意两个数不能相邻，求这 n 个数的最大值。

先考虑非环的普通序列

dp[i][j]表示序列前i个数中选择j个不相邻的数的最大值。那么，
如果选择第i个数，那么第i-1个数不能选择，需要从前i-2个数中选择j-1个数
    dp[i][j] = slices[i] + dp[i-2][j-1]
如果不选择第i个数，需要从前i-1个数中选择j个数
    dp[i][j] = dp[i-1][j]

即 dp[i][j] = max(slices[i] + dp[i-2][j-1], dp[i-1][j])

考虑边界值
dp[0][0] = 0
dp[0][j] = slices[0] 仅当 j=1时
dp[i][0] = 0

dp[1][j] = max(slices[0],slices[1]) 仅当 j=1时


环状序列的第一个和最后一个数不能同时选。可以对普通序列进行两遍动态求解，第一遍动态规划中删去普通序列中的第一个数，表示我们不会选第一个数；
第二遍动态规划中我们删去普通序列中的最后一个数，表示我们不会选最后一个数。

链接：https://leetcode.cn/problems/pizza-with-3n-slices/solutions/177086/3n-kuai-pi-sa-by-leetcode-solution/
 */
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        std::cmp::max(calc(&slices[1..]), calc(&slices[0..slices.len() - 1]))
    }
}

fn calc(slices: &[i32]) -> i32 {
    let count = (slices.len() + 1) / 3;
    let mut dp = vec![vec![0; count + 1]; slices.len()];
    dp[0][1] = slices[0];
    dp[1][1] = std::cmp::max(slices[0], slices[1]);
    for i in 2..slices.len() {
        for j in 1..count + 1 {
            dp[i][j] = std::cmp::max(slices[i] + dp[i - 2][j - 1], dp[i - 1][j]);
        }
    }
    dp[slices.len() - 1][count]
}