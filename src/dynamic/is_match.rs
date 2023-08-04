#![allow(dead_code)]
/*
44. 通配符匹配
困难
1.1K
相关企业
给你一个输入字符串 (s) 和一个字符模式 (p) ，请你实现一个支持 '?' 和 '*' 匹配规则的通配符匹配：
'?' 可以匹配任何单个字符。
'*' 可以匹配任意字符序列（包括空字符序列）。
判定匹配成功的充要条件是：字符模式必须能够 完全匹配 输入字符串（而不是部分匹配）。


示例 1：

输入：s = "aa", p = "a"
输出：false
解释："a" 无法匹配 "aa" 整个字符串。
示例 2：

输入：s = "aa", p = "*"
输出：true
解释：'*' 可以匹配任意字符串。
示例 3：

输入：s = "cb", p = "?a"
输出：false
解释：'?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。


提示：

0 <= s.length, p.length <= 2000
s 仅由小写英文字母组成
p 仅由小写英文字母、'?' 或 '*' 组成
 */
struct Solution;

/*
考虑字符串s的前i个字符和模式p的前j个字符是否匹配，
状态dp[i][j]表示s的前i个字符和p的前j个字符能够进行匹配

考虑状态转移情况
p[j]是小写字母，则 dp[i][j] = (s[i]==p[j]) && dp[i-1][j-1]
p[j]是问号，则 dp[i][j] = dp[i-1][j-1]
p[j]是星号，则 dp[i][j] = dp[i][j-1] || dp[i-1][j]

考虑状态初始条件
dp[0][0] = true
dp[i][0] = false
dp[0][j] = 当且仅p的前j个字符均为*

 */
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        // 初始化状态
        dp[0][0] = true;
        for j in 0..p.len() {
            if p.chars().nth(j).unwrap() == '*' {
                dp[0][j + 1] = true;
            } else {
                break;
            }
        }
        // 计算状态转移
        for i in 1..=s.len() {
            for j in 1..=p.len() {
                match p.chars().nth(j - 1).unwrap() {
                    '?' => dp[i][j] = dp[i - 1][j - 1],
                    '*' => dp[i][j] = dp[i][j - 1] || dp[i - 1][j],
                    a => dp[i][j] = (s.chars().nth(i - 1).unwrap() == a) && dp[i - 1][j - 1],
                }
            }
        }
        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic::is_match::Solution;

    #[test]
    fn t1() {
        assert!(!Solution::is_match("aa".into(), "a".into()));
        assert!(Solution::is_match("aa".into(), "*".into()));
        assert!(!Solution::is_match("cb".into(), "?a".into()));
        assert!(Solution::is_match("cb".into(), "?b".into()));
    }
}