package io.github.snow.dynamic;
/*
1155. 掷骰子等于目标和的方法数
中等
相关标签
相关企业
提示
这里有 n 个一样的骰子，每个骰子上都有 k 个面，分别标号为 1 到 k 。

给定三个整数 n ,  k 和 target ，返回可能的方式(从总共 kn 种方式中)滚动骰子的数量，使正面朝上的数字之和等于 target 。

答案可能很大，你需要对 109 + 7 取模 。



示例 1：

输入：n = 1, k = 6, target = 3
输出：1
解释：你扔一个有 6 个面的骰子。
得到 3 的和只有一种方法。
示例 2：

输入：n = 2, k = 6, target = 7
输出：6
解释：你扔两个骰子，每个骰子有 6 个面。
得到 7 的和有 6 种方法：1+6 2+5 3+4 4+3 5+2 6+1。
示例 3：

输入：n = 30, k = 30, target = 500
输出：222616187
解释：返回的结果必须是对 109 + 7 取模。


提示：

1 <= n, k <= 30
1 <= target <= 1000
 */

import org.junit.jupiter.api.Test;

/**
 * 1155. 掷骰子等于目标和的方法数
 *
 * @author snow
 * @since 2023/10/24
 */
public class NumRollsToTarget {
    static class Solution {
        static final int MOD = 1000000007;

        public int numRollsToTarget(int n, int k, int target) {
            int[][] dp = new int[2][target + 1];
            dp[0][0] = 1;
            for (int i = 1; i <= n; i++) {
                for (int j = 0; j <= target; j++) {
                    int sum = 0;
                    for (int x = 1; x <= k; x++) {
                        if (j >= x) {
                            sum = (sum + dp[(i - 1) % 2][j - x]) % MOD;
                        }
                    }
                    dp[i % 2][j] = sum;
                }
            }
            return dp[n % 2][target];
        }
    }

    static Solution solution = new Solution();

    @Test
    public void fun1() throws Exception {
        int ans = solution.numRollsToTarget(10, 10, 50);
        System.out.println(ans);
    }
}
