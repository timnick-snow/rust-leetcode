#![allow(dead_code)]
/*
2865. 美丽塔 I
中等
相关标签
相关企业
提示
给你一个长度为 n 下标从 0 开始的整数数组 maxHeights 。

你的任务是在坐标轴上建 n 座塔。第 i 座塔的下标为 i ，高度为 heights[i] 。

如果以下条件满足，我们称这些塔是 美丽 的：

1 <= heights[i] <= maxHeights[i]
heights 是一个 山脉 数组。
如果存在下标 i 满足以下条件，那么我们称数组 heights 是一个 山脉 数组：

对于所有 0 < j <= i ，都有 heights[j - 1] <= heights[j]
对于所有 i <= k < n - 1 ，都有 heights[k + 1] <= heights[k]
请你返回满足 美丽塔 要求的方案中，高度和的最大值 。



示例 1：

输入：maxHeights = [5,3,4,1,1]
输出：13
解释：和最大的美丽塔方案为 heights = [5,3,3,1,1] ，这是一个美丽塔方案，因为：
- 1 <= heights[i] <= maxHeights[i]
- heights 是个山脉数组，峰值在 i = 0 处。
13 是所有美丽塔方案中的最大高度和。
示例 2：

输入：maxHeights = [6,5,3,9,2,7]
输出：22
解释： 和最大的美丽塔方案为 heights = [3,3,3,9,2,2] ，这是一个美丽塔方案，因为：
- 1 <= heights[i] <= maxHeights[i]
- heights 是个山脉数组，峰值在 i = 3 处。
22 是所有美丽塔方案中的最大高度和。
示例 3：

输入：maxHeights = [3,2,5,5,2,3]
输出：18
解释：和最大的美丽塔方案为 heights = [2,2,5,5,2,2] ，这是一个美丽塔方案，因为：
- 1 <= heights[i] <= maxHeights[i]
- heights 是个山脉数组，最大值在 i = 2 处。
注意，在这个方案中，i = 3 也是一个峰值。
18 是所有美丽塔方案中的最大高度和。


提示：

1 <= n == maxHeights <= 103
1 <= maxHeights[i] <= 109
 */

struct Solution;
/*
单调栈
 */
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut prefix = vec![0_i64; n];
        let mut suffix = vec![0_i64; n];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            while !stack.is_empty() && max_heights[i] < max_heights[*stack.last().unwrap()] {
                stack.pop();
            }
            if stack.is_empty() {
                prefix[i] = (i as i64 + 1) * max_heights[i] as i64;
            } else {
                let j = *stack.last().unwrap();
                prefix[i] = prefix[j] + (i - j) as i64 * max_heights[i] as i64;
            }
            stack.push(i);
        }
        stack.clear();
        for i in (0..n).rev() {
            while !stack.is_empty() && max_heights[i] < max_heights[*stack.last().unwrap()] {
                stack.pop();
            }
            if stack.is_empty() {
                suffix[i] = (n - i) as i64 * max_heights[i] as i64;
            } else {
                let j = *stack.last().unwrap();
                suffix[i] = suffix[j] + (j - i) as i64 * max_heights[i] as i64;
            }
            stack.push(i);
        }
        println!("{:?}", prefix);
        println!("{:?}", suffix);
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::maximum_sum_of_heights::Solution;

    #[test]
    pub fn t1() {
        let arr = vec![3, 2, 5, 5, 2, 3];
        let ans = Solution::maximum_sum_of_heights(arr);
        println!("{}", ans);
    }
}