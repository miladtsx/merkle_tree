use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    let data = vec![
        "data1".to_string(),
        "data2".to_string(),
        "data3".to_string(),
        "data4".to_string(),
    ];
    let merkle_tree = MerkleTree::new(data.clone());

    merkle_tree.visualize("visualization.png");

    println!("Merkle Root: {:?}", merkle_tree.get_root());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root() {
        let data = vec![
            "data1".to_string(),
            "data2".to_string(),
            "data3".to_string(),
            "data4".to_string(),
        ];
        let merkle_tree = MerkleTree::new(data.clone());
        let root = merkle_tree.get_root();
        assert!(root.is_some(), "Root should not be None");
    }
}
