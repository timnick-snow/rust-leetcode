#![allow(dead_code)]

/*
413. 等差数列划分
中等
相关标签
相关企业
如果一个数列 至少有三个元素 ，并且任意两个相邻元素之差相同，则称该数列为等差数列。

例如，[1,3,5,7,9]、[7,7,7,7] 和 [3,-1,-5,-9] 都是等差数列。
给你一个整数数组 nums ，返回数组 nums 中所有为等差数组的 子数组 个数。

子数组 是数组中的一个连续序列。



示例 1：

输入：nums = [1,2,3,4]
输出：3
解释：nums 中有三个子等差数组：[1, 2, 3]、[2, 3, 4] 和 [1,2,3,4] 自身。
示例 2：

输入：nums = [1]
输出：0


提示：

1 <= nums.length <= 5000
-1000 <= nums[i] <= 1000
 */
struct Solution;
/*
长度为k的等差数列可以划分的子数组个数为 1+2+3+...+(k-2) = (k-1)(k-2)/2
 */
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        let mut ans = 0;
        let mut k = 2;
        for i in 2..n {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                k += 1;
            } else {
                ans += (k - 1) * (k - 2) / 2;
                k = 2;
            }
        }
        ans + (k - 1) * (k - 2) / 2
    }
}