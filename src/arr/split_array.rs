#![allow(dead_code)]
/*
410. 分割数组的最大值
困难
相关标签
相关企业
给定一个非负整数数组 nums 和一个整数 k ，你需要将这个数组分成 k 个非空的连续子数组。

设计一个算法使得这 k 个子数组各自和的最大值最小。



示例 1：

输入：nums = [7,2,5,10,8], k = 2
输出：18
解释：
一共有四种方法将 nums 分割为 2 个子数组。
其中最好的方式是将其分为 [7,2,5] 和 [10,8] 。
因为此时这两个子数组各自的和的最大值为18，在所有情况中最小。
示例 2：

输入：nums = [1,2,3,4,5], k = 2
输出：9
示例 3：

输入：nums = [1,4,4], k = 3
输出：4


提示：

1 <= nums.length <= 1000
0 <= nums[i] <= 106
1 <= k <= min(50, nums.length)
 */
struct Solution;
/*
「将数组分割为 m 段，求……」是动态规划题目常见的问法。

dp[i][j] 表示 前i个数，分隔为j段的最小值
考虑最后一段第j段的分隔范围

dp[i][j] = Min_all{ max(dp[x][j-1], sub_sum(x+1,i)) }

i个数要分成j段非空子数组，必要条件是 i>=j 对于i<j的非法情况，初始化为大值

dp[0][0] = 0
 */
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![vec![i32::MAX; k + 1]; n + 1];
        dp[0][0] = 0;

        // 前缀和
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i];
        }

        for i in 1..n + 1 {
            let j_max = k.min(i);
            for j in 1..j_max + 1 {
                for x in 0..i {
                    let a = dp[x][j - 1];
                    let b = sum[i] - sum[x];
                    dp[i][j] = dp[i][j].min(a.max(b))
                }
            }
        }

        dp[n][k]
    }
}

#[cfg(test)]
mod test {
    use crate::arr::split_array::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![7, 2, 5, 10, 8];
        let k = 2;
        let ans = Solution::split_array(nums, k);
        println!("{}", ans);
    }
}