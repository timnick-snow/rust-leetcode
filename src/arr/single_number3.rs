#![allow(dead_code)]
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
struct Solution;

/*
假设唯一数字为x,y
将数组是所有元素异或后得到 sum = x^y

lsb = sum & (-sum) 最低位1

 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        for &x in nums.iter() {
            sum ^= x;
        }
        let lsb = if sum == i32::MIN { sum } else { sum & (-sum) };
        let (mut a, mut b) = (0, 0);
        for &x in nums.iter() {
            if (lsb & x) == 0 {
                a ^= x;
            } else {
                b ^= x;
            }
        }
        vec![a, b]
    }
}