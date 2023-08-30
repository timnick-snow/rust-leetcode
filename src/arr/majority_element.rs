#![allow(dead_code)]
/*
169. 多数元素
简单
1.9K
相关企业
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。



示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2


提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109


进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
 */
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut count = 0;
        for x in nums.into_iter() {
            if count == 0 {
                ans = x;
                count += 1;
            } else {
                if x == ans {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }
        ans
    }
}