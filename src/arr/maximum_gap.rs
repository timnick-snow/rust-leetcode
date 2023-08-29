#![allow(dead_code)]
/*
164. 最大间距
困难
577
相关企业
给定一个无序的数组 nums，返回 数组在排序之后，相邻元素之间最大的差值 。如果数组元素个数小于 2，则返回 0 。

您必须编写一个在「线性时间」内运行并使用「线性额外空间」的算法。



示例 1:

输入: nums = [3,6,9,1]
输出: 3
解释: 排序后的数组是 [1,3,6,9], 其中相邻元素 (3,6) 和 (6,9) 之间都存在最大差值 3。
示例 2:

输入: nums = [10]
输出: 0
解释: 数组元素个数小于 2，因此返回 0。


提示:

1 <= nums.length <= 105
0 <= nums[i] <= 109
 */
struct Solution;

/*
最大值max, 最小值min
则ans >= (max-min)/(n-1)

取整数 d = (max−min)/(N−1)
将整个区间划分为若干个大小为 d 的桶，并找出每个整数所在的桶。根据前面的结论，能够知道，元素之间的最大间距一定不会出现在某个桶的内部，而一定会出现在不同桶当中。

维护每个桶内元素的最大值与最小值。用后一个桶的最小值与前一个桶的最大值之差作为两个桶的间距(相邻元素的差值)

 */
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        // 每个桶的大小
        let d = std::cmp::max(1, (max - min) / (n as i32 - 1));
        // 桶的总数
        let bucket_size = ((max - min) / d + 1) as usize;
        let mut buket = vec![vec![-1; 2]; bucket_size];
        for x in nums.into_iter() {
            let idx = ((x - min) / d) as usize;
            if buket[idx][0] == -1 {
                buket[idx][0] = x;
                buket[idx][1] = x;
            } else {
                buket[idx][0] = std::cmp::min(buket[idx][0], x);
                buket[idx][1] = std::cmp::max(buket[idx][1], x);
            }
        }

        let mut ans = 0;
        let mut pre: Option<usize> = None;
        for i in 0..bucket_size {
            if buket[i][0] == -1 {
                continue;
            }
            if pre.is_some() {
                ans = std::cmp::max(ans, buket[i][0] - buket[pre.unwrap()][1]);
            }
            pre = Some(i);
        }

        ans
    }


    pub fn maximum_gap_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut ans = 0;
        for i in 1..nums.len() {
            ans = std::cmp::max(ans, nums[i] - nums[i - 1]);
        }
        ans
    }
}