#![allow(dead_code)]
/*
135. 分发糖果
困难
1.3K
相关企业
n 个孩子站成一排。给你一个整数数组 ratings 表示每个孩子的评分。

你需要按照以下要求，给这些孩子分发糖果：

每个孩子至少分配到 1 个糖果。
相邻两个孩子评分更高的孩子会获得更多的糖果。
请你给每个孩子分发糖果，计算并返回需要准备的 最少糖果数目 。



示例 1：

输入：ratings = [1,0,2]
输出：5
解释：你可以分别给第一个、第二个、第三个孩子分发 2、1、2 颗糖果。
示例 2：

输入：ratings = [1,2,2]
输出：4
解释：你可以分别给第一个、第二个、第三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这满足题面中的两个条件。


提示：

n == ratings.length
1 <= n <= 2 * 104
0 <= ratings[i] <= 2 * 104
 */
struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        for i in 0..n {
            // 升序
            if i > 0 && ratings[i] > ratings[i - 1] {
                left[i] = left[i - 1] + 1;
            } else {
                left[i] = 1;
            }
        }
        for i in (0..n).rev() {
            // 降序
            if i < n - 1 && ratings[i] > ratings[i + 1] {
                right[i] = right[i + 1] + 1;
            } else {
                right[i] = 1;
            }
        }
        // println!("{:?}", left);
        // println!("{:?}", right);
        left.iter().zip(right.iter())
            .map(|(&x, &y)| std::cmp::max(x, y))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::arr::candy::Solution;

    #[test]
    pub fn t1() {
        let ratings = vec![1, 0, 2];
        let ans = Solution::candy(ratings);
        assert_eq!(ans, 5);
    }

    #[test]
    pub fn t2() {
        let ratings = vec![1, 2, 4, 3, 2, 1];
        let ans = Solution::candy(ratings);
        assert_eq!(ans, 13);
    }

    #[test]
    pub fn t3() {
        let ratings = vec![1, 2, 4, 6, 5, 3, 2, 2];
        let ans = Solution::candy(ratings);
        assert_eq!(ans, 17);
    }
}