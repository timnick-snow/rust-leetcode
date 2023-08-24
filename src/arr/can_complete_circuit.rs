#![allow(dead_code)]
/*
134. 加油站
中等
1.4K
相关企业
在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。

你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。

给定两个整数数组 gas 和 cost ，如果你可以按顺序绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。



示例 1:

输入: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
输出: 3
解释:
从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
因此，3 可为起始索引。
示例 2:

输入: gas = [2,3,4], cost = [3,4,3]
输出: -1
解释:
你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
因此，无论怎样，你都不可能绕环路行驶一周。


提示:

gas.length == n
cost.length == n
1 <= n <= 105
0 <= gas[i], cost[i] <= 104
 */
struct Solution;
/*
profit[i] 表示从i加油站加油并开往下一个加油站，汽油的盈余情况
profit[i] = gas[i] - cost[i]

从任意一个盈余大于等于0的加油站点i开始出发，到达的最远车站为j
if j=i-1, 那么可以行驶一周
else, 从 i..j点的任意一点出发都不可能行驶一周。这是因为，到达任意一点时，必须保持汽油盈余始终>=0，而如果直接从其中的任意
一点出发，其汽油都是0，因此不可能行驶的更远。此时，更换 i=j+1继续进行搜索
 */
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let profit = gas.into_iter().zip(cost.into_iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<_>>();
        let mut i = 0;
        'OUTER:
        while i < n {
            while i < n && profit[i] < 0 {
                i += 1;
            }
            if i == n {
                break;
            }
            let mut rest = 0;
            for (j, &x) in profit[i..].iter().chain(profit[0..i].iter()).enumerate() {
                rest += x;
                if rest < 0 {
                    i += j + 1;
                    continue 'OUTER;
                }
            }
            return i as i32;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::arr::can_complete_circuit::Solution;

    #[test]
    pub fn t1() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![4, 4, 1, 5, 1];
        let ans = Solution::can_complete_circuit(gas, cost);
        assert_eq!(ans, 4);
    }
}