#![allow(dead_code)]
/*
670. 最大交换
中等
相关标签
相关企业
给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。

示例 1 :

输入: 2736
输出: 7236
解释: 交换数字2和数字7。
示例 2 :

输入: 9973
输出: 9973
解释: 不需要交换。
注意:

给定数字的范围是 [0, 108]
 */
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        if num == 0 { return 0; }

        let mut deque = VecDeque::new();
        let mut x = num;
        while x > 0 {
            deque.push_front(x % 10);
            x /= 10;
        }
        let mut arr = Vec::from(deque);
        let n = arr.len();

        let mut ans = num;
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if arr[j] > arr[i] {
                    ans = ans.max(swap_value(&mut arr, i, j));
                }
            }
        }
        ans
    }
}

fn swap_value(arr: &mut [i32], i: usize, j: usize) -> i32 {
    arr.swap(i, j);
    let mut ans = 0;
    arr.iter().for_each(|&x| ans = ans * 10 + x);
    arr.swap(i, j);
    ans
}

#[cfg(test)]
mod test {
    use crate::num::maximum_swap::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::maximum_swap(98368);
        println!("{}", ans);
    }
}