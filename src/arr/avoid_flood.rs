#![allow(dead_code)]
/*
1488. 避免洪水泛滥
中等
相关标签
相关企业
提示
你的国家有无数个湖泊，所有湖泊一开始都是空的。当第 n 个湖泊下雨前是空的，那么它就会装满水。如果第 n 个湖泊下雨前是 满的 ，这个湖泊会发生 洪水 。你的目标是避免任意一个湖泊发生洪水。

给你一个整数数组 rains ，其中：

rains[i] > 0 表示第 i 天时，第 rains[i] 个湖泊会下雨。
rains[i] == 0 表示第 i 天没有湖泊会下雨，你可以选择 一个 湖泊并 抽干 这个湖泊的水。
请返回一个数组 ans ，满足：

ans.length == rains.length
如果 rains[i] > 0 ，那么ans[i] == -1 。
如果 rains[i] == 0 ，ans[i] 是你第 i 天选择抽干的湖泊。
如果有多种可行解，请返回它们中的 任意一个 。如果没办法阻止洪水，请返回一个 空的数组 。

请注意，如果你选择抽干一个装满水的湖泊，它会变成一个空的湖泊。但如果你选择抽干一个空的湖泊，那么将无事发生。



示例 1：

输入：rains = [1,2,3,4]
输出：[-1,-1,-1,-1]
解释：第一天后，装满水的湖泊包括 [1]
第二天后，装满水的湖泊包括 [1,2]
第三天后，装满水的湖泊包括 [1,2,3]
第四天后，装满水的湖泊包括 [1,2,3,4]
没有哪一天你可以抽干任何湖泊的水，也没有湖泊会发生洪水。
示例 2：

输入：rains = [1,2,0,0,2,1]
输出：[-1,-1,2,1,-1,-1]
解释：第一天后，装满水的湖泊包括 [1]
第二天后，装满水的湖泊包括 [1,2]
第三天后，我们抽干湖泊 2 。所以剩下装满水的湖泊包括 [1]
第四天后，我们抽干湖泊 1 。所以暂时没有装满水的湖泊了。
第五天后，装满水的湖泊包括 [2]。
第六天后，装满水的湖泊包括 [1,2]。
可以看出，这个方案下不会有洪水发生。同时， [-1,-1,1,2,-1,-1] 也是另一个可行的没有洪水的方案。
示例 3：

输入：rains = [1,2,0,1,2]
输出：[]
解释：第二天后，装满水的湖泊包括 [1,2]。我们可以在第三天抽干一个湖泊的水。
但第三天后，湖泊 1 和 2 都会再次下雨，所以不管我们第三天抽干哪个湖泊的水，另一个湖泊都会发生洪水。


提示：

1 <= rains.length <= 105
0 <= rains[i] <= 109
 */
use std::collections::HashMap;

/*

用结果数组ans当一个链表存储，存储那些不下雨的天数，这些天里我们可以抽干湖泊避免洪水

定义指针 head tail 表示链表的头索引和结束索引，数组存储的值为该节点的后继节点的索引位置
尾节点没有后继节点，规定 ans[tail] = -2
初始化 head = tail = -1

利用map来存储已经下雨的湖泊,值是该湖泊下雨的天数。 key: rains[i], value: i

遍历rains数组，当遍历到索引i处时
1. rains[i] = 0, 我们更新链表的 head, tail
2. rains[i] > 0, 我们让 ans[i] = -1，同时判断这个湖泊之前是否已经下过雨。
    如果没有下过，保存到set
    如果已经下过，更新链表值:
    否则，找一个可行的时间t提前将rains[i]湖泊的水抽干，ans[t] = rains[i]。然后更新链表
    如果没有没有可用的时间让我们抽干湖泊，无法避免洪水，返回空数组。
 */
struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut ans = vec![0; n];
        let (mut head, mut tail) = (-2, -2);
        // key: 湖泊rains[i] ， value: 下雨的天数 i
        let mut map = HashMap::new();
        // 处理每一天
        for i in 0..n {
            if rains[i] == 0 {
                // 不下雨，存贮这些天数
                if head < 0 {
                    head = i as i32;
                    tail = i as i32;
                    ans[i] = -2;
                } else {
                    ans[tail as usize] = i as i32;
                    ans[i] = -2;
                    tail = i as i32;
                }
            } else {
                // 下雨
                if let Some(day) = map.insert(rains[i], i as i32) {
                    // 该湖泊之前下过雨已满，需要找一个天数抽干
                    let mut t = head;
                    let mut pre: Option<i32> = None;
                    while t >= 0 && t < day {
                        pre = Some(t);
                        t = ans[t as usize];
                    }
                    if t < 0 {
                        // 未找到可行的天数t
                        return vec![];
                    }
                    match pre {
                        None => {
                            head = ans[t as usize];
                        }
                        Some(q) => {
                            ans[q as usize] = ans[t as usize];
                            if tail == t {
                                tail = q;
                            }
                        }
                    }
                    ans[t as usize] = rains[i];
                }
                ans[i] = -1;
            }
            println!("{} ==> {:?}", i, ans);
        }
        println!("head= {}, tail = {}", head, tail);
        // 处理链表尾节点
        if head >= 0 && ans[tail as usize] < 0 {
            ans[tail as usize] = 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::avoid_flood::Solution;

    #[test]
    pub fn t1() {
        let rains = vec![1, 0, 1, 0, 2, 0, 2];
        let ans = Solution::avoid_flood(rains);
        println!("{:?}", ans);
    }

    #[test]
    pub fn t2() {
        let rains = vec![69, 0, 0, 0, 69];
        let ans = Solution::avoid_flood(rains);
        println!("{:?}", ans);
    }
}