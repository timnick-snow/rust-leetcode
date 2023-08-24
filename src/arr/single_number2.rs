#![allow(dead_code)]
/*
137. 只出现一次的数字 II
中等
1K
相关企业
给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。



示例 1：

输入：nums = [2,2,3,2]
输出：3
示例 2：

输入：nums = [0,1,0,1,0,1,99]
输出：99


提示：

1 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
nums 中，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次
 */
struct Solution;
/*
统计32位二进制中，每一位中1的数量，然后对3取余就是结果中该位的值
 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut arr = [0; 32];
        // 统计各个二进制位
        for i in 0..32 {
            for &x in nums.iter() {
                arr[i] += (x >> i) & 1;
            }
        }
        let mut ans = 0;
        for i in 0..32 {
            ans |= (arr[i] % 3) << i;
        }
        ans
    }

    pub fn single_number2(nums: Vec<i32>) -> i32 {
        let mut two = 0;
        let mut one = 0;
        for &x in nums.iter() {
            one = (one ^ x) & !two;
            two = (two ^ x) & !one;
        }
        one
    }
}