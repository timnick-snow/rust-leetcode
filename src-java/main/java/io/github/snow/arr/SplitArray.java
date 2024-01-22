package io.github.snow.arr;
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

/**
 * @author snow
 * @since 2024/1/21
 */
public class SplitArray {
/*
 「将数组分割为 m 段，求……」是动态规划题目常见的问法。

dp[i][j] 表示 前i个数，分隔为j段的最小值
考虑最后一段第j段的分隔范围

dp[i][j] = Min_all{ max(dp[x][j-1], sum(x+1,i)) }

i个数要分成j段非空子数组，必要条件是 i>=j 对于i<j的非法情况，初始化为大值

dp[0][0] = 0
 */


    static class Solution {
        public int splitArray(int[] nums, int k) {
            int n = nums.length;
            int[][] dp = new int[n + 1][k + 1];
            for (int i = 0; i < n + 1; i++) {
                for (int j = 0; j < k + 1; j++) {
                    dp[i][j] = Integer.MAX_VALUE;
                }
            }
            dp[0][0] = 0;

            int[] sum = new int[n + 1];
            for (int i = 0; i < n; i++) {
                sum[i + 1] = sum[i] + nums[i];
            }

            for (int i = 1; i <= n; i++) {
                int jMax = Math.min(i, k);
                for (int j = 1; j <= jMax; j++) {
                    for (int x = 0; x < i; x++) {
                        int value = Math.max(dp[x][j - 1], sum[i] - sum[x]);
                        dp[i][j] = Math.min(dp[i][j], value);
                    }
                }
            }
            return dp[n][k];
        }
    }
}
