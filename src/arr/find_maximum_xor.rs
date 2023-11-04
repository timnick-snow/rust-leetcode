#![allow(dead_code)]
/*
421. 数组中两个数的最大异或值
中等
相关标签
相关企业
给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。



示例 1：

输入：nums = [3,10,5,25,2,8]
输出：28
解释：最大运算结果是 5 XOR 25 = 28.
示例 2：

输入：nums = [14,70,53,83,49,91,36,80,92,51,66,70]
输出：127


提示：

1 <= nums.length <= 2 * 105
0 <= nums[i] <= 231 - 1
 */
    use std::collections::HashSet;

struct Solution;
/*
异或的性质
如果  a^b=x
那么  a^x=b  b^x=a
即异或的位置发生任意交换，等式仍然成立

贪心思想，要想最后的异或结果尽可能的大，那么最高位需要极可能多的1
假定结果的高位是1，然后与数组中的元素的【同等前缀】进行异或，看结果是否在数组中
 */
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mask = 0;
        for i in (0..=30).rev() {
            mask |= 1 << i;
            // 保存前缀
            let mut set = HashSet::new();
            nums.iter().for_each(|&x| {
                set.insert(x & mask);
            });

            // 假设第i位是1
            let temp = ans ^ (1 << i);
            for &x in set.iter() {
                if set.contains(&(temp ^ x)) {
                    ans = temp;
                    break;
                }
            }
        }
        ans
    }
}