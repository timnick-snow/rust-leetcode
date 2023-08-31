#![allow(dead_code)]

use crate::tree::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
struct BSTIterator {
    cur: Option<Rc<RefCell<TreeNode>>>,
    deque: Vec<Rc<RefCell<TreeNode>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator {
            cur: root,
            deque: vec![],
        }
    }

    fn next(&mut self) -> i32 {
        while self.cur.is_some() {
            self.deque.push(Rc::clone(self.cur.as_ref().unwrap()));
            let temp = self.cur.as_ref().unwrap().borrow().left.as_ref()
                .map_or(None, |x| Some(Rc::clone(&x)));
            self.cur = temp;
        }
        let node = self.deque.pop().unwrap();
        let x = node.borrow().val;
        self.cur = node.borrow().right.as_ref()
            .map_or(None, |x| Some(Rc::clone(&x)));
        x
    }

    fn has_next(&self) -> bool {
        self.cur.is_some() || !self.deque.is_empty()
    }
}