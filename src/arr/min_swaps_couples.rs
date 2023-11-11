#![allow(dead_code)]
/*
765. 情侣牵手
困难
相关标签
相关企业
提示
n 对情侣坐在连续排列的 2n 个座位上，想要牵到对方的手。

人和座位由一个整数数组 row 表示，其中 row[i] 是坐在第 i 个座位上的人的 ID。情侣们按顺序编号，第一对是 (0, 1)，第二对是 (2, 3)，以此类推，最后一对是 (2n-2, 2n-1)。

返回 最少交换座位的次数，以便每对情侣可以并肩坐在一起。 每次交换可选择任意两人，让他们站起来交换座位。



示例 1:

输入: row = [0,2,1,3]
输出: 1
解释: 只需要交换row[1]和row[2]的位置即可。
示例 2:

输入: row = [3,2,0,1]
输出: 0
解释: 无需交换座位，所有的情侣都已经可以手牵手了。


提示:

2n == row.length
2 <= n <= 30
n 是偶数
0 <= row[i] < 2n
row 中所有元素均无重复
 */
struct Solution;

/*
pos: 情侣所在的位置

1. 情侣只能在 2i,2i+1 位置上牵手
2. 交换是等价的。比如 0,2,1,3 中， 0的右边是2，要将0和1坐在一起，交换2和1与交换0和3是等价的效果
 */
impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let n = row.len() >> 1;
        let mut pos = vec![0; n << 1];
        row.iter().enumerate().for_each(|(seat, &people)| {
            pos[people as usize] = seat;
        });

        let mut ans = 0;
        for i in 0..n {
            let p1 = row[i * 2] as usize;
            let p2 = if p1 & 1 == 0 { p1 + 1 } else { p1 - 1 };
            if pos[p2] != 2 * i + 1 {
                // 把情侣p2交换到2*i+1的位置
                let swap_people = row[i * 2 + 1];
                row.swap(2 * i + 1, pos[p2]);
                pos.swap(p2, swap_people as usize);
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::min_swaps_couples::Solution;

    #[test]
    pub fn t1() {
        let row = vec![3, 2, 0, 1];
        let ans = Solution::min_swaps_couples(row);
        assert_eq!(ans, 0);
    }
}