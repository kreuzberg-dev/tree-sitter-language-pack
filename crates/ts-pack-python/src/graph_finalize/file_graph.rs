use neo4rs::{query, Graph};
use std::sync::Arc;
use ts_pack_index::graph_schema;

use super::{one_i64, FILE_GRAPH_SOURCE_RELS};

pub(super) async fn build_file_calls_from_symbol_graph(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let cypher = format!(
        "MATCH (src:{file_label} {{project_id:$pid}})-[:{contains_rel}]->(caller:Node {{project_id:$pid}})
         MATCH (caller)-[:{calls_rel}|{calls_inferred_rel}]->(callee:Node {{project_id:$pid}})
         MATCH (dst:{file_label} {{project_id:$pid}})-[:{contains_rel}]->(callee)
         WHERE src <> dst
         MERGE (src)-[r:{calls_file_rel}]->(dst)
         SET r.project_id = $pid,
             r.last_seen_run = $run_id
         RETURN count(DISTINCT r) AS updated",
        file_label = graph_schema::NODE_LABEL_FILE,
        contains_rel = graph_schema::REL_CONTAINS,
        calls_rel = graph_schema::REL_CALLS,
        calls_inferred_rel = graph_schema::REL_CALLS_INFERRED,
        calls_file_rel = graph_schema::REL_CALLS_FILE,
    );
    one_i64(
        graph,
        query(&cypher)
            .param("pid", project_id.to_string())
            .param("run_id", run_id.to_string()),
        "updated",
    )
    .await
}

pub(super) async fn build_file_graph_links(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut total = 0i64;

    for rel in FILE_GRAPH_SOURCE_RELS {
        let cypher = format!(
            "MATCH (src:{file_label} {{project_id:$pid}})-[:{rel}]->(dst:{file_label} {{project_id:$pid}})
             WHERE src <> dst
             MERGE (src)-[r:{file_graph_link_rel}]->(dst)
             SET r.project_id = $pid,
                 r.last_seen_run = $run_id
             RETURN count(DISTINCT r) AS updated",
            file_label = graph_schema::NODE_LABEL_FILE,
            file_graph_link_rel = graph_schema::REL_FILE_GRAPH_LINK,
        );
        total += one_i64(
            graph,
            query(&cypher)
                .param("pid", project_id.to_string())
                .param("run_id", run_id.to_string()),
            "updated",
        )
        .await?;
    }

    total += one_i64(
        graph,
        query(&format!(
            "MATCH (src:{file_label} {{project_id:$pid}})-[:{calls_api_route_rel}]->(:{api_route_label} {{project_id:$pid}})-[:{handled_by_rel}]->(dst:{file_label} {{project_id:$pid}})
             WHERE src <> dst
             MERGE (src)-[r:{file_graph_link_rel}]->(dst)
             SET r.project_id = $pid,
                 r.last_seen_run = $run_id
             RETURN count(DISTINCT r) AS updated",
            file_label = graph_schema::NODE_LABEL_FILE,
            api_route_label = graph_schema::NODE_LABEL_API_ROUTE,
            calls_api_route_rel = graph_schema::REL_CALLS_API_ROUTE,
            handled_by_rel = graph_schema::REL_HANDLED_BY,
            file_graph_link_rel = graph_schema::REL_FILE_GRAPH_LINK,
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string()),
        "updated",
    )
    .await?;

    Ok(total)
}
