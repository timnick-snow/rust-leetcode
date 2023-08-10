#![allow(dead_code)]
/*
72. 编辑距离
困难
3.1K
相关企业
给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。

你可以对一个单词进行如下三种操作：

插入一个字符
删除一个字符
替换一个字符


示例 1：

输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
示例 2：

输入：word1 = "intention", word2 = "execution"
输出：5
解释：
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')


提示：

0 <= word1.length, word2.length <= 500
word1 和 word2 由小写英文字母组成
 */
struct Solution;
/*
操作的等效性:
word1中增加一个字符 <=> word2中删除一个字符
word1中删除一个字符 <=> word2中增加一个字符
word1中替换一个字符 <=> word2中替换一个字符

dp[i][j] 表示word1的前i个字符编辑为word2的前j个字符的最小操作数

dp[i][j] <= dp[i-1][j]+1
dp[i][j] <= dp[i][j-1]+1
dp[i][j] <= dp[i-1][j-1]+1

dp[0][0] = 0
dp[i][0] = i
dp[0][j] = j
 */
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        if m == 0 || n == 0 {
            return std::cmp::max(m, n) as i32;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        // init
        dp[0][0] = 0;
        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }
        for i in 0..word1.len() {
            for j in 0..word2.len() {
                let m1 = std::cmp::min(dp[i][j + 1] + 1, dp[i + 1][j] + 1);
                let m2 = dp[i][j] + if word1[i] == word2[j] { 0 } else { 1 };
                dp[i + 1][j + 1] = std::cmp::min(m1, m2);
                println!("{:?} = {}", (i + 1, j + 1), dp[i + 1][j + 1]);
            }
        }
        dp[m][n] as i32
    }
}


#[cfg(test)]
mod test {
    use crate::dynamic::min_distance::Solution;

    #[test]
    fn t1() {
        let ans = Solution::min_distance("pneumonoultramicroscopicsilicovolcanoconiosis".into(), "ultramicroscopically".into());
        assert_eq!(ans, 27)
    }
}