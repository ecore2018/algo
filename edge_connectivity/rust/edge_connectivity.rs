use graph::Graph;
use maximum_flow::maximum_flow;

pub fn edge_connectivity(g: &Graph) -> u32 {
    assert!(!g.directed, "This edge connectivity algorithm only supports undirected graphs");

    let maximum_flows = (1..g.vertex_count).map(|i| maximum_flow(g, 0, i));
    let connectivity = maximum_flows.min();

    connectivity.unwrap_or(0)
}
