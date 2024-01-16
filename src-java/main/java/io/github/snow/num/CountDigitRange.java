package io.github.snow.num;

import java.util.Arrays;

/*
2719. 统计整数数目
困难
相关标签
相关企业
提示
给你两个数字字符串 num1 和 num2 ，以及两个整数 max_sum 和 min_sum 。如果一个整数 x 满足以下条件，我们称它是一个好整数：

num1 <= x <= num2
min_sum <= digit_sum(x) <= max_sum.
请你返回好整数的数目。答案可能很大，请返回答案对 109 + 7 取余后的结果。

注意，digit_sum(x) 表示 x 各位数字之和。



示例 1：

输入：num1 = "1", num2 = "12", min_num = 1, max_num = 8
输出：11
解释：总共有 11 个整数的数位和在 1 到 8 之间，分别是 1,2,3,4,5,6,7,8,10,11 和 12 。所以我们返回 11 。
示例 2：

输入：num1 = "1", num2 = "5", min_num = 1, max_num = 5
输出：5
解释：数位和在 1 到 5 之间的 5 个整数分别为 1,2,3,4 和 5 。所以我们返回 5 。


提示：

1 <= num1 <= num2 <= 1022
1 <= min_sum <= max_sum <= 400
 */


/**
 * @author snow
 * @since 2024/1/16
 */
public class CountDigitRange {
    static class Solution {
        int[][] dp;
        int n;
        char[] s;
        int mod = 1_000_000_007;

        public int count(String num1, String num2, int min_sum, int max_sum) {
            int a = calc(num1, min_sum, max_sum);
            int b = calc(num2, min_sum, max_sum);
            int ans = (b - a + mod) % mod;
            int sum = 0;
            for (int i = 0; i < num1.length(); i++) {
                sum += num1.charAt(i) - '0';
            }
            if (sum >= min_sum && sum <= max_sum) {
                ans++;
            }
            return ans;
        }

        private int calc(String num, int minSum, int maxSum) {
            // 数字长度
            n = num.length();
            dp = new int[n][maxSum + 1];
            s = num.toCharArray();
            for (int i = 0; i < n; i++) {
                Arrays.fill(dp[i], -1);
            }
            return f(0, 0, true, minSum, maxSum);
        }

        private int f(int i, int sum, boolean isLimit, int minSum, int maxSum) {
            if (sum > maxSum) {
                // 当前数位和已超出限制 不合法
                return 0;
            }
            if (i == n) {
                // 已经填完所有的数位
                return sum >= minSum ? 1 : 0;
            }
            if (!isLimit && dp[i][sum] != -1) {
                // 已经计算过
                return dp[i][sum];
            }
            // 计算
            int res = 0;
            int up = isLimit ? s[i] - '0' : 9;
            for (int d = 0; d <= up; d++) {
                res = (res + f(i + 1, sum + d, isLimit && d == up, minSum, maxSum)) % mod;
            }
            if (!isLimit) {
                dp[i][sum] = res;
            }
            return res;
        }
    }


    public static void main(String[] args) {
        String n1 = "4179205230";
        String n2 = "7748704426";
        int v = new Solution().count(n1, n2, 8, 46);
        System.out.println(v);
    }
}
