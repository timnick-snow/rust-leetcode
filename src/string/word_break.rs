#![allow(dead_code)]
/*
139. 单词拆分
中等
2.3K
相关企业
给你一个字符串 s 和一个字符串列表 wordDict 作为字典。请你判断是否可以利用字典中出现的单词拼接出 s 。

注意：不要求字典中出现的单词全部都使用，并且字典中的单词可以重复使用。



示例 1：

输入: s = "leetcode", wordDict = ["leet", "code"]
输出: true
解释: 返回 true 因为 "leetcode" 可以由 "leet" 和 "code" 拼接成。
示例 2：

输入: s = "applepenapple", wordDict = ["apple", "pen"]
输出: true
解释: 返回 true 因为 "applepenapple" 可以由 "apple" "pen" "apple" 拼接成。
     注意，你可以重复使用字典中的单词。
示例 3：

输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
输出: false


提示：

1 <= s.length <= 300
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 20
s 和 wordDict[i] 仅由小写英文字母组成
wordDict 中的所有字符串 互不相同
 */
use std::collections::HashSet;

struct Solution;
/*
动态规划
dp[i]表示字符串s的前i个字符能否表示拆分
dp[i] = dp[i-m] && s[i-m..i]属于字典中的单词
其中m为字典中单词的长度，由于单词长度最大为20，因此 1 <= m <= 20 且 m <= i

s = "leetcode"， wordDict = ["leet", "code"]

dp[0] = true
dp[4] = true, 因为当m=4时有，dp[4-4] = true 并且 s[4-4..4] = "leet" 属于字典中的单词
dp[8] = true, 因为当m=4时有，dp[8-4] = true 并且 s[8-4..8] = "code" 属于字典中的单词

s = "applepenapple", wordDict = ["apple", "pen"]
dp[0] = true
dp[5] = true, 因为当m=5时有，dp[5-5] = true 并且 s[5-5..5] = "apple" 属于字典中的单词
dp[8] = true, 因为当m=3时有，dp[8-3] = true 并且 s[8-3..8] = "pen" 属于字典中的单词
dp[13] = true, 因为当m=5时有，dp[13-5] = true 并且 s[13-5..13] = "apple" 属于字典中的单词
 */
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..n + 1 {
            for j in 1..=i {
                // 每个单词最大长度为20 因此超过20长度时直接跳出循环
                if j > 20 {
                    break;
                }
                if word_dict.contains(&s[i - j..i]) && dp[i - j] {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}