#![allow(dead_code)]
/*
227. 基本计算器 II
中等
相关标签
相关企业
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

整数除法仅保留整数部分。

你可以假设给定的表达式总是有效的。所有中间结果将在 [-231, 231 - 1] 的范围内。

注意：不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。



示例 1：

输入：s = "3+2*2"
输出：7
示例 2：

输入：s = " 3/2 "
输出：1
示例 3：

输入：s = " 3+5 / 2 "
输出：5


提示：

1 <= s.length <= 3 * 105
s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
s 表示一个 有效表达式
表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内
题目数据保证答案是一个 32-bit 整数
 */
struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.into_bytes();
        let mut stack = Vec::new();
        let mut i = 0;
        // -1: -, 1: +, 2: *, 3: /
        let mut sign = 1;
        while i < s.len() {
            match s[i] {
                b' ' => {
                    i += 1;
                }
                b'+' => {
                    sign = 1;
                    i += 1;
                }
                b'-' => {
                    sign = -1;
                    i += 1;
                }
                b'*' => {
                    sign = 2;
                    i += 1;
                }
                b'/' => {
                    sign = 3;
                    i += 1;
                }
                _ => {
                    let mut num: i32 = 0;
                    while i < s.len() && matches!(s[i],b'0'..=b'9') {
                        num = num * 10 + s[i] as i32 - b'0' as i32;
                        i += 1;
                    }
                    match sign {
                        -1 => stack.push(-num),
                        1 => stack.push(num),
                        2 => {
                            let temp = stack.pop().unwrap() * num;
                            stack.push(temp);
                        }
                        3 => {
                            let temp = stack.pop().unwrap() / num;
                            stack.push(temp);
                        }
                        _ => unreachable!()
                    }
                }
            }
        }
        stack.iter().sum()
    }
}