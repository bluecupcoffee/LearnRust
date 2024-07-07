use pointers::binary_tree::*;
use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let num = rng.gen_range(-50..50);
    let root_node: BinaryTreeNode<i32> = BinaryTreeNode::create_node();
    let mut leaves: Vec<Box<BinaryTreeNode<i32>>> = vec![];
    for _i in 0..50 {
        let leaf: BinaryTreeNode<i32> = BinaryTreeNode {
            left: None,
            right: None,
            leaf_value: Some(rng.gen_range(-50..50))
        };
        leaves.push(Box::new(leaf));
    }
    println!("hello world");
    println!("leaves:\n{leaves:#?}");
    let ll = leaves.len();
    println!("leaves length: {ll}");
    let mut parents: Vec<Option<Box<BinaryTreeNode<i32>>>> = vec![];
    let mut parent: Option<Box<BinaryTreeNode<i32>>> = None;
    while leaves.len() > 0 {
        if parent.is_none() {
            let mut temp = BinaryTreeNode::<i32>::create_node();
            parent = Some(Box::new(temp));
        }
        if parent.as_mut().unwrap().left
            .is_none() {
            parent.as_mut().unwrap().left = Some(leaves.pop().expect("Couldn't pop"));
        }
        if parent.as_mut().unwrap().right
            .is_none() {
            parent.as_mut().unwrap().right = Some(leaves.pop().expect("Couldn't pop"));
            parents.push(parent);
            parent = None;
        }
    }
    println!("Parents:");
    println!("{parents:#?}");
    let plen = parents.len();
    println!("Parents length: {plen}");
    let llen = leaves.len();
    println!("leaves after populating parents:\n{llen}");
    let mut v: Vec<i32> = vec![];
    {
        let mut u = &mut v;
        let bah = BinaryTreeNode::traverse_tree(parents[24].as_mut().unwrap(), u);
    }
    println!("{:#?}", v);

}