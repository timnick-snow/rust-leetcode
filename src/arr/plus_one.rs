#![allow(dead_code)]
/*
66. 加一
简单
1.3K
相关企业
给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。

最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。

你可以假设除了整数 0 之外，这个整数不会以零开头。



示例 1：

输入：digits = [1,2,3]
输出：[1,2,4]
解释：输入数组表示数字 123。
示例 2：

输入：digits = [4,3,2,1]
输出：[4,3,2,2]
解释：输入数组表示数字 4321。
示例 3：

输入：digits = [0]
输出：[1]


提示：

1 <= digits.length <= 100
0 <= digits[i] <= 9
 */
struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        for x in digits.iter_mut().rev() {
            let sum = *x + carry;
            *x = sum % 10;
            carry = sum / 10;
            if carry == 0 {
                break;
            }
        }
        if carry == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}