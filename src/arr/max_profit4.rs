#![allow(dead_code)]
/*
188. 买卖股票的最佳时机 IV
困难
1K
相关企业
给你一个整数数组 prices 和一个整数 k ，其中 prices[i] 是某支给定的股票在第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。也就是说，你最多可以买 k 次，卖 k 次。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。



示例 1：

输入：k = 2, prices = [2,4,1]
输出：2
解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
示例 2：

输入：k = 2, prices = [3,2,6,5,0,3]
输出：7
解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
     随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。


提示：

1 <= k <= 100
1 <= prices.length <= 1000
0 <= prices[i] <= 1000
 */
struct Solution;

/*
buy[i][j] 是第i天时，进行恰好 j 笔交易，并且当前手上持有一支股票，此时的最大利润
sell[i][j] 是第i天时，进行恰好 j 笔交易，并且当前手上不持有股票，此时的最大利润

状态转移
buy[i][j] 考虑第i天的行为，可能购入一支股票，也可能什么都不做
购入股票：那么其在i-1天时，进行了j笔交易并且不持有股票，即 sell[i-1][j]，那么 buy[i][j] = sell[i-1][j] - prices[i]
啥也不做：那么其在i-1天时，进行了j笔交易并且持有股票，即 buy[i-1][j]，那么 buy[i][j] = buy[i-1][j]
所以，buy[i][j] = max(buy[i-1][j], sell[i-1][j] - prices[i])

sell[i][j] 考虑第i天的行为，可能卖出一支股票，也可能什么都不做
卖出股票：那么其在第i-1天时，进行了j-1笔交易并且持有股票，即 buy[i-1][j-1]，那么 sell[i][j] = buy[i-1][j-1]+prices[i]
啥也不做：那么其在i-1天时，进行了j笔交易并且不持有股票，即 sell[i-1][j]，那么 sell[i][j] = sell[i-1][j]
所以，sell[i][j] = max(sell[i-1][j], buy[i-1][j-1] + prices[i])

初始状态
buy[0][0] = -price[0]
sell[0][0] = 0

j>0时: buy[0][j] 和 sell[0][j] 无意义
i>0时:
    buy[i][0] = max(buy[i-1][0], sell[i-1][0] - prices[i])
    sell[i][0] = 0
 */
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let k = std::cmp::min(k as usize, len >> 1);
        let mut buy = vec![vec![0; k + 1]; len];
        let mut sell = vec![vec![0; k + 1]; len];
        // 初始状态设置
        buy[0][0] = -prices[0];
        for j in 1..k + 1 {
            buy[0][j] = i32::MIN / 2;
            sell[0][j] = i32::MIN / 2;
        }
        for i in 1..len {
            buy[i][0] = std::cmp::max(buy[i - 1][0], sell[i - 1][0] - prices[i])
        }
        // 计算转移
        for i in 1..len {
            for j in 1..k + 1 {
                buy[i][j] = std::cmp::max(buy[i - 1][j], sell[i - 1][j] - prices[i]);
                sell[i][j] = std::cmp::max(sell[i - 1][j], buy[i - 1][j - 1] + prices[i]);
            }
        }
        *sell[len - 1].iter().max().unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::arr::max_profit4::Solution;

    #[test]
    pub fn t1() {
        let k = 2;
        let price = vec![3, 2, 6, 5, 0, 3];
        let profit = Solution::max_profit(k, price);
        println!("{}", profit);
    }
}