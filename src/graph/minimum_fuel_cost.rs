#![allow(dead_code)]

/*
2477. 到达首都的最少油耗
中等
相关标签
相关企业
提示
给你一棵 n 个节点的树（一个无向、连通、无环图），每个节点表示一个城市，编号从 0 到 n - 1 ，且恰好有 n - 1 条路。0 是首都。给你一个二维整数数组 roads ，其中 roads[i] = [ai, bi] ，表示城市 ai 和 bi 之间有一条 双向路 。

每个城市里有一个代表，他们都要去首都参加一个会议。

每座城市里有一辆车。给你一个整数 seats 表示每辆车里面座位的数目。

城市里的代表可以选择乘坐所在城市的车，或者乘坐其他城市的车。相邻城市之间一辆车的油耗是一升汽油。

请你返回到达首都最少需要多少升汽油。



示例 1：



输入：roads = [[0,1],[0,2],[0,3]], seats = 5
输出：3
解释：
- 代表 1 直接到达首都，消耗 1 升汽油。
- 代表 2 直接到达首都，消耗 1 升汽油。
- 代表 3 直接到达首都，消耗 1 升汽油。
最少消耗 3 升汽油。
示例 2：



输入：roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
输出：7
解释：
- 代表 2 到达城市 3 ，消耗 1 升汽油。
- 代表 2 和代表 3 一起到达城市 1 ，消耗 1 升汽油。
- 代表 2 和代表 3 一起到达首都，消耗 1 升汽油。
- 代表 1 直接到达首都，消耗 1 升汽油。
- 代表 5 直接到达首都，消耗 1 升汽油。
- 代表 6 到达城市 4 ，消耗 1 升汽油。
- 代表 4 和代表 6 一起到达首都，消耗 1 升汽油。
最少消耗 7 升汽油。
示例 3：



输入：roads = [], seats = 1
输出：0
解释：没有代表需要从别的城市到达首都。


提示：

1 <= n <= 105
roads.length == n - 1
roads[i].length == 2
0 <= ai, bi < n
ai != bi
roads 表示一棵合法的树。
1 <= seats <= 105
 */
use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

/*
拓扑排序

官解：贪心 + 深度优先搜索
那么我们可以通过从根结点 0 往下进行「深度优先搜索」，每一条边上「车子」的数目即为该条边上汽油的开销，统计全部边上汽油的开销即为最终答案。

 */
impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut degree = vec![0; n];
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        roads.into_iter().for_each(|x| {
            degree[x[0] as usize] += 1;
            degree[x[1] as usize] += 1;
            map.entry(x[0]).or_default().insert(x[1]);
            map.entry(x[1]).or_default().insert(x[0]);
        });

        let mut ans: i64 = 0;
        // 初始化度为1的城市
        let mut deque = degree.iter().enumerate()
            .filter(|x| *x.1 == 1 && x.0 != 0)
            .map(|x| x.0 as i32)
            .collect::<VecDeque<_>>();
        // 每个城市目前的人数
        let mut people = vec![1; n];
        while !deque.is_empty() {
            let mut temp = VecDeque::new();
            while let Some(x) = deque.pop_front() {
                // 编号x的所有人乘车去当前唯一连接的城市 y
                let y = *map.get_mut(&x).unwrap().iter().next().unwrap();
                degree[x as usize] -= 1;
                degree[y as usize] -= 1;
                map.get_mut(&y).unwrap().remove(&x);
                people[y as usize] += people[x as usize];
                // 移除y的连接城市x后，如果y的度只剩1了 将y加入队列
                if degree[y as usize] == 1 && y != 0 {
                    temp.push_back(y);
                }
                // 计算耗油
                ans += ((people[x as usize] + seats - 1) / seats) as i64;
            }
            deque = temp;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::graph::minimum_fuel_cost::Solution;

    #[test]
    pub fn t1() {
        let roads = vec![vec![3, 1], vec![3, 2], vec![1, 0], vec![0, 4], vec![0, 5], vec![4, 6]];
        let seats = 2;
        let ans = Solution::minimum_fuel_cost(roads, seats);
        println!("{}", ans);
    }
}