#![allow(dead_code)]
/*
123. 买卖股票的最佳时机 III
困难
1.5K
相关企业
给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。



示例 1:

输入：prices = [3,3,5,0,0,3,1,4]
输出：6
解释：在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
     随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。
示例 2：

输入：prices = [1,2,3,4,5]
输出：4
解释：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
     注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
     因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
示例 3：

输入：prices = [7,6,4,3,1]
输出：0
解释：在这个情况下, 没有交易完成, 所以最大利润为 0。
示例 4：

输入：prices = [1]
输出：0


提示：

1 <= prices.length <= 10^5
0 <= prices[i] <= 10^5
 */
struct Solution;
/*
跳过头部的降序序列 首个波谷出现前的波峰无意义
跳过最后的降序序列 最后一个波峰后的波谷无意义

首先寻找波峰、波谷的位置，并将其保存到数组 lower， higher
lower[i] 表示第i个波谷的值
higher[i] 表示第i个波峰的值
我们会跳过无意义的波峰和波谷，因此lower.length = higher.length

我们可以做两次交易，
p[i] 表示第i次波谷前完成一次交易的最大收益
q[i] 从第i次波谷开始完成一次交易的最大收益

求 max(p[i] + q[i])
 */
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n <= 1 {
            return 0;
        }
        let mut lower = Vec::new();
        let mut higher = Vec::new();
        let mut i = 0;
        while i < n {
            // 寻找波谷
            while i + 1 < n && prices[i + 1] <= prices[i] {
                i += 1;
            }
            if i == n - 1 {
                // 结尾处的波谷无意义
                break;
            }
            lower.push(prices[i]);
            // 寻找波峰
            while i + 1 < n && prices[i + 1] >= prices[i] {
                i += 1;
            }
            higher.push(prices[i]);
        }
        let n = lower.len();
        if n == 0 {
            return 0;
        }
        // p[i] 第i次波谷前完成一次交易的最大收益
        let mut p = vec![0; n];
        let mut min_price = lower[0];
        for i in 1..n {
            min_price = std::cmp::min(min_price, lower[i - 1]);
            p[i] = std::cmp::max(p[i - 1], higher[i - 1] - min_price);
        }
        // q[i] 从第i次波谷开始完成一次交易的最大收益  逆向遍历求解
        let mut q = vec![0; n];
        let mut max_price = higher[n - 1];
        for i in (0..n).rev() {
            max_price = std::cmp::max(max_price, higher[i]);
            if i == n - 1 {
                q[i] = higher[i] - lower[i];
            } else {
                q[i] = std::cmp::max(q[i + 1], max_price - lower[i]);
            }
        }
        // println!("{:?}", p);
        // println!("{:?}", q);
        // 枚举i值 得到做两次交易的最大收益
        p.into_iter().zip(q.into_iter())
            .map(|(x, y)| x + y)
            .max().unwrap_or(0)
    }
}


#[cfg(test)]
mod test {
    use crate::arr::max_profit3::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]);
        assert_eq!(ans, 13);
    }

    #[test]
    pub fn t2() {
        let ans = Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]);
        assert_eq!(ans, 6);
    }

    #[test]
    pub fn t3() {
        let ans = Solution::max_profit(vec![14, 9, 10, 12, 4, 8, 1, 16]);
        assert_eq!(ans, 19);
    }

    #[test]
    pub fn t4() {
        let ans = Solution::max_profit(vec![1, 2, 3, 4, 5]);
        assert_eq!(ans, 4);
    }

    #[test]
    pub fn t5() {
        let ans = Solution::max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(ans, 0);
    }
}