#![allow(dead_code)]
/*
405. 数字转换为十六进制数
简单
相关标签
相关企业
给定一个整数，编写一个算法将这个数转换为十六进制数。对于负整数，我们通常使用 补码运算 方法。

注意:

十六进制中所有字母(a-f)都必须是小写。
十六进制字符串中不能包含多余的前导零。如果要转化的数为0，那么以单个字符'0'来表示；对于其他情况，十六进制字符串中的第一个字符将不会是0字符。
给定的数确保在32位有符号整数范围内。
不能使用任何由库提供的将数字直接转换或格式化为十六进制的方法。
示例 1：

输入:
26

输出:
"1a"
示例 2：

输入:
-1

输出:
"ffffffff"
 */
struct Solution;

const HEX: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut num = num as u32;
        let mut ans = Vec::new();
        while num != 0 {
            ans.push(HEX[(num & 0xf) as usize]);
            num >>= 4;
        }
        ans.iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::num::to_hex::Solution;

    #[test]
    pub fn t1() {
        let num = -1;
        println!("{}", Solution::to_hex(num));
    }
}