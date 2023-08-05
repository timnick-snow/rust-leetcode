#![allow(dead_code)]
/*
53. 最大子数组和
中等
6.2K
相关企业
给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

子数组 是数组中的一个连续部分。



示例 1：

输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
输出：6
解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
示例 2：

输入：nums = [1]
输出：1
示例 3：

输入：nums = [5,4,-1,7,8]
输出：23


提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104


进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
 */
struct Solution;

/*
动态规划
dp[i] 表示所有以nums[i]结尾的子数组和的最大值

状态转移方程:
如果dp[i-1]<=0  那么：dp[i] = nums[i]
如果dp[i-1]>0   那么：dp[i] = dp[i-1]+dp[i]

初始状态 dp[0] = nums[0]

考虑到状态转移方程中只依赖dp[i-1]
可以不需要使用状态数组 只使用变量代替dp[i-1]
 */
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 初始态
        let mut pre = nums[0];
        let mut max = pre;
        nums.iter()
            .skip(1)
            .for_each(|&x| {
                if pre <= 0 {
                    pre = x;
                } else {
                    pre = pre + x;
                }
                max = std::cmp::max(max, pre);
            });
        // 最大值
        max
    }
}


#[cfg(test)]
mod test {
    use crate::num::max_sub_array::Solution;

    #[test]
    fn t1() {
        let ans = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(ans, 6);
    }

}