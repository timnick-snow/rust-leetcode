#![allow(dead_code)]
/*
2003. 每棵子树内缺失的最小基因值
困难
相关标签
相关企业
提示
有一棵根节点为 0 的 家族树 ，总共包含 n 个节点，节点编号为 0 到 n - 1 。给你一个下标从 0 开始的整数数组 parents ，其中 parents[i] 是节点 i 的父节点。由于节点 0 是 根 ，所以 parents[0] == -1 。

总共有 105 个基因值，每个基因值都用 闭区间 [1, 105] 中的一个整数表示。给你一个下标从 0 开始的整数数组 nums ，其中 nums[i] 是节点 i 的基因值，且基因值 互不相同 。

请你返回一个数组 ans ，长度为 n ，其中 ans[i] 是以节点 i 为根的子树内 缺失 的 最小 基因值。

节点 x 为根的 子树 包含节点 x 和它所有的 后代 节点。



示例 1：



输入：parents = [-1,0,0,2], nums = [1,2,3,4]
输出：[5,1,1,1]
解释：每个子树答案计算结果如下：
- 0：子树包含节点 [0,1,2,3] ，基因值分别为 [1,2,3,4] 。5 是缺失的最小基因值。
- 1：子树只包含节点 1 ，基因值为 2 。1 是缺失的最小基因值。
- 2：子树包含节点 [2,3] ，基因值分别为 [3,4] 。1 是缺失的最小基因值。
- 3：子树只包含节点 3 ，基因值为 4 。1是缺失的最小基因值。
示例 2：



输入：parents = [-1,0,1,0,3,3], nums = [5,4,6,2,1,3]
输出：[7,1,1,4,2,1]
解释：每个子树答案计算结果如下：
- 0：子树内包含节点 [0,1,2,3,4,5] ，基因值分别为 [5,4,6,2,1,3] 。7 是缺失的最小基因值。
- 1：子树内包含节点 [1,2] ，基因值分别为 [4,6] 。 1 是缺失的最小基因值。
- 2：子树内只包含节点 2 ，基因值为 6 。1 是缺失的最小基因值。
- 3：子树内包含节点 [3,4,5] ，基因值分别为 [2,1,3] 。4 是缺失的最小基因值。
- 4：子树内只包含节点 4 ，基因值为 1 。2 是缺失的最小基因值。
- 5：子树内只包含节点 5 ，基因值为 3 。1 是缺失的最小基因值。
示例 3：

输入：parents = [-1,2,3,0,2,4,1], nums = [2,3,4,5,6,7,8]
输出：[1,1,1,1,1,1,1]
解释：所有子树都缺失基因值 1 。


提示：

n == parents.length == nums.length
2 <= n <= 105
对于 i != 0 ，满足 0 <= parents[i] <= n - 1
parents[0] == -1
parents 表示一棵合法的树。
1 <= nums[i] <= 105
nums[i] 互不相同。
 */
use std::collections::HashSet;

struct Solution;

/*
考虑基因值1，如果基因值1不存在，那么所有节点都缺失基因值1
如果基因值1存在，那么节点可以分为两种：
1. 该节点的子树不包含基因值1
2. 该节点的子树包含基因值1，当且仅当该节点是基因值1节点的祖先节点

于是，我们可以找到基因值1的节点，然后自底向上遍历求出这些节点的缺失值。至于其它节点缺失值一定是1

要求出缺失值，必须记录直接点的所有基因值。记genSet为基因值集合，进行DFS将所有子节点的基因值存储。
    由于我们向上遍历，node节点遍历完成后，向上找到node节点的父节点parent。假设node节点是parent的左孩子，那么node子树
    其实已经遍历过了，不需要重复进行搜索，我们只需要搜索parent的右孩子子树并记录其基因值即可。
    为了避免DFS搜索时重复搜索，使用visited数组来记录已经访问过的节点。


为了方便DFS，需要将parents数组转化成children数组
 */
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len() as usize;
        let mut children: Vec<Vec<i32>> = vec![vec![]; n];
        parents.iter().enumerate().for_each(|(i, &parent)| {
            if parent != -1 {
                children[parent as usize].push(i as i32);
            }
        });

        // 找到基因值为1的节点
        let mut node = -1;
        for (i, &gen_val) in nums.iter().enumerate() {
            if gen_val == 1 {
                node = i as i32;
                break;
            }
        }

        // 缺失值集合 ans 初始值为1
        let mut ans = vec![1; n];

        let mut gen_set: HashSet<i32> = HashSet::new();
        let mut missing_value = 2;
        let mut visited: Vec<bool> = vec![false; n];
        // 从基因值为1的点向上遍历，直至到达根节点，根节点的父节点为-1
        while node != -1 {
            // DFS搜索并记录所有的基因值
            dfs(node, &nums, &children, &mut visited, &mut gen_set);
            // 找到缺失的基因值
            while gen_set.contains(&missing_value) {
                missing_value += 1;
            }
            ans[node as usize] = missing_value;
            // 更新node值 向上遍历
            node = parents[node as usize];
        }

        ans
    }
}

fn dfs(node: i32, nums: &Vec<i32>, children: &Vec<Vec<i32>>, visited: &mut Vec<bool>, gen_set: &mut HashSet<i32>) {
    let node_key = node as usize;
    if visited[node_key] {
        return;
    }
    visited[node_key] = true;
    gen_set.insert(nums[node_key]);
    for x in &children[node_key] {
        dfs(*x, nums, children, visited, gen_set);
    }
}