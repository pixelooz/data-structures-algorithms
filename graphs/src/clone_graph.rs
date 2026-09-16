use crate::Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let node = node?;
        let mut visited = HashMap::new();
        Some(Self::clone_graph_dfs(node.clone(), &mut visited))
    }

    fn clone_graph_dfs(
        node: Rc<RefCell<Node>>,
        visited: &mut HashMap<*mut Node, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        let key_ptr = node.as_ptr();

        if let Some(cloned) = visited.get(&key_ptr) {
            return cloned.clone();
        }
        let clone = Rc::new(RefCell::new(Node {
            val: node.borrow().val,
            neighbors: Vec::new(),
        }));
        visited.insert(node.as_ptr(), clone.clone());
        for neighbor in &node.borrow().neighbors {
            let new_neighbor = Self::clone_graph_dfs(neighbor.clone(), visited);
            clone.borrow_mut().neighbors.push(new_neighbor);
        }
        clone
    }
}
