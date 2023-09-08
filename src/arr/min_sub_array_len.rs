#![allow(dead_code)]
/*
209. 长度最小的子数组
中等
1.9K
相关企业
给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其总和大于等于 target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。
如果不存在符合条件的子数组，返回 0 。



示例 1：

输入：target = 7, nums = [2,3,1,2,4,3]
输出：2
解释：子数组 [4,3] 是该条件下的长度最小的子数组。
示例 2：

输入：target = 4, nums = [1,4,4]
输出：1
示例 3：

输入：target = 11, nums = [1,1,1,1,1,1,1,1]
输出：0


提示：

1 <= target <= 109
1 <= nums.length <= 105
1 <= nums[i] <= 105


进阶：

如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。
 */
struct Solution;

/*
滑动窗口
 */
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, 0);
        let mut ans = i32::MAX;
        // 滑动窗口内的子数组和
        let mut sum = 0;
        while l < n && r < n {
            sum += nums[r];
            // println!("window: {:?}, sum: {}", (l, r), sum);
            if sum == target {
                // 找到一组解
                ans = std::cmp::min(ans, (r - l + 1) as i32);
                if ans == 1 {
                    break;
                }
                sum -= nums[l];
                l += 1;
                r += 1;
            } else if sum < target {
                // 当前窗口和小了 右边扩大窗口
                r += 1;
            } else {
                ans = std::cmp::min(ans, (r - l + 1) as i32);
                if ans == 1 {
                    break;
                }
                // 当前窗口和大了 左边缩小窗口
                if l == r {
                    l += 1;
                    r += 1;
                    sum = 0;
                } else {
                    sum -= nums[l];
                    sum -= nums[r];
                    l += 1;
                }
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::min_sub_array_len::Solution;

    #[test]
    pub fn t1() {
        // [2,3,1,2,4,3]  7
        let target = 7;
        let arr = vec![2, 3, 1, 2, 4, 3];
        let ans = Solution::min_sub_array_len(target, arr);
        assert_eq!(ans, 2);
    }

    #[test]
    pub fn t2() {
        // [1,4,4] 4
        let target = 4;
        let arr = vec![1, 4, 4];
        let ans = Solution::min_sub_array_len(target, arr);
        assert_eq!(ans, 1);
    }

    #[test]
    pub fn t3() {
        // [1,1,1,1,1,1,1,1] 11
        let target = 11;
        let arr = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let ans = Solution::min_sub_array_len(target, arr);
        assert_eq!(ans, 0);
    }

    #[test]
    pub fn t4() {
        // [1,1,1,1,1,1,1,1] 11
        let target = 11;
        let arr = vec![1,2,3,4,5];
        let ans = Solution::min_sub_array_len(target, arr);
        assert_eq!(ans, 0);
    }
}