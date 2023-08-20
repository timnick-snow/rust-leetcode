#![allow(dead_code)]
/*
119. 杨辉三角 II
简单
503
相关企业
给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。





示例 1:

输入: rowIndex = 3
输出: [1,3,3,1]
示例 2:

输入: rowIndex = 0
输出: [1]
示例 3:

输入: rowIndex = 1
输出: [1,1]


提示:

0 <= rowIndex <= 33


进阶：

你可以优化你的算法到 O(rowIndex) 空间复杂度吗？
 */
struct Solution;
/*
使用二项式定理
 */
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let mut ans = Vec::new();
        for i in 0..row_index + 1 {
            ans.push(c(row_index, i));
        }
        ans
    }
}

fn c(n: i32, i: i32) -> i32 {
    let n = n as u64;
    let i = i as u64;
    let i = std::cmp::min(i, n - i);
    if i == 0 {
        1
    } else {
        let mut ans = 1_u64;
        for j in 0..i {
            ans = ans * (n - j) / (1 + j);
        }
        ans as i32
    }
}


#[cfg(test)]
mod test {
    use crate::math::get_row::c;

    #[test]
    pub fn t1() {
        println!("{}", c(4, 0));
        println!("{}", c(4, 1));
        println!("{}", c(4, 2));
    }
}

