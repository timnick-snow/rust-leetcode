#![allow(dead_code)]
/*
415. 字符串相加
简单
相关标签
相关企业
给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和并同样以字符串形式返回。

你不能使用任何內建的用于处理大整数的库（比如 BigInteger）， 也不能直接将输入的字符串转换为整数形式。



示例 1：

输入：num1 = "11", num2 = "123"
输出："134"
示例 2：

输入：num1 = "456", num2 = "77"
输出："533"
示例 3：

输入：num1 = "0", num2 = "0"
输出："0"




提示：

1 <= num1.length, num2.length <= 104
num1 和num2 都只包含数字 0-9
num1 和num2 都不包含任何前导零
 */
struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let m = num1.len();
        let n = num2.len();

        let mut it1 = num1.chars();
        let mut it2 = num2.chars();
        let mut ans = String::new();

        let mut carry = 0;
        for _ in 0..m.max(n) {
            let a = it1.next_back().map(|x| x as u8 - b'0').unwrap_or(0);
            let b = it2.next_back().map(|x| x as u8 - b'0').unwrap_or(0);
            let value = (a + b + carry) % 10;
            carry = (a + b + carry) / 10;
            ans.push((value + b'0') as char);
        }
        if carry > 0 {
            ans.push((carry + b'0') as char);
        }
        ans.chars().rev().collect()
    }
}