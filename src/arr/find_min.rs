#![allow(dead_code)]
/*
153. 寻找旋转排序数组中的最小值
提示
中等
1K
相关企业
已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。例如，原数组 nums = [0,1,2,4,5,6,7] 在变化后可能得到：
若旋转 4 次，则可以得到 [4,5,6,7,0,1,2]
若旋转 7 次，则可以得到 [0,1,2,4,5,6,7]
注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组 [a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

给你一个元素值 互不相同 的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [3,4,5,1,2]
输出：1
解释：原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。
示例 2：

输入：nums = [4,5,6,7,0,1,2]
输出：0
解释：原数组为 [0,1,2,4,5,6,7] ，旋转 4 次得到输入数组。
示例 3：

输入：nums = [11,13,15,17]
输出：11
解释：原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。


提示：

n == nums.length
1 <= n <= 5000
-5000 <= nums[i] <= 5000
nums 中的所有整数 互不相同
nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转
 */
struct Solution;
/*
0 1 2 3 4 5 6
1 2 3 4 5 6 0
2 3 4 5 6 0 1
3 4 5 6 0 1 2
4 5 6 0 1 2 3
5 6 0 1 2 3 4
6 0 1 2 3 4 5

观察：
先找到中点
1. 如果左侧是升序，右侧也是升序的，那么首位就是最小
2. 左侧升序，右侧不是，找右侧继续找
3. 左侧不是, 右侧升序，左侧(含中点)继续找

由于 mid = (l+r)>>1 的值，当r-l=1时，mid = l
因此，我们先判断 num[mid] 和 num[r]

 */
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return nums[0];
        }
        let (mut l, mut r) = (0, len - 1);
        while l < r {
            let mid = (l + r) >> 1;
            if nums[l] < nums[mid] && nums[mid] < nums[r] {
                break;
            }
            if nums[mid] < nums[r] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        nums[l]
    }
}