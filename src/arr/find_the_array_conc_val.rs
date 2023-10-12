#![allow(dead_code)]
/*
2562. 找出数组的串联值
简单
相关标签
相关企业
提示
给你一个下标从 0 开始的整数数组 nums 。

现定义两个数字的 串联 是由这两个数值串联起来形成的新数字。

例如，15 和 49 的串联是 1549 。
nums 的 串联值 最初等于 0 。执行下述操作直到 nums 变为空：

如果 nums 中存在不止一个数字，分别选中 nums 中的第一个元素和最后一个元素，将二者串联得到的值加到 nums 的 串联值 上，然后从 nums 中删除第一个和最后一个元素。
如果仅存在一个元素，则将该元素的值加到 nums 的串联值上，然后删除这个元素。
返回执行完所有操作后 nums 的串联值。



示例 1：

输入：nums = [7,52,2,4]
输出：596
解释：在执行任一步操作前，nums 为 [7,52,2,4] ，串联值为 0 。
 - 在第一步操作中：
我们选中第一个元素 7 和最后一个元素 4 。
二者的串联是 74 ，将其加到串联值上，所以串联值等于 74 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [52,2] 。
 - 在第二步操作中：
我们选中第一个元素 52 和最后一个元素 2 。
二者的串联是 522 ，将其加到串联值上，所以串联值等于 596 。
接着我们从 nums 中移除这两个元素，所以 nums 变为空。
由于串联值等于 596 ，所以答案就是 596 。
示例 2：

输入：nums = [5,14,13,8,12]
输出：673
解释：在执行任一步操作前，nums 为 [5,14,13,8,12] ，串联值为 0 。
- 在第一步操作中：
我们选中第一个元素 5 和最后一个元素 12 。
二者的串联是 512 ，将其加到串联值上，所以串联值等于 512 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [14,13,8] 。
- 在第二步操作中：
我们选中第一个元素 14 和最后一个元素 8 。
二者的串联是 148 ，将其加到串联值上，所以串联值等于 660 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [13] 。
- 在第三步操作中：
nums 只有一个元素，所以我们选中 13 并将其加到串联值上，所以串联值等于 673 。
接着我们从 nums 中移除这个元素，所以 nums 变为空。
由于串联值等于 673 ，所以答案就是 673 。


提示：

1 <= nums.length <= 1000
1 <= nums[i] <= 104
 */
struct Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let connect = |a: i32, b: i32| {
            a * 10_i32.pow(b.ilog10() + 1) + b
            // a * 10_i32.pow((b as f64).log10() as u32 + 1) + b
        };
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        let mut ans: i64 = 0;
        while left <= right {
            if left == right {
                ans += nums[left] as i64;
                break;
            } else {
                ans += connect(nums[left], nums[right]) as i64
            }
            left += 1;
            right -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::find_the_array_conc_val::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![7, 52, 2, 4];
        println!("{}", Solution::find_the_array_conc_val(nums));
    }
}