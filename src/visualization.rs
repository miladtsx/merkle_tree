use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn visualize(graph: &Graph<String, ()>, root_index: Option<NodeIndex>, output_file: &str) {
    if let Some(dot) = generate_dot(graph, root_index) {

        std::fs::create_dir_all("out").expect("Failed to create output directory");
        
        let mut child = Command::new("dot")
            .arg("-Tpng") // Generate a PNG
            .arg("-o") // Specify output file
            .arg(format!("out/{}", output_file))
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn dot command");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(dot.as_bytes())
                .expect("Failed to write to dot command");
        }

        let status = child.wait().expect("Failed to wait on dot command");

        if status.success() {
            println!("Merkle tree visualization saved to '{}'", output_file);
        } else {
            eprintln!("dot command failed with status: {:?}", status);
        }
    } else {
        eprintln!("Failed to generate DOT representation.");
    }
}

fn generate_dot(graph: &Graph<String, ()>, root_index: Option<NodeIndex>) -> Option<String> {
    root_index?;
    Some(format!(
        "{:?}",
        Dot::with_config(graph, &[Config::EdgeNoLabel])
    ))
}
