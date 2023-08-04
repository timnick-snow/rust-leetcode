#![allow(dead_code)]
struct Solution;

/*
给你一个字符串 s，找到 s 中最长的回文子串。

如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。

示例 1：

输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
示例 2：

输入：s = "cbbd"
输出："bb"

提示：

1 <= s.length <= 1000
s 仅由数字和英文字母组成

链接：https://leetcode.cn/problems/longest-palindromic-substring
 */
impl Solution {
    pub fn longest_palindrome(_s: String) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(&Solution::longest_palindrome("babad".to_string()), "bab");
    }


}


