#![allow(dead_code)]
/*
689. 三个无重叠子数组的最大和
困难
相关标签
相关企业
给你一个整数数组 nums 和一个整数 k ，找出三个长度为 k 、互不重叠、且全部数字和（3 * k 项）最大的子数组，并返回这三个子数组。

以下标的数组形式返回结果，数组中的每一项分别指示每个子数组的起始位置（下标从 0 开始）。如果有多个结果，返回字典序最小的一个。



示例 1：

输入：nums = [1,2,1,2,6,7,5,1], k = 2
输出：[0,3,5]
解释：子数组 [1, 2], [2, 6], [7, 5] 对应的起始下标为 [0, 3, 5]。
也可以取 [2, 1], 但是结果 [1, 3, 5] 在字典序上更大。
示例 2：

输入：nums = [1,2,1,2,1,2,1,2,1], k = 2
输出：[0,2,4]


提示：

1 <= nums.length <= 2 * 104
1 <= nums[i] < 216
1 <= k <= floor(nums.length / 3)
 */
struct Solution;
/*
若不考虑输出方案，仅是求「三个无重叠子数组的最大和」的最大值。

只需要使用动态规划求解即可：定义 f[i][j] 为考虑前 i 个数，凑成无重叠子数组数量为 j 个时的最大值。

最终答案为 f[n−1][3]

考虑计算 f[i][j]
对于num[i]，可以挑选，也可以不选
1. 选择num[i]，那么最后一个子数组位[i-k+1, i]，我们需要再[0,i-k]中选择j-1个子数组
    f[i][j] = f[i-k][j-1] + rang_sum(i-k+1,i)

2. 不选择num[i]，我们需要再[0,i-1]中选择j个子数组
    f[i][j] = f[i-1][j]

最终f[i][j]为两者较大值


 */
impl Solution {
    pub fn max_sum_of_three_subarrays(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        nums.reverse();
        // 求前缀和
        let mut sum = vec![0_i64; n + 1];
        for i in 1..=n {
            sum[i] = sum[i - 1] + nums[i - 1] as i64;
        }
        // 动态规划
        let mut f = vec![vec![0_i64; 4]; n + 1];
        for i in k..=n {
            for j in 1..=3 {
                let a = f[i - k][j - 1] + sum[i] - sum[i - k];
                let b = f[i - 1][j];
                f[i][j] = a.max(b);
            }
        }

        // 回溯出具体数组
        let mut ans = vec![0; 3];
        let (mut i, mut j) = (n, 3);
        let mut idx = 0;
        while idx < 3 {
            if f[i - 1][j] > f[i - k][j - 1] + sum[i] - sum[i - k] {
                // 这意味着 num[i]不在最大子数组中
                i -= 1;
            } else {
                ans[idx] = (n - i) as i32;
                i -= k;
                j -= 1;
                idx += 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use crate::arr::max_sum_of_three_subarrays::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let ans = Solution::max_sum_of_three_subarrays(nums, 2);
        println!("{:?}", ans);
    }
}