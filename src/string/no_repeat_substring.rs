#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;
/*
给定一个字符串 s ，请你找出其中不含有重复字符的最长子串的长度。

示例1:

输入: s = "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是"wke"，所以其长度为 3。
  请注意，你的答案必须是 子串 的长度，"pwke"是一个子序列，不是子串。

提示：
0 <= s.length <= 5 * 104
s由英文字母、数字、符号和空格组成
 */

impl Solution {
    pub fn length_of_longest_substring1(s: String) -> i32 {
        if s.len() < 2 {
            s.len() as i32
        } else {
            let mut map = HashMap::new();
            let mut left = 0;
            let mut res = 0;
            'outer:
            while left < s.len() {
                let mut right = left;
                while right < s.len() {
                    if let Some(index) = map.get(&s.as_bytes()[right]) {
                        // 出现重复字符
                        // 更新长度
                        res = max(res, (right - left) as i32);
                        left = index + 1;
                        map.clear();
                        continue 'outer;
                    } else {
                        map.insert(s.as_bytes()[right], right);
                        right += 1;
                    }
                }
                res = max(res, (right - left) as i32);
                break;
            }
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn assert_test(s: &str, expect: i32) {
        let len = Solution::length_of_longest_substring1(s.to_string());
        assert_eq!(len, expect);
    }

    #[test]
    fn example1() {
        assert_test("abcdef", 6);
    }

    #[test]
    fn example2() {
        assert_test("abcabcbb", 3);
    }

    #[test]
    fn example3() {
        assert_test("bbbbb", 1);
    }

    #[test]
    fn example4() {
        assert_test("pwwkew", 3);
    }
}