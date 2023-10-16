#![allow(dead_code)]
/*
307. 区域和检索 - 数组可修改
中等
相关标签
相关企业
给你一个数组 nums ，请你完成两类查询。

其中一类查询要求 更新 数组 nums 下标对应的值
另一类查询要求返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 ，其中 left <= right
实现 NumArray 类：

NumArray(int[] nums) 用整数数组 nums 初始化对象
void update(int index, int val) 将 nums[index] 的值 更新 为 val
int sumRange(int left, int right) 返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 （即，nums[left] + nums[left + 1], ..., nums[right]）


示例 1：

输入：
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
输出：
[null, 9, null, 8]

解释：
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1,2,5]
numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8


提示：

1 <= nums.length <= 3 * 104
-100 <= nums[i] <= 100
0 <= index < nums.length
-100 <= val <= 100
0 <= left <= right < nums.length
调用 update 和 sumRange 方法次数不大于 3 * 104
 */

/*
树状数组
将原索引+1以方便后续处理

lowbit(x) = x & (-x)
lowbit(x) = x & (~x + 1)

简单来说，树状数组就是利用lowbit的性质，把n个节点串起来，隐式地构造一棵树。
每个节点x的父亲是x + lowbit(x)，每个节点维护其子节点的和。

更重要的一点，也是树状数组算法的核心，即处于当前x节点左边且不属于x子树中最大的节点是x - lowbit(x)，

新增修改节点：修改节点x，会影响当前节点以及所有的父节点 x + lowbit(x)，最终影响到根节点(n)
查询前缀和：节点x的前缀和，包括自身的值以及其左边不属于x子树的最大节点 x-lowbit(x)，最终递归到0
求区间和：使用前缀和相减即可
 */
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn add(&mut self, mut i: usize, delta: i32) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += self.lowbit(i);
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans += self.tree[i];
            i -= self.lowbit(i);
        }
        ans
    }

    fn lowbit(&self, x: usize) -> usize {
        x & (!x + 1)
    }
}

struct NumArray {
    nums: Vec<i32>,
    fenwick_tree: FenwickTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut fenwick_tree = FenwickTree {
            tree: vec![0; nums.len() + 1],
        };
        for i in 0..nums.len() {
            fenwick_tree.add(i + 1, nums[i]);
        }
        Self { nums, fenwick_tree }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.fenwick_tree
            .add(index as usize + 1, val - self.nums[index as usize]);
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.fenwick_tree.query(right as usize + 1) - self.fenwick_tree.query(left as usize)
    }
}

/*
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */