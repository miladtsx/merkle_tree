use crate::visualization;
use petgraph::graph::{Graph, NodeIndex};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct MerkleTree {
    root: Option<String>,
    leaves: Vec<String>,
    graph: Graph<String, ()>,
    root_index: Option<NodeIndex>,
}

impl MerkleTree {
    pub fn new(data: Vec<String>) -> Self {
        let mut graph = Graph::<String, ()>::new();
        let leaves: Vec<_> = data
            .iter()
            .map(|item| {
                let hash = hash(item);
                graph.add_node(hash.clone());
                hash
            })
            .collect();

        let root_index = build_merkle_graph(&mut graph, &leaves);

        MerkleTree {
            root: root_index.map(|idx| graph[idx].clone()),
            leaves,
            graph,
            root_index,
        }
    }

    pub fn get_root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    pub fn visualize(&self, output_file: &str) {
        visualization::visualize(&self.graph, self.root_index, output_file);
    }

}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn build_merkle_graph(graph: &mut Graph<String, ()>, leaves: &[String]) -> Option<NodeIndex> {
    if leaves.is_empty() {
        return None;
    }
    let mut current_level: Vec<NodeIndex> = leaves
        .iter()
        .map(|leaf| {
            graph
                .node_indices()
                .find(|&idx| graph[idx] == *leaf)
                .unwrap()
        })
        .collect();

    while current_level.len() > 1 {
        let mut next_level = vec![];

        for pair in current_level.chunks(2) {
            let combined_hash = match pair {
                [a, b] => {
                    let concat = format!("{}{}", graph[*a], graph[*b]);
                    hash(&concat)
                }
                [a] => graph[*a].clone(),
                _ => unreachable!(),
            };

            let parent_index = graph.add_node(combined_hash);
            for &child in pair {
                graph.add_edge(parent_index, child, ());
            }
            next_level.push(parent_index);
        }
        current_level = next_level;
    }

    current_level.first().copied()
}
