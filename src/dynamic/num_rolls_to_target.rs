#![allow(dead_code)]
/*
1155. 掷骰子等于目标和的方法数
中等
相关标签
相关企业
提示
这里有 n 个一样的骰子，每个骰子上都有 k 个面，分别标号为 1 到 k 。

给定三个整数 n ,  k 和 target ，返回可能的方式(从总共 kn 种方式中)滚动骰子的数量，使正面朝上的数字之和等于 target 。

答案可能很大，你需要对 109 + 7 取模 。



示例 1：

输入：n = 1, k = 6, target = 3
输出：1
解释：你扔一个有 6 个面的骰子。
得到 3 的和只有一种方法。
示例 2：

输入：n = 2, k = 6, target = 7
输出：6
解释：你扔两个骰子，每个骰子有 6 个面。
得到 7 的和有 6 种方法：1+6 2+5 3+4 4+3 5+2 6+1。
示例 3：

输入：n = 30, k = 30, target = 500
输出：222616187
解释：返回的结果必须是对 109 + 7 取模。


提示：

1 <= n, k <= 30
1 <= target <= 1000
 */
struct Solution;

/*
动态规划
dp[i][j]表示i个骰子和为j的结果数

考虑最后一个骰子的状态，其可能是1~k其中任何一个值
dp[i][j] = sum(dp[i-1][j-x])   1<=x<=k

初始状态
dp[0][0] = 1
 */

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![0; target as usize + 1]; 2];
        dp[0][0] = 1;

        for i in 1..=n as usize {
            for j in 0..=target as usize {
                let mut sum = 0;
                for x in 1..=k as usize {
                    if j >= x {
                        sum = (sum + dp[(i - 1) % 2][j - x]) % MOD;
                    }
                }
                dp[i % 2][j] = sum;
            }
        }
        dp[n as usize % 2][target as usize] as i32
    }
}