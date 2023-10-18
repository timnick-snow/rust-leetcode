#![allow(dead_code)]

/*
312. 戳气球
困难
相关标签
相关企业
有 n 个气球，编号为0 到 n - 1，每个气球上都标有一个数字，这些数字存在数组 nums 中。

现在要求你戳破所有的气球。戳破第 i 个气球，你可以获得 nums[i - 1] * nums[i] * nums[i + 1] 枚硬币。 这里的 i - 1 和 i + 1 代表和 i 相邻的两个气球的序号。如果 i - 1或 i + 1 超出了数组的边界，那么就当它是一个数字为 1 的气球。

求所能获得硬币的最大数量。



示例 1：
输入：nums = [3,1,5,8]
输出：167
解释：
nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
示例 2：

输入：nums = [1,5]
输出：10


提示：

n == nums.length
1 <= n <= 300
0 <= nums[i] <= 100
 */
struct Solution;

/*
动态规划

考虑戳破开区间(i,j)内的所有气球可以获得的最大分数，即我们不戳破边界i与j气球，只考虑中间的气球。
假设最后一个戳破的气球为k，那么开区间(i,j)内获得的分数为 nums[i]*nums[k]*nums[j] + scores(i,k) + scores(k,j)
其中scores(i,k)表示你在开区间(i,k)内获得的分数，这里采用分治的思想，将大区间分成两个小区间，你获得的分数是两个小区间的分数和
加上你最后戳破的气球的分数。


设dp[i][j]表示开区间(i,j)内你能获得的最大分数

那么 dp[i][j] = MAX( nums[i]*nums[k]*nums[j] + dp[i][k] + dp[k][j] ) 其中 i<k<j

为了解决问题，我们在原数组左右两端分别加上一个元素1，用于表示问题的边界。
最后的结果就是 dp[0][n+1]
 */
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        // 增加边界
        let nums = std::iter::once(1)
            .chain(nums.into_iter())
            .chain(std::iter::once(1))
            .collect::<Vec<_>>();
        // 动态规划
        for i in (0..=n - 1).rev() {
            for j in i + 2..n + 2 {
                for k in i + 1..j {
                    dp[i][j] = std::cmp::max(dp[i][j], nums[i]*nums[k]*nums[j] + dp[i][k] + dp[k][j]);
                }
            }
        }
        // 返回开区间的值
        dp[0][n + 1]
    }
}