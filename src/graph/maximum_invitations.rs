#![allow(dead_code)]
/*
2127. 参加会议的最多员工数
困难
相关标签
相关企业
提示
一个公司准备组织一场会议，邀请名单上有 n 位员工。公司准备了一张 圆形 的桌子，可以坐下 任意数目 的员工。

员工编号为 0 到 n - 1 。每位员工都有一位 喜欢 的员工，每位员工 当且仅当 他被安排在喜欢员工的旁边，他才会参加会议。每位员工喜欢的员工 不会 是他自己。

给你一个下标从 0 开始的整数数组 favorite ，其中 favorite[i] 表示第 i 位员工喜欢的员工。请你返回参加会议的 最多员工数目 。



示例 1：



输入：favorite = [2,2,1,2]
输出：3
解释：
上图展示了公司邀请员工 0，1 和 2 参加会议以及他们在圆桌上的座位。
没办法邀请所有员工参与会议，因为员工 2 没办法同时坐在 0，1 和 3 员工的旁边。
注意，公司也可以邀请员工 1，2 和 3 参加会议。
所以最多参加会议的员工数目为 3 。
示例 2：

输入：favorite = [1,2,0]
输出：3
解释：
每个员工都至少是另一个员工喜欢的员工。所以公司邀请他们所有人参加会议的前提是所有人都参加了会议。
座位安排同图 1 所示：
- 员工 0 坐在员工 2 和 1 之间。
- 员工 1 坐在员工 0 和 2 之间。
- 员工 2 坐在员工 1 和 0 之间。
参与会议的最多员工数目为 3 。
示例 3：



输入：favorite = [3,0,1,4,1]
输出：4
解释：
上图展示了公司可以邀请员工 0，1，3 和 4 参加会议以及他们在圆桌上的座位。
员工 2 无法参加，因为他喜欢的员工 0 旁边的座位已经被占领了。
所以公司只能不邀请员工 2 。
参加会议的最多员工数目为 4 。


提示：

n == favorite.length
2 <= n <= 105
0 <= favorite[i] <= n - 1
favorite[i] != i
 */
use std::collections::{HashSet, VecDeque};

struct Solution;
/*
员工之间构成有向图  a -> b 表示 a喜欢b

每个员工都有喜欢的员工，所以这些员工至少形成一个环。
注意可能形成多个连通分量：

2 -> 0 <--> 1 <- 3  ||  4 <--> 5

   ↙--------|
  6 -> 7 -> 8 <- 9

分析可知以下结论：
1. 每个连通分量中都有且只有一个环
2. 成环是通过相互喜欢的设为ring2, 3个或以上构成喜欢环的设为ring3
3. 最终的安排中如果存在ring3，那么这个ring3中只有成环的这些人能够被安排。
   连通分量中的其他人无法安排，即任何喜欢环中的人都无法加入到座位中，因为那个人旁边已经被占满了，分别是他喜欢和喜欢他的人。
   其它连通分量也无法与这个ring3共存。因为没有可以挤进去的空间。
4. 多个ring2连通分量可以共存。
5. ring2分量中，两个相互喜欢的设为center，那么这个分量中喜欢center的也可以加入到座位中，每个center都可以有一条被喜欢的长链。

为了求ring2的被喜欢长链，我们先处理得到被喜欢的map

最终结果，取最长的ring3，MAX(ring3)
取所有ring2的和  SUM(ring2)
再取两者的较大值
 */
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        // 喜欢索引i的人的集合
        let mut liked_map = vec![vec![]; n];
        for i in 0..n {
            // i -> favor[i]
            liked_map[favorite[i] as usize].push(i);
        }

        // 处理得到所有连通分量
        let mut visited = HashSet::new();
        // ring3 保存每个分量中形成环的任意一人
        let mut ring3 = vec![];
        // ring2 保存每个分量中相互喜欢的任意一人
        let mut ring2 = vec![];
        'OUTER:
        for i in 0..n {
            let mut cur = i;
            if visited.contains(&cur) {
                continue;
            }
            // 每一个连通分量
            let mut segment = HashSet::new();
            while segment.insert(cur) {
                cur = favorite[cur] as usize;
                if visited.contains(&cur) {
                    visited.extend(segment.into_iter());
                    continue 'OUTER;
                }
            }
            // cur处形成环
            if favorite[favorite[cur] as usize] as usize == cur {
                ring2.push(cur);
            } else {
                ring3.push(cur);
            }
            // 标记为已访问
            visited.extend(segment.into_iter());
        }

        // 处理得到 max_ring3
        let max_ring3 = ring3.into_iter().map(|x| {
            let mut set = HashSet::new();
            let mut cur = x;
            while set.insert(cur) {
                cur = favorite[cur] as usize;
            }
            set.len() as i32
        }).max().unwrap_or(0);

        // 处理得到 sum_ring2
        let sum_ring2 = ring2.into_iter().map(|x| {
            let center1 = x;
            let center2 = favorite[x] as usize;
            bfs(&liked_map, center1, center2) + bfs(&liked_map, center2, center1)
        }).sum();

        max_ring3.max(sum_ring2)
    }
}

fn bfs(liked_map: &Vec<Vec<usize>>, start: usize, exclude: usize) -> i32 {
    let mut deque: VecDeque<usize> = liked_map[start].iter()
        .filter(|&&x| x != exclude)
        .cloned()
        .collect();
    let mut cnt = 1;
    while !deque.is_empty() {
        let mut temp = VecDeque::new();
        while let Some(p) = deque.pop_front() {
            temp.extend(liked_map[p].iter().cloned());
        }
        deque = temp;
        cnt += 1;
    }
    cnt
}