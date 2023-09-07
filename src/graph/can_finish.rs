#![allow(dead_code)]
/*
207. 课程表
提示
中等
1.7K
相关企业
你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。

在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，其中 prerequisites[i] = [ai, bi] ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。

例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。



示例 1：

输入：numCourses = 2, prerequisites = [[1,0]]
输出：true
解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
示例 2：

输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
输出：false
解释：总共有 2 门课程。学习课程 1 之前，你需要先完成课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。


提示：

1 <= numCourses <= 2000
0 <= prerequisites.length <= 5000
prerequisites[i].length == 2
0 <= ai, bi < numCourses
prerequisites[i] 中的所有课程对 互不相同
 */
use std::collections::VecDeque;

struct Solution;

/*
有循环依赖时 无法完成

拓扑排序

广度优先  队列实现
有向图: u -> v, 表示学习了u课程才能学习v课程
入度: v课程的前置课程数量

首先将所有入度为0的课程加入队列，这些课程是可以直接学习的
将这些课程指向的课程的入度-1，遇到课程入度为0时加入队列

edges[i] 表示课程i的指向节点列表
indeg[i] 表示课程i的入度
 */
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut edges = vec![vec![]; n];
        let mut indeg = vec![0; n];
        // 计算边和入度
        for x in prerequisites.into_iter() {
            // b -> a
            let (a, b) = (x[0], x[1]);
            edges[b as usize].push(a);
            indeg[a as usize] += 1;
        }
        // 队列初始化为
        let mut deque = VecDeque::new();
        for i in 0..n {
            if indeg[i] == 0 {
                deque.push_back(i);
            }
        }
        // 已学习课程数
        let mut visited = 0;
        while !deque.is_empty() {
            // 学习课程x
            let x = deque.pop_front().unwrap();
            // 所有x指向的课程入度-1
            for &y in edges[x].iter() {
                let y = y as usize;
                indeg[y] -= 1;
                if indeg[y] == 0 {
                    deque.push_back(y);
                }
            }
            visited += 1;
        }
        visited == num_courses
    }
}

#[cfg(test)]
mod test {
    use crate::graph::can_finish::Solution;

    #[test]
    pub fn t1() {
        let nc = 2;
        let prerequisites = vec![vec![1, 0]];
        let finish = Solution::can_finish(nc, prerequisites);
        println!("{}", finish);
    }
}