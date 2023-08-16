#![allow(dead_code)]
/*
97. 交错字符串
中等
899
相关企业
给定三个字符串 s1、s2、s3，请你帮忙验证 s3 是否是由 s1 和 s2 交错 组成的。

两个字符串 s 和 t 交错 的定义与过程如下，其中每个字符串都会被分割成若干 非空 子字符串：

s = s1 + s2 + ... + sn
t = t1 + t2 + ... + tm
|n - m| <= 1
交错 是 s1 + t1 + s2 + t2 + s3 + t3 + ... 或者 t1 + s1 + t2 + s2 + t3 + s3 + ...
注意：a + b 意味着字符串 a 和 b 连接。



示例 1：


输入：s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
输出：true
示例 2：

输入：s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
输出：false
示例 3：

输入：s1 = "", s2 = "", s3 = ""
输出：true


提示：

0 <= s1.length, s2.length <= 100
0 <= s3.length <= 200
s1、s2、和 s3 都由小写英文字母组成


进阶：您能否仅使用 O(s2.length) 额外的内存空间来解决它?
 */
struct Solution;

/*
dp[i][j] 表示 s1的前i个字符和s2的前j个字符能否交错成 s3的前 i+j个字符

转移方程
dp[i][j] = (s1[i-1]==s3[i+j-1] && dp[i-1][j]) || (s2[j-1]==s3[i+j-1] && dp[i][j-1])

初始状态
dp[0][0] = true
dp[0][j] = dp[0][j-1] && s2[j-1] == s3[j-1]
dp[i][0] = dp[i-1][0] && s1[i-1] == s3[i-1]

 */
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m + n != s3.len() {
            return false;
        }
        let (s1, s2, s3) = (s1.into_bytes(), s2.into_bytes(), s3.into_bytes());
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 1..=m {
            if s1[i - 1] != s3[i - 1] {
                break;
            }
            dp[i][0] = true;
        }
        for j in 1..=n {
            if s2[j - 1] != s3[j - 1] {
                break;
            }
            dp[0][j] = true;
        }
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = (s1[i - 1] == s3[i + j - 1] && dp[i - 1][j])
                    || (s2[j - 1] == s3[i + j - 1] && dp[i][j - 1]);
            }
        }
        dp[m][n]
    }
}