use std::collections::{HashMap, VecDeque};

use super::edge::Edge;
use super::error::GraphError;
use super::node::{Node, NodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<NID, ROLE, IMPL, NP> {
    nodes: Vec<Node<NID, ROLE, IMPL, NP>>,
    edges: Vec<Edge<NID>>,
}

impl<NID, ROLE, IMPL, NP> Graph<NID, ROLE, IMPL, NP>
where
    NID: Clone + Eq + std::hash::Hash,
{
    pub fn new(
        nodes: Vec<Node<NID, ROLE, IMPL, NP>>,
        edges: Vec<Edge<NID>>,
    ) -> Result<Self, GraphError<NID>> {
        let mut seen = HashMap::new();
        for (index, node) in nodes.iter().enumerate() {
            if seen.insert(node.id.clone(), index).is_some() {
                return Err(GraphError::DuplicateNode(node.id.clone()));
            }
        }
        for edge in &edges {
            if !seen.contains_key(&edge.from) {
                return Err(GraphError::UnknownNode(edge.from.clone()));
            }
            if !seen.contains_key(&edge.to) {
                return Err(GraphError::UnknownNode(edge.to.clone()));
            }
        }
        Ok(Self { nodes, edges })
    }

    pub fn nodes(&self) -> &[Node<NID, ROLE, IMPL, NP>] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge<NID>] {
        &self.edges
    }

    pub fn node(&self, id: &NodeId<NID>) -> Option<&Node<NID, ROLE, IMPL, NP>> {
        self.nodes.iter().find(|node| node.id == *id)
    }

    pub fn topological_order(&self) -> Result<Vec<NodeId<NID>>, GraphError<NID>> {
        let mut indegree = HashMap::new();
        let mut outgoing: HashMap<NodeId<NID>, Vec<NodeId<NID>>> = HashMap::new();
        for node in &self.nodes {
            indegree.insert(node.id.clone(), 0_usize);
            outgoing.insert(node.id.clone(), Vec::new());
        }
        for edge in &self.edges {
            *indegree.get_mut(&edge.to).expect("validated") += 1;
            outgoing
                .get_mut(&edge.from)
                .expect("validated")
                .push(edge.to.clone());
        }

        let mut queue = VecDeque::new();
        for node in &self.nodes {
            if indegree[&node.id] == 0 {
                queue.push_back(node.id.clone());
            }
        }

        let mut order = Vec::new();
        while let Some(id) = queue.pop_front() {
            for next in outgoing[&id].clone() {
                let degree = indegree.get_mut(&next).expect("validated");
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(next);
                }
            }
            order.push(id);
        }

        if order.len() != self.nodes.len() {
            return Err(GraphError::Cycle);
        }
        Ok(order)
    }
}
