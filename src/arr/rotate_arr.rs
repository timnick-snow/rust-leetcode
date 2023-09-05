#![allow(dead_code)]
/*
189. 轮转数组
提示
中等
1.9K
相关企业
给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。



示例 1:

输入: nums = [1,2,3,4,5,6,7], k = 3
输出: [5,6,7,1,2,3,4]
解释:
向右轮转 1 步: [7,1,2,3,4,5,6]
向右轮转 2 步: [6,7,1,2,3,4,5]
向右轮转 3 步: [5,6,7,1,2,3,4]
示例 2:

输入：nums = [-1,-100,3,99], k = 2
输出：[3,99,-1,-100]
解释:
向右轮转 1 步: [99,-1,-100,3]
向右轮转 2 步: [3,99,-1,-100]


提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
0 <= k <= 105


进阶：

尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
 */
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        let mut arr = Vec::new();
        for i in n - k..n {
            arr.push(nums[i]);
        }
        for i in 0..n - k {
            arr.push(nums[i]);
        }
        for i in 0..n {
            nums[i] = arr[i];
        }
    }
    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        fn reverse(arr: &mut Vec<i32>, mut start: usize, mut end: usize) {
            while start < end {
                let temp = arr[start];
                arr[start] = arr[end];
                arr[end] = temp;
                start += 1;
                end -= 1;
            }
        }
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }
}