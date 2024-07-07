pub mod binary_tree {
    use std::{cell::RefCell, rc::Rc};
    use std::fmt::Display;
    use std::error::Error;
    use std::process;

    #[derive(Debug, Clone)]
    pub struct BinaryTreeNode<T> {
        pub left: Option<Box<BinaryTreeNode<T>>>,
        pub right: Option<Box<BinaryTreeNode<T>>>,
        pub leaf_value: Option<T>
    }

    impl<T> BinaryTreeNode<T>
    where
        T: Clone + Copy + Display + PartialEq
    {
        pub fn create_node() -> BinaryTreeNode<T> {
            BinaryTreeNode{
                left: None,
                right: None,
                leaf_value: None
            }
        }

        pub fn add_node<'a>(&'a mut self, new_node: BinaryTreeNode<T>) -> Result<(), Box<dyn Error>> {
            if self.left.is_none() {
                self.left = Some(Box::new(new_node));
                Ok(())
            } else if self.right.is_none() {
                self.right = Some(Box::new(new_node));
                Ok(())
            } else {
                let err: Box<dyn Error> = "Both sides (L/R) are occupied. Cannot add child node.".into();
                Err(err)
            }
        }
        pub fn traverse_tree<'a>(root_node: &'a mut Box<BinaryTreeNode<T>>,
                             visited_leaves: &'a mut Vec<T>) -> () {
            if root_node.leaf_value.is_some() {
                visited_leaves.push(*root_node.leaf_value.as_mut().unwrap());
            } else {
                if root_node.left.is_some() {
                    Self::traverse_tree(root_node.left.as_mut().unwrap(), visited_leaves);
                }
                if root_node.right.is_some() {
                    Self::traverse_tree(root_node.right.as_mut().unwrap(), visited_leaves);
                }
            }
        }

        pub fn delete_child(root_node: &mut Box<BinaryTreeNode<T>>,
                            del_left: bool,
                            del_right: bool) {
            if del_left && root_node.left.is_some() {
                root_node.left = None;
            }
            if del_right && root_node.right.is_some() {
                root_node.right = None;
            }
        }
    }

    //type BinaryTreeNodeRef<'a, T> = Rc<RefCell<BinaryTreeNode<T>>>;
}

#[cfg(test)]
pub mod tests {
    use super::binary_tree::*;
    #[test]
    pub fn btn_create() {
        let new_node: BinaryTreeNode<i32> = BinaryTreeNode::create_node();
        assert!(new_node.left.is_none());
        assert!(new_node.right.is_none());
        assert!(new_node.leaf_value.is_none());
    }
    #[test]
    pub fn add_node_test() {
        let mut root_node: BinaryTreeNode<i32> = BinaryTreeNode::create_node();
        let mut child_node: BinaryTreeNode<i32> = BinaryTreeNode::create_node();
        child_node.leaf_value = Some(5);
        let _res = BinaryTreeNode::add_node(&mut root_node, child_node);
        assert_eq!(5, root_node.left.as_mut().unwrap().leaf_value.unwrap());
        assert!(root_node.left.is_some());
        assert!(root_node.left.as_mut().unwrap().leaf_value.is_some());
    }


}