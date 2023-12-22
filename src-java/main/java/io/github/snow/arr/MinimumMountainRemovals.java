package io.github.snow.arr;

import org.junit.jupiter.api.Test;

import java.util.Arrays;
/*
1671. 得到山形数组的最少删除次数
已解答
困难
相关标签
相关企业
提示
我们定义 arr 是 山形数组 当且仅当它满足：

arr.length >= 3
存在某个下标 i （从 0 开始） 满足 0 < i < arr.length - 1 且：
arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
给你整数数组 nums​ ，请你返回将 nums 变成 山形状数组 的​ 最少 删除次数。



示例 1：

输入：nums = [1,3,1]
输出：0
解释：数组本身就是山形数组，所以我们不需要删除任何元素。
示例 2：

输入：nums = [2,1,1,5,6,2,3,1]
输出：3
解释：一种方法是将下标为 0，1 和 5 的元素删除，剩余元素为 [1,5,6,3,1] ，是山形数组。


提示：

3 <= nums.length <= 1000
1 <= nums[i] <= 109
题目保证 nums 删除一些元素后一定能得到山形数组。
 */

/**
 * 1671. 得到山形数组的最少删除次数
 * @author snow
 * @since 2023/12/22
 */
public class MinimumMountainRemovals {

    static class Solution {
        public int minimumMountainRemovals(int[] nums) {
            int n = nums.length;
            int[] f = new int[n];
            int[] g = new int[n];

            for (int i = 1; i < n; i++) {
                for (int j = i - 1; j >= 0; j--) {
                    if (nums[i] > nums[j]) {
                        f[i] = Math.max(f[i], f[j] + 1);
                    }
                }
            }

            for (int i = n - 2; i >= 0; i--) {
                for (int j = i + 1; j < n; j++) {
                    if (nums[i] > nums[j]) {
                        g[i] = Math.max(g[i], g[j] + 1);
                    }
                }
            }

            System.out.println(Arrays.toString(f));
            System.out.println(Arrays.toString(g));
            int res = n;
            for (int i = 1; i < n - 1; i++) {
                if (f[i] > 0 && g[i] > 0) {
                    // n - (f+g+1)
                    res = Math.min(res, n - (f[i] + g[i] + 1));
                }
            }
            return res;
        }
    }

    @Test
    public void fun1() throws Exception {
        int[] nums = {100, 92, 89, 77, 74, 66, 64, 66, 64};
        int res = new Solution().minimumMountainRemovals(nums);
        System.out.println(res);
    }
}
