#![allow(dead_code)]


use std::iter;

struct Solution;
/*
67. 二进制求和
简单
1.1K
相关企业
给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。



示例 1：

输入:a = "11", b = "1"
输出："100"
示例 2：

输入：a = "1010", b = "1011"
输出："10101"


提示：

1 <= a.length, b.length <= 104
a 和 b 仅由字符 '0' 或 '1' 组成
字符串如果不是 "0" ，就不含前导零
 */
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        // 确保 a <= b
        if a.len() > b.len() {
            let temp = a;
            a = b;
            b = temp;
        }
        let mut vec = Vec::new();
        let mut carry = 0;
        for (x, y) in a.chars().rev()
            .chain(iter::repeat('0').take(b.len() - a.len()))
            .zip(b.chars().rev())
        {
            let x = x as u8 - b'0';
            let y = y as u8 - b'0';
            vec.push((x ^ y ^ carry + b'0') as char);
            carry = if x + y + carry > 1 { 1 } else { 0 };
        }
        // 最后的进位
        if carry > 0 {
            vec.push('1');
        }
        vec.iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::string::add_binary::Solution;

    #[test]
    pub fn t1() {
        let string = Solution::add_binary("100".into(), "110010".into());
        println!("{}", string);
    }
}

