use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 递归插入节点（内部方法）
    fn insert_node(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 插入左子树
                if let Some(ref mut left) = self.left {
                    left.insert_node(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                // 插入右子树
                if let Some(ref mut right) = self.right {
                    right.insert_node(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 相等时不处理（二叉搜索树通常不允许重复，或根据需求处理）
            Ordering::Equal => {}
        }
    }

    // 递归搜索节点（内部方法）
    fn search_node(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map(|n| n.search_node(value)).unwrap_or(false),
            Ordering::Greater => self.right.as_ref().map(|n| n.search_node(value)).unwrap_or(false),
            Ordering::Equal => true,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 公共插入接口
    fn insert(&mut self, value: T) {
        if let Some(root_node) = self.root.as_mut() {
            root_node.insert_node(value);
        } else {
            // 根节点为空时创建新根
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 公共搜索接口
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map(|n| n.search_node(value)).unwrap_or(false)
    }
}

// 测试用例保持不变（已包含在问题描述中）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}