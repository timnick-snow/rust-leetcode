#![allow(dead_code)]
/*
338. 比特位计数
简单
相关标签
相关企业
提示
给你一个整数 n ，对于 0 <= i <= n 中的每个 i ，计算其二进制表示中 1 的个数 ，返回一个长度为 n + 1 的数组 ans 作为答案。



示例 1：

输入：n = 2
输出：[0,1,1]
解释：
0 --> 0
1 --> 1
2 --> 10
示例 2：

输入：n = 5
输出：[0,1,1,2,1,2]
解释：
0 --> 0
1 --> 1
2 --> 10
3 --> 11
4 --> 100
5 --> 101


提示：

0 <= n <= 105


进阶：

很容易就能实现时间复杂度为 O(n log n) 的解决方案，你可以在线性时间复杂度 O(n) 内用一趟扫描解决此问题吗？
你能不使用任何内置函数解决此问题吗？（如，C++ 中的 __builtin_popcount ）
 */
struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            ans[i] = ans[i & (i - 1)] + 1;
        }
        ans
    }
}