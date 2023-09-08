#![allow(dead_code)]
/*
210. 课程表 II
提示
中等
810
相关企业
现在你总共有 numCourses 门课需要选，记为 0 到 numCourses - 1。给你一个数组 prerequisites ，其中 prerequisites[i] = [ai, bi] ，表示在选修课程 ai 前 必须 先选修 bi 。

例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示：[0,1] 。
返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要返回 任意一种 就可以了。如果不可能完成所有课程，返回 一个空数组 。



示例 1：

输入：numCourses = 2, prerequisites = [[1,0]]
输出：[0,1]
解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
示例 2：

输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
输出：[0,2,1,3]
解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
示例 3：

输入：numCourses = 1, prerequisites = []
输出：[0]
 */
use std::collections::VecDeque;

struct Solution;

/*

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
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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
        let mut ans = Vec::new();
        while !deque.is_empty() {
            // 学习课程x
            let x = deque.pop_front().unwrap();
            ans.push(x as i32);
            // 所有x指向的课程入度-1
            for &y in edges[x].iter() {
                let y = y as usize;
                indeg[y] -= 1;
                if indeg[y] == 0 {
                    deque.push_back(y);
                }
            }
        }
        if ans.len() == n {
            ans
        }else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::graph::find_order::Solution;

    #[test]
    pub fn t1() {
        let nc = 2;
        let prerequisites = vec![vec![1, 0]];
        let order = Solution::find_order(nc, prerequisites);
        assert_eq!(order, vec![0, 1])
    }
}