#![allow(dead_code)]
/*
132. 分割回文串 II
困难
696
相关企业
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。

返回符合要求的 最少分割次数 。



示例 1：

输入：s = "aab"
输出：1
解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
示例 2：

输入：s = "a"
输出：0
示例 3：

输入：s = "ab"
输出：1


提示：

1 <= s.length <= 2000
s 仅由小写英文字母组成
 */
struct Solution;
/*
类似131题，我们可以记录子串回文状态
记dp[i][j]表示字符串s的子串s[i..j+1]是否为回文串
那么有
    dp[i][j] = true, if j<=i
    dp[i][j] = dp[i+1][j-1] && s[i]==s[j], 其它情况

这里的状态转移方程中： i依赖i+1, 即依赖后状态，因此i应该逆向遍历


再使用动态规划求出最小分割次数
记 f[i] 为字符串s的子串s[0..i+1]的最小分割次数

为了求出f[i]，我们需要枚举所有最后一次分隔的可能方案
    最后一次分隔点为j，(j<=i) ,其中有 dp[j][i] = true
    枚举所有可能的j值，f[i] = min(f[j-1] + 1)
 */
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.into_bytes();
        let n = s.len();
        // 子串回文状态
        let mut dp = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        // 分隔次数动态规划
        let mut f = vec![i32::MAX; n];
        for i in 0..n {
            if dp[0][i] {
                // 本身就是回文串 无需切割
                f[i] = 0;
            } else {
                // 最后一次分隔不可能从0索引分，因为这意味着本身是回文串
                for j in 1..=i {
                    if dp[j][i] {
                        // j是一个可行的分割点
                        f[i] = std::cmp::min(f[i], f[j - 1] + 1);
                    }
                }
            }
        }
        f[n - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::string::min_cut::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::min_cut("aab".into());
        assert_eq!(ans, 1);
    }

    #[test]
    pub fn t2() {
        let ans = Solution::min_cut("aabaccax".into());
        assert_eq!(ans, 3);
    }
}