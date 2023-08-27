#![allow(dead_code)]
/*
146. LRU 缓存
中等
2.8K
相关企业
请你设计并实现一个满足  LRU (最近最少使用) 缓存 约束的数据结构。
实现 LRUCache 类：
LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存
int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
void put(int key, int value)
    如果关键字 key 已经存在，则变更其数据值 value ；
    如果不存在，则向缓存中插入该组 key-value 。
    如果插入操作导致关键字数量超过 capacity ，则应该 逐出 最久未使用的关键字。

函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。


示例：

输入
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
输出
[null, null, null, 1, null, -1, null, -1, 3, 4]

解释
LRUCache lRUCache = new LRUCache(2);
lRUCache.put(1, 1); // 缓存是 {1=1}
lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
lRUCache.get(1);    // 返回 1
lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
lRUCache.get(2);    // 返回 -1 (未找到)
lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
lRUCache.get(1);    // 返回 -1 (未找到)
lRUCache.get(3);    // 返回 3
lRUCache.get(4);    // 返回 4


提示：

1 <= capacity <= 3000
0 <= key <= 10000
0 <= value <= 105
最多调用 2 * 105 次 get 和 put
 */

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    val: i32,
    pre: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            pre: None,
            next: None,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            key: 0,
            val: 0,
            pre: None,
            next: None,
        }
    }
}

struct LRUCache {
    data: HashMap<i32, Rc<RefCell<Node>>>,
    capacity: i32,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        // 使用伪头尾节点 方便后续处理
        let head = Some(Rc::new(RefCell::new(Node::default())));
        let tail = Some(Rc::new(RefCell::new(Node::default())));
        head.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(tail.as_ref().unwrap()));
        tail.as_ref().unwrap().borrow_mut().pre = Some(Rc::clone(head.as_ref().unwrap()));
        LRUCache {
            data: HashMap::with_capacity(capacity as usize),
            capacity,
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.data.get(&key) {
            None => -1,
            Some(n) => {
                let value = n.borrow().val;
                self.move_to_head(Rc::clone(n));
                value
            }
        }
    }


    fn put(&mut self, key: i32, value: i32) {
        let mut flag: Option<Rc<RefCell<Node>>> = None;
        if let Some(node) = self.data.get_mut(&key) {
            node.borrow_mut().val = value;
            flag = Some(Rc::clone(node));
            // self.move_to_head(Rc::clone(node));
        } else {
            // 节点不存在 创建一个节点
            let new_node = Rc::new(RefCell::new(Node::new(key, value)));
            self.data.insert(key, Rc::clone(&new_node));
            // 添加到头部
            self.add_to_head(new_node);
            // 容量超标?
            if self.data.len() > self.capacity as usize {
                let last = self.remove_tail();
                self.data.remove(&last.borrow().key);
            }
        }
        if flag.is_some() {
            self.move_to_head(flag.unwrap());
        }
    }

    fn move_to_head(&mut self, node: Rc<RefCell<Node>>) {
        Self::remove_node(Rc::clone(&node));
        self.add_to_head(node);
    }

    fn remove_node(node: Rc<RefCell<Node>>) {
        let pre = node.borrow_mut().pre.take();
        let next = node.borrow_mut().next.take();
        pre.as_ref().unwrap().borrow_mut().next = next.clone();
        next.as_ref().unwrap().borrow_mut().pre = pre;
    }

    fn add_to_head(&mut self, node: Rc<RefCell<Node>>) -> &mut Self {
        let next_link = self.head.as_ref().unwrap().borrow_mut().next.take();
        // node 与head 关联
        node.borrow_mut().pre = self.head.clone();
        self.head.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&node));
        // node 与 next_link 关联
        next_link.as_ref().unwrap().borrow_mut().pre = Some(Rc::clone(&node));
        node.borrow_mut().next = next_link;
        self
    }

    fn remove_tail(&mut self) -> Rc<RefCell<Node>> {
        let last = Rc::clone(self.tail.as_ref().unwrap().borrow().pre.as_ref().unwrap());
        Self::remove_node(Rc::clone(&last));
        last
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn t1() {
        println!("hello");
    }
}