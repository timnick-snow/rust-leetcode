#![allow(dead_code)]
/*
128. 最长连续序列
中等
1.8K
相关企业
给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。

请你设计并实现时间复杂度为 O(n) 的算法解决此问题。



示例 1：

输入：nums = [100,4,200,1,3,2]
输出：4
解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
示例 2：

输入：nums = [0,3,7,2,5,8,4,6,0,1]
输出：9


提示：

0 <= nums.length <= 105
-109 <= nums[i] <= 109
 */

struct Solution;
/*
最简单的方法是 排序后 一次遍历求出最长序列

可以使用 Map 来降低复杂度
key: 序列起始值
value: 以key为起始值时，原数组的最长序列

示例1 nums = [100,4,200,1,3,2]
将得到如下map
{
 1: 4,
 100: 1,
 200: 1
}
我们的key没有2,3,4，因为它们不是序列最低点。
只需要最长序列，可以省略实际的map存储
 */
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = nums.iter().cloned().collect::<HashSet<i32>>();
        let mut ans = 0;
        for x in nums.into_iter() {
            if set.contains(&(x - 1)) {
                // x不是序列最低点
                continue;
            }
            // 找到最低点，开始求出以该最低点开始的最长序列
            let mut count = 1;
            set.remove(&x);
            for i in x + 1.. {
                if set.remove(&i) {
                    count += 1;
                } else {
                    break;
                }
            }
            ans = std::cmp::max(ans, count);
        }
        ans
    }
}