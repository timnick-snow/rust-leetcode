#![allow(dead_code)]
/*
322. 零钱兑换
中等
相关标签
相关企业
给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。

计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。

你可以认为每种硬币的数量是无限的。



示例 1：

输入：coins = [1, 2, 5], amount = 11
输出：3
解释：11 = 5 + 5 + 1
示例 2：

输入：coins = [2], amount = 3
输出：-1
示例 3：

输入：coins = [1], amount = 0
输出：0


提示：

1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104
 */
struct Solution;


/*
动态规划
dp[i]表示总金额为i时的最小硬币数量

dp[i] = 1 + min( dp[i-coins[j]] )
 */
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for i in 1..amount as usize + 1 {
            let mut flag = false;
            for &x in coins.iter() {
                if i >= x as usize && dp[i - x as usize] != -1 {
                    dp[i] = dp[i].min(1 + dp[i - x as usize]);
                    flag = true;
                }
            }
            if !flag {
                dp[i] = -1;
            }
        }

        dp[amount as usize]
    }
}