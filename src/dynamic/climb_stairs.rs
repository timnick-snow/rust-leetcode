#![allow(dead_code)]
/*
70. 爬楼梯
提示
简单
3.2K
相关企业
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？



示例 1：

输入：n = 2
输出：2
解释：有两种方法可以爬到楼顶。
1. 1 阶 + 1 阶
2. 2 阶
示例 2：

输入：n = 3
输出：3
解释：有三种方法可以爬到楼顶。
1. 1 阶 + 1 阶 + 1 阶
2. 1 阶 + 2 阶
3. 2 阶 + 1 阶


提示：

1 <= n <= 45
 */
struct Solution;
/*
动态规划
dp[i] 表示 到达i阶阶梯 可以有的方法总数
那么
dp[i] = dp[i-1] + dp[i-2]

初始值
dp[1] = 1;
dp[2] = 2;
 */
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut dp1 = 2;
        let mut dp2 = 1;
        for _ in 3..=n {
            let temp = dp1 + dp2;
            dp2 = dp1;
            dp1 = temp;
        }
        dp1
    }
}