package io.github.snow.num;

/*
600. 不含连续1的非负整数
困难
相关标签
相关企业
给定一个正整数 n ，请你统计在 [0, n] 范围的非负整数中，有多少个整数的二进制表示中不存在 连续的 1 。



示例 1:

输入: n = 5
输出: 5
解释:
下面列出范围在 [0, 5] 的非负整数与其对应的二进制表示：
0 : 0
1 : 1
2 : 10
3 : 11
4 : 100
5 : 101
其中，只有整数 3 违反规则（有两个连续的 1 ），其他 5 个满足规则。
示例 2:

输入: n = 1
输出: 2
示例 3:

输入: n = 2
输出: 3


提示:

1 <= n <= 109
 */

import java.util.Arrays;

/**
 * @author snow
 * @since 2024/1/16
 */
public class FindIntegers {
    static class Solution {
        // i,digit
        int[][] dp;
        char[] s;

        public int findIntegers(int n) {
            s = Integer.toString(n, 2).toCharArray();
            dp = new int[s.length][2];
            for (int i = 0; i < dp.length; i++) {
                Arrays.fill(dp[i], -1);
            }
            return f(0, 0, true);
        }

        private int f(int i, int mask, boolean isLimit) {
            if (i == s.length) {
                return 1;
            }
            int pre = mask == 0 ? 0 : mask >> (s.length - i) & 1;
            if (!isLimit && dp[i][pre] != -1) {
                return dp[i][pre];
            }
            int res = 0;
            int up = isLimit ? s[i] - '0' : 1;
            for (int d = 0; d <= up; d++) {
                if (pre == 1 && d == 1) {
                    continue;
                }
                res += f(i + 1, mask | (d << s.length - 1 - i), isLimit && d == up);
            }
            if (!isLimit) {
                dp[i][pre] = res;
            }
            return res;
        }
    }

    public static void main(String[] args) {
        Solution s = new Solution();
        // 1 0 0 0

        // 0 1 2 3
        // 0 0 0 0 x
        // 0 0 0 1
        // 0 0 1 0 x
        // 0 1 0 0 x
        // 0 1 0 1
        // 1 0 0 0
        System.out.println(s.findIntegers(8));
    }
}
