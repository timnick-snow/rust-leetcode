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
mod build_tree2;
mod level_order_bottom;
mod sorted_array_to_bst;
mod is_balanced;
mod sorted_list_to_bst;
mod min_depth;
mod has_path_sum;
mod check_tree;
mod path_sum;
mod flatten;
mod max_path_sum;
mod sum_numbers;
mod good_nodes;
mod preorder_traversal;
mod postorder_traversal;
mod num_factored_binary_trees;

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