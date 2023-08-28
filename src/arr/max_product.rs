#![allow(dead_code)]
/*
152. 乘积最大子数组
中等
2.1K
相关企业
给你一个整数数组 nums ，请你找出数组中乘积最大的非空连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。

测试用例的答案是一个 32-位 整数。

子数组 是数组的连续子序列。



示例 1:

输入: nums = [2,3,-2,4]
输出: 6
解释: 子数组 [2,3] 有最大乘积 6。
示例 2:

输入: nums = [-2,0,-1]
输出: 0
解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。


提示:

1 <= nums.length <= 2 * 104
-10 <= nums[i] <= 10
nums 的任何前缀或后缀的乘积都 保证 是一个 32-位 整数
 */
struct Solution;

/*
和最大子数组和不同的是，这里的因数有负数，这意味着，我们要保存
以num[i-1]结尾的子数组的最小乘积和最大乘积，它们都有可能和num[i]相乘后得到最大值
 */

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut min, mut max) = (nums[0], nums[0]);
        let mut ans = nums[0];
        for x in nums.into_iter().skip(1) {
            if x < 0 {
                let temp = min;
                min = std::cmp::min(max * x, x);
                max = std::cmp::max(temp * x, x);
            } else {
                min = std::cmp::min(min * x, x);
                max = std::cmp::max(max * x, x);
            }
            ans = std::cmp::max(ans, max);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::max_product::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(nums), 6);
    }

    #[test]
    pub fn t2() {
        let nums = vec![-2, 0, -1];
        assert_eq!(Solution::max_product(nums), 0);
    }

    #[test]
    pub fn t3() {
        let nums = vec![1, 3, -2, -4, -1];
        assert_eq!(Solution::max_product(nums), 24);
    }

    #[test]
    pub fn t4() {
        let nums = vec![2, 3, -2, 4, -1];
        assert_eq!(Solution::max_product(nums), 48);
    }
}