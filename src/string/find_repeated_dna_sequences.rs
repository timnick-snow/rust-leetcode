#![allow(dead_code)]
/*
187. 重复的DNA序列
中等
495
相关企业
DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'.。

例如，"ACGAATTCCG" 是一个 DNA序列 。
在研究 DNA 时，识别 DNA 中的重复序列非常有用。

给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。



示例 1：

输入：s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
输出：["AAAAACCCCC","CCCCCAAAAA"]
示例 2：

输入：s = "AAAAAAAAAAAAA"
输出：["AAAAAAAAAA"]


提示：

0 <= s.length <= 105
s[i]=='A'、'C'、'G' or 'T'
 */
use std::collections::HashSet;

struct Solution;

/*
找重复 去重复  考虑Set
子字符串 考虑 滑动窗口, 由于只需要找长度为10的序列，因此我们保持滑动窗口的大小始终为10
 */
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let len = s.len();
        if len <= 10 {
            return vec![];
        }
        let mut set = HashSet::new();
        let mut ans = HashSet::new();
        let (mut l, mut r) = (0, 9);
        while r < len {
            let candidate = &s[l..r + 1];
            if !set.insert(candidate) {
                ans.insert(candidate.to_string());
            }
            l += 1;
            r += 1;
        }
        ans.into_iter().collect()
    }
}