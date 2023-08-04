#![allow(dead_code)]
/*
32. 最长有效括号
困难
2.3K
相关企业
给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。



示例 1：

输入：s = "(()"
输出：2
解释：最长有效括号子串是 "()"
示例 2：

输入：s = ")()())"
输出：4
解释：最长有效括号子串是 "()()"
示例 3：

输入：s = ""
输出：0


提示：

0 <= s.length <= 3 * 104
s[i] 为 '(' 或 ')'
 */
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut deque = VecDeque::new();
        deque.push_back(-1);
        let mut max = 0;
        for (i, x) in s.chars().enumerate() {
            match x {
                '(' => {
                    deque.push_back(i as i32);
                }
                ')' => {
                    deque.pop_back();
                    if deque.is_empty() {
                        deque.push_back(i as i32);
                        continue;
                    }
                    let len = i as i32 - deque.back().unwrap();
                    max = std::cmp::max(max, len);
                }
                _ => unreachable!()
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic::longest_valid_parentheses::Solution;

    #[test]
    fn t1() {
        let a = Solution::longest_valid_parentheses("(()))()()()".into());
        assert_eq!(a, 6);
    }

    #[test]
    fn t2() {
        let a = Solution::longest_valid_parentheses("()(()".into());
        assert_eq!(a, 2);
    }

    #[test]
    fn t3() {
        let a = Solution::longest_valid_parentheses("(())))()()".into());
        assert_eq!(a, 4);
    }
}