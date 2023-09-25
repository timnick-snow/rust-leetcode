#![allow(dead_code)]
/*
239. 滑动窗口最大值
困难
相关标签
相关企业
提示
给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。

返回 滑动窗口中的最大值 。



示例 1：

输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
输出：[3,3,5,5,6,7]
解释：
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
示例 2：

输入：nums = [1], k = 1
输出：[1]


提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104
1 <= k <= nums.length
 */
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut deque = VecDeque::new();
        for i in 0..k {
            while !deque.is_empty() && nums[i] > nums[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(i);
        }

        let mut ans = Vec::new();
        ans.push(nums[*deque.front().unwrap()]);
        // 滑动窗口 [i-k+1, i]
        for i in k..n {
            while !deque.is_empty() && nums[i] > nums[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(i);
            while *deque.front().unwrap() < i - k + 1 {
                deque.pop_front();
            }
            ans.push(nums[*deque.front().unwrap()]);
        }
        ans
    }
}
