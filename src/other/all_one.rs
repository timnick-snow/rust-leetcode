#![allow(dead_code)]
/*
432. 全 O(1) 的数据结构
已解答
困难
相关标签
相关企业
请你设计一个用于存储字符串计数的数据结构，并能够返回计数最小和最大的字符串。

实现 AllOne 类：

AllOne() 初始化数据结构的对象。
inc(String key) 字符串 key 的计数增加 1 。如果数据结构中尚不存在 key ，那么插入计数为 1 的 key 。
dec(String key) 字符串 key 的计数减少 1 。如果 key 的计数在减少后为 0 ，那么需要将这个 key 从数据结构中删除。测试用例保证：在减少计数前，key 存在于数据结构中。
getMaxKey() 返回任意一个计数最大的字符串。如果没有元素存在，返回一个空字符串 "" 。
getMinKey() 返回任意一个计数最小的字符串。如果没有元素存在，返回一个空字符串 "" 。
注意：每个函数都应当满足 O(1) 平均时间复杂度。



示例：

输入
["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
[[], ["hello"], ["hello"], [], [], ["leet"], [], []]
输出
[null, null, null, "hello", "hello", null, "hello", "leet"]

解释
AllOne allOne = new AllOne();
allOne.inc("hello");
allOne.inc("hello");
allOne.getMaxKey(); // 返回 "hello"
allOne.getMinKey(); // 返回 "hello"
allOne.inc("leet");
allOne.getMaxKey(); // 返回 "hello"
allOne.getMinKey(); // 返回 "leet"


提示：

1 <= key.length <= 10
key 由小写英文字母组成
测试用例保证：在每次调用 dec 时，数据结构中总存在 key
最多调用 inc、dec、getMaxKey 和 getMinKey 方法 5 * 104 次
 */

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct AllOne {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
    map: HashMap<String, Rc<RefCell<Node>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        // 哨兵头尾节点
        let head = Rc::new(RefCell::new(Node::new("".to_string(), -1)));
        let tail = Rc::new(RefCell::new(Node::new("".to_string(), -1)));

        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        AllOne {
            head,
            tail,
            map: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        if !self.map.contains_key(&key) {
            // 新key
            if self.head.borrow().next.as_ref().unwrap().borrow().count == 1 {
                // node已存在 将key添加到set中
                let node = Rc::clone(self.head.borrow().next.as_ref().unwrap());
                node.borrow_mut().keys.insert(key.clone());
                self.map.insert(key, Rc::clone(&node));
            } else {
                // 新node
                let node = Rc::new(RefCell::new(Node::new(key.clone(), 1)));
                self.head.borrow_mut().insert(Rc::clone(&node));
                self.map.insert(key, Rc::clone(&node));
            }
        } else {
            // 当前key已存在于cur节点中
            let cur = Rc::clone(self.map.get(&key).unwrap());
            let next = Rc::clone(cur.borrow().next.as_ref().unwrap());

            if next.borrow().count == cur.borrow().count + 1 {
                // 将当前key转移到next节点即可
                next.borrow_mut().keys.insert(key.clone());
                // 更新map
                self.map.insert(key.clone(), Rc::clone(&next));
            } else {
                // cur节点后面需要新增一个node节点 当前key存放在node中
                let node = Rc::new(RefCell::new(Node::new(key.clone(), cur.borrow().count + 1)));
                cur.borrow_mut().insert(Rc::clone(&node));
                // 更新map
                self.map.insert(key.clone(), Rc::clone(&node));
            }
            // cur节点移除当前key
            cur.borrow_mut().keys.remove(&key);
            if cur.borrow_mut().keys.is_empty() {
                cur.borrow_mut().remove();
            }
        }
    }

    fn dec(&mut self, key: String) {
        // 当前key一定存在于cur节点中
        let cur = Rc::clone(self.map.get(&key).unwrap());
        let prev = Rc::clone(cur.borrow().prev.as_ref().unwrap());

        if prev.borrow().count == cur.borrow().count - 1 {
            // 将当前key转移到prev节点即可
            prev.borrow_mut().keys.insert(key.clone());
            // 更新map
            self.map.insert(key.clone(), Rc::clone(&prev));
        } else {
            if cur.borrow().count == 1 {
                // 归零 map清除这个key
                self.map.remove(&key);
            } else {
                // prev节点后面需要新增一个node节点 将当前key存放在node中
                let node = Rc::new(RefCell::new(Node::new(key.clone(), cur.borrow().count - 1)));
                prev.borrow_mut().insert(Rc::clone(&node));
                // 更新map
                self.map.insert(key.clone(), Rc::clone(&node));
            }
        }
        // cur节点移除当前key
        cur.borrow_mut().keys.remove(&key);
        if cur.borrow().keys.is_empty() {
            cur.borrow_mut().remove();
        }
    }

    fn get_max_key(&self) -> String {
        // 最后一个有效节点  tail.prev
        self.tail.as_ref().borrow().prev.as_ref()
            .unwrap().borrow().keys.iter().next()
            .unwrap().clone()
    }

    fn get_min_key(&self) -> String {
        // 第一个有效节点 head.next
        self.head.as_ref().borrow().next.as_ref()
            .unwrap().borrow().keys.iter().next()
            .unwrap().clone()
    }
}

#[derive(Debug)]
struct Node {
    keys: HashSet<String>,
    count: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: String, count: i32) -> Self {
        let mut keys = HashSet::new();
        keys.insert(key);
        Node {
            keys,
            count,
            prev: None,
            next: None,
        }
    }

    /// 当前节点后插入node
    fn insert(&mut self, node: Rc<RefCell<Node>>) {
        let this_next = self.next.take().unwrap();
        let this = Rc::clone(this_next.borrow().prev.as_ref().unwrap());

        self.next = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::clone(&this));
        node.borrow_mut().next = Some(Rc::clone(&this_next));
        this_next.borrow_mut().prev = Some(Rc::clone(&node));
    }

    /// 从链表中移除当前节点
    fn remove(&mut self) {
        let this_prev = self.prev.take().unwrap();
        let this_next = self.next.take().unwrap();

        this_prev.borrow_mut().next = Some(Rc::clone(&this_next));
        this_next.borrow_mut().prev = Some(Rc::clone(&this_prev));
    }
}

#[cfg(test)]
mod test {
    use crate::other::all_one::AllOne;

    #[test]
    pub fn t1() {
        let mut all_one = AllOne::new();
        all_one.inc("a".to_string());
        all_one.inc("b".to_string());
        all_one.inc("b".to_string());
        all_one.inc("c".to_string());
        all_one.inc("c".to_string());
        all_one.inc("c".to_string());

        all_one.dec("b".to_string());
        all_one.dec("b".to_string());
        assert_eq!(all_one.get_min_key(), "a");

        all_one.dec("a".to_string());
        assert_eq!(all_one.get_max_key(), "c");
        assert_eq!(all_one.get_min_key(), "c");
    }
}