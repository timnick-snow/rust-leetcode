#![allow(dead_code)]
/*
409. 最长回文串
简单
相关标签
相关企业
给定一个包含大写字母和小写字母的字符串 s ，返回 通过这些字母构造成的 最长的回文串 。

在构造过程中，请注意 区分大小写 。比如 "Aa" 不能当做一个回文字符串。



示例 1:

输入:s = "abccccdd"
输出:7
解释:
我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
示例 2:

输入:s = "a"
输出:1
示例 3：

输入:s = "aaaaaccc"
输出:7


提示:

1 <= s.length <= 2000
s 只由小写 和/或 大写英文字母组成
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = HashMap::new();
        for &x in s.as_bytes() {
            cnt.entry(x).and_modify(|x| *x += 1).or_insert(1);
        }

        let even = cnt.into_values()
            .map(|x| x >> 1 << 1)
            .sum::<i32>();

        (even+1).min(s.len() as i32)
    }
}