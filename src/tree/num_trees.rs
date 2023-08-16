#![allow(dead_code)]

struct Solution;

/*
96. 不同的二叉搜索树
中等
2.3K
相关企业
给你一个整数 n ，求恰由 n 个节点组成且节点值从 1 到 n 互不相同的 二叉搜索树 有多少种？返回满足题意的二叉搜索树的种数。



示例 1：


输入：n = 3
输出：5
示例 2：

输入：n = 1
输出：1


提示：

1 <= n <= 19
 */
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as u64;
        if n == 1 {
            return 1;
        }
        let mut ans = 1;
        for i in 0..n {
            ans = ans * (4 * i + 2) / (i + 2)
        }
        ans as i32
    }
}