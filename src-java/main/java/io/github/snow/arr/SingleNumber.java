package io.github.snow.arr;

/*
260. 只出现一次的数字 III
中等
相关标签
相关企业
给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。

你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。



示例 1：

输入：nums = [1,2,1,3,2,5]
输出：[3,5]
解释：[5, 3] 也是有效的答案。
示例 2：

输入：nums = [-1,0]
输出：[-1,0]
示例 3：

输入：nums = [0,1]
输出：[1,0]


提示：

2 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
除两个只出现一次的整数外，nums 中的其他数字都出现两次
 */

/**
 * 260. 只出现一次的数字 III
 *
 * @author snow
 * @since 2023/10/17
 */
public class SingleNumber {
    /*
     * 假设数组 nums 中只出现一次的元素分别是x和y
     * 把 nums 中的所有元素全部异或起来，得到结果xor，那么有 xor = x^y
     * 显然 xor!=0，否则x=y，与题意不符
     *
     * xor&(-xor) 取出异或和的最低位1，设为第l位。那么x和y的第l位其中一个是1，另一个是0。
     * 这样一来，我们就可以把nums中的所有元素分成两类，其中一类包含所有二进制表示的第l位是1，另一类的第l位是0
     *
     */
    static class Solution {
        public int[] singleNumber(int[] nums) {
            int xor = 0;
            for (int num : nums) {
                xor ^= num;
            }
            // 防止溢出
            int lowbit = xor == Integer.MIN_VALUE ? xor : xor & (-xor);
            int type1 = 0, type2 = 0;
            for (int num : nums) {
                if ((num & lowbit) == 0) {
                    type1 ^= num;
                } else {
                    type2 ^= num;
                }
            }
            return new int[]{type1, type2};
        }
    }
}
