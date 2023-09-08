#![allow(dead_code)]
/*
208. 实现 Trie (前缀树)
中等
1.5K
相关企业
Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补完和拼写检查。

请你实现 Trie 类：

Trie() 初始化前缀树对象。
void insert(String word) 向前缀树中插入字符串 word 。
boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回 false 。
boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true ；否则，返回 false 。


示例：

输入
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
输出
[null, null, true, false, true, null, true]

解释
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // 返回 True
trie.search("app");     // 返回 False
trie.startsWith("app"); // 返回 True
trie.insert("app");
trie.search("app");     // 返回 True


提示：

1 <= word.length, prefix.length <= 2000
word 和 prefix 仅由小写英文字母组成
insert、search 和 startsWith 调用次数 总计 不超过 3 * 10^4 次
 */


#[derive(Default)]
struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.into_bytes() {
            let idx = (c - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert(Box::new(Trie::default()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.search_node_by_prefix(&word)
            .map_or(false, |x| x.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.search_node_by_prefix(&prefix).is_some()
    }

    fn search_node_by_prefix(&self, prefix: &str) -> Option<&Trie> {
        let mut node = self;
        for &c in prefix.as_bytes() {
            let idx = (c - b'a') as usize;
            match &node.children[idx] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }
        Some(node)
    }
}
