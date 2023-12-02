#![allow(dead_code)]
/*
1094. 拼车
中等
相关标签
相关企业
提示
车上最初有 capacity 个空座位。车 只能 向一个方向行驶（也就是说，不允许掉头或改变方向）

给定整数 capacity 和一个数组 trips ,  trip[i] = [numPassengersi, fromi, toi] 表示第 i 次旅行有 numPassengersi 乘客，接他们和放他们的位置分别是 fromi 和 toi 。这些位置是从汽车的初始位置向东的公里数。

当且仅当你可以在所有给定的行程中接送所有乘客时，返回 true，否则请返回 false。



示例 1：

输入：trips = [[2,1,5],[3,3,7]], capacity = 4
输出：false
示例 2：

输入：trips = [[2,1,5],[3,3,7]], capacity = 5
输出：true


提示：

1 <= trips.length <= 1000
trips[i].length == 3
1 <= numPassengersi <= 100
0 <= fromi < toi <= 1000
1 <= capacity <= 105
 */
struct Solution;
/*
排序
1 +9
2 +4
4 -9
4 +3
4 +5
5 -4
5 +3
6 -3
6 +5
6 -5
8 -5
9 -3

 */
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let n = trips.len();
        let mut from: Vec<(i32, i32)> = trips.iter()
            .map(|x| (x[1], x[0]))
            .collect();
        let mut to: Vec<(i32, i32)> = trips.iter()
            .map(|x| (x[2], x[0]))
            .collect();
        from.sort_unstable_by_key(|x| x.0);
        to.sort_unstable_by_key(|x| x.0);

        let mut x = 0;
        let (mut i, mut j) = (0, 0);
        while i < n {
            if from[i].0 < to[j].0 {
                // 上车
                x += from[i].1;
                i += 1;
                if x > capacity {
                    return false;
                }
            } else {
                // 下车
                x -= to[j].1;
                j += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::arr::car_pooling::Solution;

    #[test]
    pub fn t1() {
        let arr = vec![vec![3, 5, 9], vec![4, 2, 5], vec![3, 4, 6], vec![9, 1, 4], vec![5, 6, 8], vec![5, 4, 6]];
        let cap = 14;
        let ans = Solution::car_pooling(arr, cap);
        assert!(ans)
    }
}