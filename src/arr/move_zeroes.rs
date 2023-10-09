#![allow(dead_code)]
/*
283. 移动零
简单
相关标签
相关企业
提示
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

请注意 ，必须在不复制数组的情况下原地对数组进行操作。



示例 1:

输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]
示例 2:

输入: nums = [0]
输出: [0]


提示:

1 <= nums.length <= 104
-231 <= nums[i] <= 231 - 1


进阶：你能尽量减少完成的操作次数吗？
 */
struct Solution;
/*
指针p指向0首个元素
 */
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        // 初始化p
        let mut p = 0;
        while p < n && nums[p] != 0 {
            p += 1;
        }
        // 当前指针cur
        let mut cur = p + 1;
        while cur < n {
            if nums[cur] != 0 {
                // swap
                nums[p] = nums[cur];
                nums[cur] = 0;
                p += 1;
                cur += 1;
            } else {
                cur += 1;
            }
        }
    }
}