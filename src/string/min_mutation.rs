#![allow(dead_code)]
/*
433. 最小基因变化
中等
相关标签
相关企业
基因序列可以表示为一条由 8 个字符组成的字符串，其中每个字符都是 'A'、'C'、'G' 和 'T' 之一。

假设我们需要调查从基因序列 start 变为 end 所发生的基因变化。一次基因变化就意味着这个基因序列中的一个字符发生了变化。

例如，"AACCGGTT" --> "AACCGGTA" 就是一次基因变化。
另有一个基因库 bank 记录了所有有效的基因变化，只有基因库中的基因才是有效的基因序列。（变化后的基因必须位于基因库 bank 中）

给你两个基因序列 start 和 end ，以及一个基因库 bank ，请你找出并返回能够使 start 变化为 end 所需的最少变化次数。如果无法完成此基因变化，返回 -1 。

注意：起始基因序列 start 默认是有效的，但是它并不一定会出现在基因库中。



示例 1：

输入：start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
输出：1
示例 2：

输入：start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
输出：2
示例 3：

输入：start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
输出：3


提示：

start.length == 8
end.length == 8
0 <= bank.length <= 10
bank[i].length == 8
start、end 和 bank[i] 仅由字符 ['A', 'C', 'G', 'T'] 组成
 */
use std::collections::{HashSet, VecDeque};

/*
位运算
将字符串转变成数字
 */
struct Solution;

const MASKS: [u32; 8] = [0xffff_fff0, 0xffff_ff0f, 0xffff_f0ff, 0xffff_0fff,
    0xfff0_ffff, 0xff0f_ffff, 0xf0ff_ffff, 0x0fff_ffff];

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        // 1. 预处理字符为数字
        let start = to_u32(start_gene);
        let end = to_u32(end_gene);
        if start == end {
            return 0;
        }
        let bank: HashSet<u32> = bank.into_iter().map(|x| to_u32(x)).collect();

        if !bank.contains(&end) {
            return -1;
        }

        // BFS搜索
        let mut deque = VecDeque::new();
        let mut visited = HashSet::new();
        deque.push_front(start);
        visited.insert(start);
        let mut cnt = 0;
        while !deque.is_empty() {
            cnt += 1;
            let mut temp = VecDeque::new();
            while let Some(cur) = deque.pop_front() {
                for &gene in bank.iter() {
                    if !visited.contains(&gene) && check_mutation(cur, gene) {
                        if gene == end {
                            return cnt;
                        }
                        temp.push_front(gene);
                        visited.insert(gene);
                    }
                }
            }
            deque = temp;
        }
        -1
    }
}

#[inline]
fn check_mutation(cur: u32, target: u32) -> bool {
    MASKS.iter().any(|&mask| mask & (cur ^ target) == 0)
}

fn to_u32(s: String) -> u32 {
    let mut ans = 0;
    s.chars().for_each(|c| {
        let x = match c {
            'A' => 1,
            'C' => 2,
            'G' => 4,
            'T' => 8,
            _ => unreachable!()
        };
        ans = (ans << 4) | x;
    });
    ans
}


#[cfg(test)]
mod test {
    use crate::string::min_mutation::Solution;

    #[test]
    pub fn t1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string(), "AACCGCTA".to_string(), "AAACGGTA".to_string()];

        let ans = Solution::min_mutation(start_gene, end_gene, bank);
        assert_eq!(ans, 2);
    }
}