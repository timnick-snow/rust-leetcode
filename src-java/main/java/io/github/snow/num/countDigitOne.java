package io.github.snow.num;

import java.util.Arrays;

/*
233. 数字 1 的个数
困难
相关标签
相关企业
提示
给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。



示例 1：

输入：n = 13
输出：6
示例 2：

输入：n = 0
输出：0


提示：

0 <= n <= 109
 */

/**
 * @author snow
 * @since 2024/1/16
 */
public class countDigitOne {
    static class Solution {
        char[] s;
        int[][] dp;

        public int countDigitOne(int n) {
            s = String.valueOf(n).toCharArray();
            int m = s.length;
            dp = new int[m][m];
            for (int i = 0; i < m; i++) {
                Arrays.fill(dp[i], -1);
            }
            return f(0, 0, true);
        }

        private int f(int i, int cnt, boolean isLimit) {
            if (i == s.length) {
                return cnt;
            }
            if (!isLimit && dp[i][cnt] != -1) {
                return dp[i][cnt];
            }
            // 计算
            int res = 0;
            int up = isLimit ? s[i] - '0' : 9;
            for (int d = 0; d <= up; d++) {
                res += f(i + 1, d == 1 ? cnt + 1 : cnt, isLimit && d == up);
            }
            if (!isLimit) {
                dp[i][cnt] = res;
            }
            return res;
        }
    }

    public static void main(String[] args) {
        System.out.println(new Solution().countDigitOne(12));
    }
}
