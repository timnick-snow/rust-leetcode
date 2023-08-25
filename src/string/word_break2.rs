#![allow(dead_code)]
/*
140. 单词拆分 II
困难
713
相关企业
给定一个字符串 s 和一个字符串字典 wordDict ，在字符串 s 中增加空格来构建一个句子，使得句子中所有的单词都在词典中。以任意顺序 返回所有这些可能的句子。

注意：词典中的同一个单词可能在分段中被重复使用多次。



示例 1：

输入:s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
输出:["cats and dog","cat sand dog"]
示例 2：

输入:s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
输出:["pine apple pen apple","pineapple pen apple","pine applepen apple"]
解释: 注意你可以重复使用字典中的单词。
示例 3：

输入:s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
输出:[]


提示：

1 <= s.length <= 20
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 10
s 和 wordDict[i] 仅有小写英文字母组成
wordDict 中所有字符串都 不同
 */

use std::collections::HashSet;

struct Solution;

/*
回溯
    选择
    递归
    取消选择

s的长度不超过20，单词的长度不超过10
 */
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let word_dict = word_dict.into_iter().collect::<HashSet<_>>();
        back_trace(&s, &mut String::new(), &word_dict, &mut ans);
        ans
    }
}

fn back_trace(s: &str, cur: &mut String, word_dict: &HashSet<String>, ans: &mut Vec<String>) {
    if s.len() == 0 {
        let mut sentence = cur.clone();
        sentence.pop();
        ans.push(sentence);
        return;
    }
    // 选择一个前缀满足单词  m为选择的前缀长度
    let mut m = 1;
    while m <= s.len() && m <= 10 {
        if word_dict.contains(&s[0..m]) {
            // 选择
            let old_len = cur.len();
            cur.push_str(&s[0..m]);
            cur.push(' ');
            // 递归
            back_trace(&s[m..], cur, word_dict, ans);
            // 取消选择
            cur.truncate(old_len);
        }
        m += 1;
    }
}
