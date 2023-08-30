#![allow(dead_code)]
/*
171. Excel 表列序号
简单
393
相关企业
给你一个字符串 columnTitle ，表示 Excel 表格中的列名称。返回 该列名称对应的列序号 。

例如：

A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...


示例 1:

输入: columnTitle = "A"
输出: 1
示例 2:

输入: columnTitle = "AB"
输出: 28
示例 3:

输入: columnTitle = "ZY"
输出: 701


提示：

1 <= columnTitle.length <= 7
columnTitle 仅由大写英文组成
columnTitle 在范围 ["A", "FXSHRXW"] 内
 */
struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans = 0;
        for (i, &x) in column_title.into_bytes().iter().rev().enumerate() {
            ans += 26_i32.pow(i as u32) * (x - b'A' + 1) as i32;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::math::title_to_number::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::title_to_number("AB".into());
        assert_eq!(ans, 28);
    }
    #[test]
    pub fn t2() {
        let ans = Solution::title_to_number("ZY".into());
        assert_eq!(ans, 701);
    }
}