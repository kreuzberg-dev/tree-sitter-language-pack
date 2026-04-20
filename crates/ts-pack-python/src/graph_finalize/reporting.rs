use serde_json::{Value, json};

pub(super) fn rel_count_map(rels: &[String], counts: &[i64]) -> Value {
    let mut obj = serde_json::Map::new();
    for (rel, count) in rels.iter().zip(counts.iter()) {
        obj.insert(rel.clone(), json!(count));
    }
    Value::Object(obj)
}

pub(super) fn file_graph_projection_rels(file_graph_projection_rels: &[&str]) -> Vec<String> {
    file_graph_projection_rels
        .iter()
        .map(|rel| (*rel).to_string())
        .collect()
}

pub(super) fn finalize_payload(
    added_manifest: i64,
    parsed_marked: i64,
    aliased: i64,
    file_call_edges: i64,
    file_graph_links: i64,
    swift_enrichment: Value,
    pagerank: i64,
    louvain: Value,
    betweenness: Value,
    isolated: Value,
) -> Value {
    json!({
        "manifest_added": added_manifest,
        "parsed_marked": parsed_marked,
        "file_path_aliased": aliased,
        "file_call_edges": file_call_edges,
        "file_graph_links": file_graph_links,
        "swift_enrichment": swift_enrichment,
        "pagerank": pagerank,
        "louvain": louvain,
        "betweenness": betweenness,
        "isolated": isolated,
    })
}
