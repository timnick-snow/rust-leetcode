#![allow(dead_code)]
/*
343. 整数拆分
中等
相关标签
相关企业
提示
给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。

返回 你可以获得的最大乘积 。



示例 1:

输入: n = 2
输出: 1
解释: 2 = 1 + 1, 1 × 1 = 1。
示例 2:

输入: n = 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。


提示:

2 <= n <= 58
 */
struct Solution;

/*
动态规划

数学归纳
 */
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 4 {
            return n - 1;
        }

        let x = n / 3;
        let remain = n % 3;
        match remain {
            0 => 3_i32.pow(x as u32),
            1 => 3_i32.pow(x as u32 - 1) * 4,
            2 => 3_i32.pow(x as u32) * 2,
            _ => unreachable!()
        }
    }
    pub fn integer_break_dyn(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 2;
        for i in 3..=n {
            for j in 1..i {
                dp[i] = dp[i].max(std::cmp::max(j * (i - j), j * dp[i - j]));
            }
        }
        dp[n] as i32
    }
}