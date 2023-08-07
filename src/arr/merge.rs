#![allow(dead_code)]
/*
56. 合并区间
中等
2K
相关企业
以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。



示例 1：

输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
示例 2：

输入：intervals = [[1,4],[4,5]]
输出：[[1,5]]
解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。


提示：

1 <= intervals.length <= 104
intervals[i].length == 2
0 <= starti <= endi <= 104
 */

struct Solution;

/*
首先将区间进行排序，优先按区间左值(start)升序排，再按照区间右值(end)降序排。
这么做有以下好处：
1. start升序：可以从左到右依次处理区间
2. end降序：当start相等时，我们优先拿到的是范围最大的区间

如何合并
首先定义一个待定区间 start = intervals[0][0], end = intervals[0][1]
然后依次遍历剩余区间，
如果 intervals[i][0] <= end, 则此区间可以合并到待定区间，变成一个新待定区间 [start, max(end,intervals[i][1])]

如果 intervals[i][0] > end, 那么此区间无法与待定区间合并，并且后续的所有区间也无法与待定区间合并，待定区间确定，加入到结果集。
    更新待定区间 [intervals[i][0], intervals[i][1]]

特别地：当 intervals[i][1]] <= end，则此区间是待定区间的子区间，可忽略处理。
 */
use std::cmp::Ordering;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 按照start升序 end降序 将区间进行排序
        intervals.sort_unstable_by(|a, b|
            match a[0].partial_cmp(&b[0]).unwrap() {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => b[1].partial_cmp(&a[1]).unwrap(),
                Ordering::Greater => Ordering::Greater,
            }
        );
        let mut ans = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[1] <= end {
                continue;
            }
            if interval[0] <= end {
                end = interval[1];
            } else {
                ans.push(vec![start, end]);
                start = interval[0];
                end = interval[1];
            }
        }
        ans.push(vec![start, end]);
        ans
    }
}