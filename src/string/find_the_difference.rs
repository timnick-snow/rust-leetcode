#![allow(dead_code)]
/*
389. 找不同
简单
相关标签
相关企业
给定两个字符串 s 和 t ，它们只包含小写字母。

字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。

请找出在 t 中被添加的字母。



示例 1：

输入：s = "abcd", t = "abcde"
输出："e"
解释：'e' 是那个被添加的字母。
示例 2：

输入：s = "", t = "y"
输出："y"


提示：

0 <= s.length <= 1000
t.length == s.length + 1
s 和 t 只包含小写字母

 */
struct Solution;
/*
1. 计数统计
2. 求和做差
3. 异或运算
 */
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let xor = s.into_bytes().iter()
            .chain(t.into_bytes().iter())
            .fold(0_u8, |acc, &x| acc ^ x);
        xor as char
    }
}