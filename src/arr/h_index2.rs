#![allow(dead_code)]
/*
275. H 指数 II
中等
相关标签
相关企业
提示
给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数，citations 已经按照 升序排列 。计算并返回该研究者的 h 指数。

h 指数的定义：h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （n 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。

请你设计并实现对数时间复杂度的算法解决此问题。



示例 1：

输入：citations = [0,1,3,5,6]
输出：3
解释：给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
     由于研究者有 3 篇论文每篇 至少 被引用了 3 次，其余两篇论文每篇被引用 不多于 3 次，所以她的 h 指数是 3 。
示例 2：

输入：citations = [1,2,100]
输出：2


提示：

n == citations.length
1 <= n <= 105
0 <= citations[i] <= 1000
citations 按 升序排列
 */
struct Solution;
/*
二分法

对于引用次数 arr[mid], 至少有 n - mid 篇论文
 */
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len() as i32;
        let (mut left, mut right) = (0, n - 1);
        while left < right {
            let mid = (left + right + 1) >> 1;
            if n - mid > citations[mid as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        n - left
    }
}