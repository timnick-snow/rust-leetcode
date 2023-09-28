#![allow(dead_code)]
/*
2251. 花期内花的数目
困难
相关标签
相关企业
提示
给你一个下标从 0 开始的二维整数数组 flowers ，其中 flowers[i] = [starti, endi] 表示第 i 朵花的 花期 从 starti 到 endi （都 包含）。同时给你一个下标从 0 开始大小为 n 的整数数组 people ，people[i] 是第 i 个人来看花的时间。

请你返回一个大小为 n 的整数数组 answer ，其中 answer[i]是第 i 个人到达时在花期内花的 数目 。



示例 1：



输入：flowers = [[1,6],[3,7],[9,12],[4,13]], people = [2,3,7,11]
输出：[1,2,2,2]
解释：上图展示了每朵花的花期时间，和每个人的到达时间。
对每个人，我们返回他们到达时在花期内花的数目。
示例 2：



输入：flowers = [[1,10],[3,3]], people = [3,3,2]
输出：[2,2,1]
解释：上图展示了每朵花的花期时间，和每个人的到达时间。
对每个人，我们返回他们到达时在花期内花的数目。


提示：

1 <= flowers.length <= 5 * 104
flowers[i].length == 2
1 <= starti <= endi <= 109
1 <= people.length <= 5 * 104
1 <= people[i] <= 109
 */
struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let n = flowers.len();
        let mut start = vec![0; n];
        let mut end = vec![0; n];
        for i in 0..n {
            start[i] = flowers[i][0];
            end[i] = flowers[i][1];
        }
        start.sort_unstable();
        end.sort_unstable();
        let m = people.len();
        let mut ans = vec![0; m];
        for i in 0..m {
            let x = binary_search(&start, people[i] + 1);
            let y = binary_search(&end, people[i]);
            ans[i] = x - y;
        }
        ans
    }
}

fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut res = arr.len();
    let (mut left, mut right) = (0, arr.len() - 1);
    while left <= right {
        let mid = (left + right) >> 1;
        if arr[mid] >= target {
            res = mid;
            match mid.checked_sub(1) {
                None => break,
                Some(x) => {
                    right = x
                }
            }
        } else {
            left = mid + 1;
        }
    }
    res as i32
}

#[cfg(test)]
mod test {
    use crate::arr::full_bloom_flowers::Solution;

    #[test]
    pub fn t1() {
        let flower = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];
        let ans = Solution::full_bloom_flowers(flower, people);
        println!("{:?}", ans);
    }
}