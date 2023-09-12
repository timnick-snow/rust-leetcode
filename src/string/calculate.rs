#![allow(dead_code)]
/*
224. 基本计算器
困难
相关标签
相关企业
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

注意:不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。



示例 1：

输入：s = "1 + 1"
输出：2
示例 2：

输入：s = " 2-1 + 2 "
输出：3
示例 3：

输入：s = "(1+(4+5+2)-3)+(6+8)"
输出：23


提示：

1 <= s.length <= 3 * 105
s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
s 表示一个有效的表达式
'+' 不能用作一元运算(例如， "+1" 和 "+(2 + 3)" 无效)
'-' 可以用作一元运算(即 "-1" 和 "-(2 + 3)" 是有效的)
输入中不存在两个连续的操作符
每个数字和运行的计算将适合于一个有符号的 32位 整数
 */

struct Solution;
/*
括号展开
 */
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.into_bytes();
        let mut deque = Vec::new();
        deque.push(1);

        let mut sign = 1;
        let mut ans = 0;
        let mut i = 0;
        while i < s.len() {
            match s[i] {
                b' ' => (),
                b'+' => sign = *deque.last().unwrap(),
                b'-' => sign = -*deque.last().unwrap(),
                b'(' => deque.push(sign),
                b')' => {
                    deque.pop();
                }
                _ => {
                    let mut num = 0_i64;
                    while i < s.len() && matches!(s[i],b'0'..=b'9') {
                        num = num * 10 + s[i] as i64 - b'0' as i64;
                        i += 1;
                    }
                    ans += num * sign;
                    continue;
                }
            }
            i += 1
        }
        ans as i32
    }
}