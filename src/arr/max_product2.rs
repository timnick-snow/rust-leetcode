#![allow(dead_code)]
/*
318. 最大单词长度乘积
中等
相关标签
相关企业
给你一个字符串数组 words ，找出并返回 length(words[i]) * length(words[j]) 的最大值，并且这两个单词不含有公共字母。如果不存在这样的两个单词，返回 0 。



示例 1：

输入：words = ["abcw","baz","foo","bar","xtfn","abcdef"]
输出：16
解释：这两个单词为 "abcw", "xtfn"。
示例 2：

输入：words = ["a","ab","abc","d","cd","bcd","abcd"]
输出：4
解释：这两个单词为 "ab", "cd"。
示例 3：

输入：words = ["a","aa","aaa","aaaa"]
输出：0
解释：不存在这样的两个单词。


提示：

2 <= words.length <= 1000
1 <= words[i].length <= 1000
words[i] 仅包含小写字母
 */
struct Solution;
/*
用位运算保存每个单词具有的小写字母种类
 */

const ALL_MASK: i32 = 0x1ff_ffff;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut kind = vec![0; n];
        for i in 0..n {
            kind[i] = get_mask(&words[i]);
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 1..n {
                if kind[i] & kind[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}

fn get_mask(s: &str) -> i32 {
    let mut mask = 0;
    for &x in s.as_bytes() {
        mask |= 1 << (x - b'a') as i32;
        if mask == ALL_MASK {
            break;
        }
    }
    mask
}