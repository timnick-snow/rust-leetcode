#![allow(dead_code)]
/*
2605. 从两个数字数组里生成最小数字
提示
简单
24
相关企业
给你两个只包含 1 到 9 之间数字的数组 nums1 和 nums2 ，每个数组中的元素 互不相同 ，请你返回 最小 的数字，两个数组都 至少 包含这个数字的某个数位。


示例 1：

输入：nums1 = [4,1,3], nums2 = [5,7]
输出：15
解释：数字 15 的数位 1 在 nums1 中出现，数位 5 在 nums2 中出现。15 是我们能得到的最小数字。
示例 2：

输入：nums1 = [3,5,2,6], nums2 = [3,1,7]
输出：3
解释：数字 3 的数位 3 在两个数组中都出现了。


提示：

1 <= nums1.length, nums2.length <= 9
1 <= nums1[i], nums2[i] <= 9
每个数组中，元素 互不相同 。
 */
struct Solution;
/*
如果两个数组中有相同元素，则最小的相同元素就是答案
否则，取出两个数组各自的最小元素，它们组成的最小两位数就是答案
 */
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 100;
        let mut arr = [0; 10];
        let mut a = nums1[0];
        let mut b = nums2[0];
        for x in nums1.into_iter() {
            arr[x as usize] = 1;
            a = std::cmp::min(a, x);
        }
        for y in nums2.into_iter() {
            if arr[y as usize] == 1 {
                ans = std::cmp::min(ans, y);
            }
            b = std::cmp::min(b, y);
        }
        if ans > 9 {
            ans = std::cmp::min(10 * a + b, 10 * b + a);
        }
        ans
    }
}