#![allow(dead_code)]
/*
334. 递增的三元子序列
中等
相关标签
相关企业
给你一个整数数组 nums ，判断这个数组中是否存在长度为 3 的递增子序列。

如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k ，使得 nums[i] < nums[j] < nums[k] ，返回 true ；否则，返回 false 。



示例 1：

输入：nums = [1,2,3,4,5]
输出：true
解释：任何 i < j < k 的三元组都满足题意
示例 2：

输入：nums = [5,4,3,2,1]
输出：false
解释：不存在满足题意的三元组
示例 3：

输入：nums = [2,1,5,0,4,6]
输出：true
解释：三元组 (3, 4, 5) 满足题意，因为 nums[3] == 0 < nums[4] == 4 < nums[5] == 6


提示：

1 <= nums.length <= 5 * 105
-231 <= nums[i] <= 231 - 1


进阶：你能实现时间复杂度为 O(n) ，空间复杂度为 O(1) 的解决方案吗？
 */
struct Solution;
/*
单调栈
 */
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut stack = Vec::new();
        let mut sec = i32::MAX;
        for x in nums.into_iter() {
            if x > sec {
                return true;
            }
            while !stack.is_empty() && x <= *stack.last().unwrap() {
                stack.pop();
            }
            stack.push(x);
            if stack.len() == 2 {
                sec = sec.min(x);
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::arr::increasing_triplet::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let ans = Solution::increasing_triplet(nums);
        assert!(ans)
    }
}