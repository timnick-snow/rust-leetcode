#![allow(dead_code)]
/*
402. 移掉 K 位数字
中等
相关标签
相关企业
给你一个以字符串表示的非负整数 num 和一个整数 k ，移除这个数中的 k 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。


示例 1 ：

输入：num = "1432219", k = 3
输出："1219"
解释：移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219 。
示例 2 ：

输入：num = "10200", k = 1
输出："200"
解释：移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
示例 3 ：

输入：num = "10", k = 2
输出："0"
解释：从原数字移除所有的数字，剩余为空就是 0 。


提示：

1 <= k <= num.length <= 105
num 仅由若干位数字（0 - 9）组成
除了 0 本身之外，num 不含任何前导零
 */
struct Solution;
/*
单调栈
 */
impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = Vec::new();

        for x in num.chars() {
            while let Some(&y) = stack.last() {
                if k == 0 || x >= y {
                    break;
                }
                stack.pop();
                k -= 1;
            }
            stack.push(x);
        }

        while k > 0 {
            stack.pop();
            k -= 1;
        }

        let mut ans = stack.into_iter()
            .skip_while(|x| *x == '0')
            .collect::<String>();
        if ans.len() == 0 {
            ans.push('0');
        }
        ans
    }
}