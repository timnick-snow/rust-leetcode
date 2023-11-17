#![allow(dead_code)]

/*
2736. 最大和查询
困难
相关标签
相关企业
提示
给你两个长度为 n 、下标从 0 开始的整数数组 nums1 和 nums2 ，另给你一个下标从 1 开始的二维数组 queries ，其中 queries[i] = [xi, yi] 。

对于第 i 个查询，在所有满足 nums1[j] >= xi 且 nums2[j] >= yi 的下标 j (0 <= j < n) 中，找出 nums1[j] + nums2[j] 的 最大值 ，如果不存在满足条件的 j 则返回 -1 。

返回数组 answer ，其中 answer[i] 是第 i 个查询的答案。



示例 1：

输入：nums1 = [4,3,1,2], nums2 = [2,4,9,5], queries = [[4,1],[1,3],[2,5]]
输出：[6,10,7]
解释：
对于第 1 个查询：xi = 4 且 yi = 1 ，可以选择下标 j = 0 ，此时 nums1[j] >= 4 且 nums2[j] >= 1 。nums1[j] + nums2[j] 等于 6 ，可以证明 6 是可以获得的最大值。
对于第 2 个查询：xi = 1 且 yi = 3 ，可以选择下标 j = 2 ，此时 nums1[j] >= 1 且 nums2[j] >= 3 。nums1[j] + nums2[j] 等于 10 ，可以证明 10 是可以获得的最大值。
对于第 3 个查询：xi = 2 且 yi = 5 ，可以选择下标 j = 3 ，此时 nums1[j] >= 2 且 nums2[j] >= 5 。nums1[j] + nums2[j] 等于 7 ，可以证明 7 是可以获得的最大值。
因此，我们返回 [6,10,7] 。
示例 2：

输入：nums1 = [3,2,5], nums2 = [2,3,4], queries = [[4,4],[3,2],[1,1]]
输出：[9,9,9]
解释：对于这个示例，我们可以选择下标 j = 2 ，该下标可以满足每个查询的限制。
示例 3：

输入：nums1 = [2,1], nums2 = [2,3], queries = [[3,3]]
输出：[-1]
解释：示例中的查询 xi = 3 且 yi = 3 。对于每个下标 j ，都只满足 nums1[j] < xi 或者 nums2[j] < yi 。因此，不存在答案。


提示：

nums1.length == nums2.length
n == nums1.length
1 <= n <= 105
1 <= nums1[i], nums2[i] <= 109
1 <= queries.length <= 105
queries[i].length == 2
xi == queries[i][1]
yi == queries[i][2]
1 <= xi, yi <= 109
 */
struct Solution;

/*
单调栈 + 二分

nums[1]和nums[2]必须是相同索引上的数才能构成数对作为答案，所以我们先绑定它们，将两个数组合并成一个数组 nums<(x,y)>
再根据x进行降序排序，这样后续的x满足 x <= x'

遍历所有满足 x>=qx 的数对数组。 当我们遍历到一个y时
1. 如果 y <= y' 。 由于 x<=x'，那么(x',y')一定是优于(x,y)的答案，将(x,y)抛弃。  => 引入单调栈
2. 如果 y > y' 。 将(x,y)入栈，同时处理无效答案。 如果栈顶的 x'+y' <= x+y，那么栈顶的数据可以丢弃。

遍历完成后，将得到一个x+y降序且y升序的单调栈，二分找出第一个 y>=qy 的数据即可

该单调栈的内容可继续服务于后续查询，这要求我们后续查询的 qx <= qx'，这样才能满足 x >= qx' >= qx 。即单调栈中之前
遍历的满足条件的x也满足后续qx条件，只要继续往后遍历剩余的满足条件的x即可。

 */
impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 合并成数对数组
        let mut nums: Vec<(i32, i32)> = nums1.into_iter()
            .zip(nums2.into_iter())
            .collect();
        // 按照x降序排列
        nums.sort_unstable_by_key(|x| -x.0);

        let n = queries.len();
        let mut ans = vec![0; n];

        // 索引按照qx降序排列
        let mut idx = (0..n).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| -queries[i][0]);

        // (y,sum)
        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut j = 0;
        for i in idx.into_iter() {
            let qx = queries[i][0];
            let qy = queries[i][1];

            while j < nums.len() && nums[j].0 >= qx {
                let x = nums[j].0;
                let y = nums[j].1;
                while !stack.is_empty() && stack.last().unwrap().1 <= x + y {
                    stack.pop();
                };
                if stack.is_empty() || y > stack.last().unwrap().0 {
                    stack.push((y, x + y));
                }
                j += 1;
            }

            // stack是一个y升序的数组  找到 y>= qy 的第一个位置
            let p = stack.partition_point(|&(y, _)| y < qy);
            ans[i] = if p < stack.len() { stack[p].1 } else { -1 };
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::maximum_sum_queries::Solution;

    #[test]
    pub fn t1() {
        let nums1 = vec![4, 3, 1, 2];
        let nums2 = vec![2, 4, 9, 5];
        let queries = vec![vec![4, 1], vec![1, 3], vec![2, 5]];
        let ans = Solution::maximum_sum_queries(nums1, nums2, queries);
        println!("{:?}", ans);
    }
}