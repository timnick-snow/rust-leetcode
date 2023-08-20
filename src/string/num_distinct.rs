#![allow(dead_code)]
/*
115. 不同的子序列
困难
1.1K
相关企业
给你两个字符串 s 和 t ，统计并返回在 s 的 子序列 中 t 出现的个数。

题目数据保证答案符合 32 位带符号整数范围。



示例 1：

输入：s = "rabbbit", t = "rabbit"
输出：3
解释：
如下所示, 有 3 种可以从 s 中得到 "rabbit" 的方案。
rabbbit
rabbbit
rabbbit
示例 2：

输入：s = "babgbag", t = "bag"
输出：5
解释：
如下所示, 有 5 种可以从 s 中得到 "bag" 的方案。
babgbag
babgbag
babgbag
babgbag
babgbag


提示：

1 <= s.length, t.length <= 1000
s 和 t 由英文字母组成
 */
struct Solution;
/*
dp[i][j] 表示 s中的前i个字符的子序列 中 t的前j个字符 出现的个数。
i < j 时 dp[i][j] = 0

i >= j 时,
    dp[i][j] =  dp[i-1][j] + dp[i-1][j-1]      ;if s[i] == t[j], 可以使用s[i]字符，也可以不使用
    dp[i][j] =  dp[i-1][j]      ;if s[i] != t[j], 无法使用s[i]字符

dp[0][0] = s[0] == t[0]?1:0
j>0, dp[0][j] = 0
i>0, dp[i][0] = dp[i-1][0] + s[i] == t[0]?1:0

 */
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = if s[0] == t[0] { 1 } else { 0 };
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + if s[i] == t[0] { 1 } else { 0 };
        }
        // 状态转移
        for i in 1..m {
            for j in 1..n {
                if i < j {
                    break;
                }
                if s[i] == t[j] {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        dp[m - 1][n - 1]
    }
}