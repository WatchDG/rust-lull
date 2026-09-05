use std::collections::HashMap;
use std::hash::Hash;

use super::error::ExecuteError;
use super::graph::Graph;
use super::node::NodeId;
use super::registry::Registry;

pub type NodeOutputs<NID, M> = HashMap<NodeId<NID>, Vec<M>>;

pub(crate) fn execute<NID, IMPL, NP, M, E>(
    graph: &Graph<NID, IMPL, NP>,
    registry: &Registry<IMPL, M, NP, E>,
    seeds: &NodeOutputs<NID, M>,
    order: &[NodeId<NID>],
) -> Result<NodeOutputs<NID, M>, ExecuteError<NID, IMPL, E>>
where
    NID: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
    M: Clone,
{
    let mut outputs: NodeOutputs<NID, M> = HashMap::new();

    for id in order {
        let node = graph.node(id).expect("order contains graph nodes");
        let mut inputs = Vec::new();
        if let Some(seed) = seeds.get(id) {
            inputs.extend(seed.iter().cloned());
        }
        for edge in graph.edges() {
            if edge.to == *id {
                if let Some(predecessor) = outputs.get(&edge.from) {
                    inputs.extend(predecessor.iter().cloned());
                }
            }
        }

        let factory = registry
            .get(node.r#type, &node.implementation)
            .ok_or_else(|| ExecuteError::UnknownFactory {
                r#type: node.r#type,
                implementation: node.implementation.clone(),
            })?;
        let mut handler = factory
            .create(&node.params)
            .map_err(|source| ExecuteError::Node {
                id: id.clone(),
                source,
            })?;
        let output = handler
            .process(&inputs)
            .map_err(|source| ExecuteError::Node {
                id: id.clone(),
                source,
            })?;
        outputs.insert(id.clone(), output);
    }

    Ok(outputs)
}
