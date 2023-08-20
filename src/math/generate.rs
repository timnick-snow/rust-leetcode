#![allow(dead_code)]
/*
118. 杨辉三角
简单
1K
相关企业
给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。





示例 1:

输入: numRows = 5
输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
示例 2:

输入: numRows = 1
输出: [[1]]


提示:

1 <= numRows <= 30
 */
struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let first = vec![1];
        if num_rows == 1 {
            return vec![first];
        }
        let mut ans = Vec::new();
        ans.push(first);
        let mut list = &ans[0];
        for _ in 1..num_rows {
            let mut next = vec![1];
            for i in 0..list.len() - 1 {
                next.push(list[i] + list[i + 1]);
            }
            next.push(1);
            ans.push(next);
            list = ans.last().as_ref().unwrap();
        }
        ans
    }
}