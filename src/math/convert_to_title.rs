#![allow(dead_code)]
/*
168. Excel表列名称

例如：

A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...


示例 1：

输入：columnNumber = 1
输出："A"
示例 2：

输入：columnNumber = 28
输出："AB"
示例 3：

输入：columnNumber = 701
输出："ZY"
示例 4：

输入：columnNumber = 2147483647
输出："FXSHRXW"


提示：

1 <= columnNumber <= 231 - 1
 */
struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ans = String::new();
        let mut x = column_number;
        while x != 0 {
            ans.insert(0, (((x - 1) % 26) as u8 + b'A') as char);
            x = (x - 1) / 26;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::math::convert_to_title::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::convert_to_title(28);
        assert_eq!(&ans, "AB");
    }

    #[test]
    pub fn t2() {
        let ans = Solution::convert_to_title(701);
        assert_eq!(&ans, "ZY");
    }

    #[test]
    pub fn t3() {
        let ans = Solution::convert_to_title(2147483647);
        assert_eq!(&ans, "FXSHRXW");
    }
}