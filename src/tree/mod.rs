mod inorder_traversal;
mod generate_trees;
mod num_trees;
mod valid_bst;
mod recover_tree;
mod is_same_tree;
mod is_symmetric;
mod level_order;
mod zigzag_level_order;
mod max_depth;
mod build_tree;

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}