use std::rc::Rc;
use std::cell::RefCell;

// Полное бинарное дерево
type TreeLink = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    data: String,
    left: TreeLink,
    right: TreeLink,
}

pub struct FullBinaryTree {
    root: TreeLink,
}

impl FullBinaryTree {
    pub fn new() -> Self {
        FullBinaryTree { root: None }
    }

    pub fn insert(&mut self, value: String) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(TreeNode {
                data: value,
                left: None,
                right: None,
            })));
            return;
        }

        let node_count = Self::count_nodes(&self.root);
        Self::insert_helper(&self.root, value, 0, node_count);
    }

    fn count_nodes(root: &TreeLink) -> usize {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + Self::count_nodes(&node.left) + Self::count_nodes(&node.right)
            }
        }
    }

    fn insert_helper(root: &TreeLink, value: String, index: usize, target_index: usize) -> bool {
        if let Some(node) = root {
            if index == target_index {
                node.borrow_mut().data = value;
                return true;
            }

            let mut node_mut = node.borrow_mut();

            if Self::insert_helper(&node_mut.left, value.clone(), 2 * index + 1, target_index) {
                return true;
            }

            if node_mut.left.is_none() && 2 * index + 1 == target_index {
                node_mut.left = Some(Rc::new(RefCell::new(TreeNode {
                    data: value,
                    left: None,
                    right: None,
                })));
                return true;
            }

            if Self::insert_helper(&node_mut.right, value.clone(), 2 * index + 2, target_index) {
                return true;
            }

            if node_mut.right.is_none() && 2 * index + 2 == target_index {
                node_mut.right = Some(Rc::new(RefCell::new(TreeNode {
                    data: value,
                    left: None,
                    right: None,
                })));
                return true;
            }
        }
        false
    }

    pub fn search(&self, value: &str) -> bool {
        Self::search_helper(&self.root, value)
    }

    fn search_helper(root: &TreeLink, value: &str) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow();
                node.data == value
                    || Self::search_helper(&node.left, value)
                    || Self::search_helper(&node.right, value)
            }
        }
    }

    pub fn is_full(&self) -> bool {
        Self::is_full_helper(&self.root)
    }

    fn is_full_helper(root: &TreeLink) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => true,
                    (Some(_), Some(_)) => {
                        Self::is_full_helper(&node.left) && Self::is_full_helper(&node.right)
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn print_inorder(&self) {
        if self.root.is_none() {
            println!("Дерево пустое");
            return;
        }

        println!("Структура дерева:");
        Self::print_tree_helper(&self.root, String::new(), false);

        print!("Симметричный обход (inorder): ");
        Self::print_inorder_linear(&self.root);
        println!();
    }

    fn print_tree_helper(root: &TreeLink, prefix: String, is_left: bool) {
        if let Some(node) = root {
            let node = node.borrow();
            println!("{}{}{}", prefix, if is_left { "├──" } else { "└──" }, node.data);

            if node.left.is_some() || node.right.is_some() {
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                if node.left.is_some() {
                    Self::print_tree_helper(&node.left, new_prefix.clone(), true);
                }
                if node.right.is_some() {
                    Self::print_tree_helper(&node.right, new_prefix, false);
                }
            }
        }
    }

    fn print_inorder_linear(root: &TreeLink) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::print_inorder_linear(&node.left);
            print!("{} ", node.data);
            Self::print_inorder_linear(&node.right);
        }
    }

    pub fn collect_inorder(&self) -> Vec<String> {
        let mut result = Vec::new();
        Self::collect_inorder_helper(&self.root, &mut result);
        result
    }

    fn collect_inorder_helper(root: &TreeLink, result: &mut Vec<String>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::collect_inorder_helper(&node.left, result);
            result.push(node.data.clone());
            Self::collect_inorder_helper(&node.right, result);
        }
    }
}