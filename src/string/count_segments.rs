#![allow(dead_code)]
/*
434. 字符串中的单词数
简单
相关标签
相关企业
统计字符串中的单词个数，这里的单词指的是连续的不是空格的字符。

请注意，你可以假定字符串里不包括任何不可打印的字符。

示例:

输入: "Hello, my name is John"
输出: 5
解释: 这里的单词是指连续的不是空格的字符，所以 "Hello," 算作 1 个单词。
 */
struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut ans = 0;
        let n = s.len();
        for i in 0..n {
            if (i == 0 || s.as_bytes()[i - 1] == b' ')
                && (s.as_bytes()[i] != b' ') {
                ans += 1;
            }
        }
        ans
    }
}