use std::collections::HashMap;
use std::sync::Arc;

use neo4rs::{BoltType, Graph, Query};
use serde_json::Value;
use tokio::time::{Duration, sleep};

use crate::{
    ApiRouteCallRow, ApiRouteHandlerRow, CargoCrateFileRow, CargoCrateRow, CargoDependencyEdgeRow,
    CargoWorkspaceCrateRow, CargoWorkspaceRow, CloneCanonRow, CloneGroupRow, CloneMemberRow, DbEdgeRow, DbModelEdgeRow,
    ExportAliasEdgeRow, ExportSymbolEdgeRow, ExternalApiEdgeRow, ExternalApiNode, FileCloneCanonRow, FileCloneGroupRow,
    FileCloneMemberRow, FileEdgeRow, FileImportEdgeRow, FileNode, ImplicitImportSymbolEdgeRow, ImportNode,
    ImportSymbolEdgeRow, InferredCallRow, LaunchEdgeRow, PythonInferredCallRow, RelRow, ResourceBackingRow,
    ResourceTargetEdgeRow, ResourceUsageRow, RustImplTraitEdgeRow, RustImplTypeEdgeRow, SymbolCallRow, SymbolNode,
    XcodeSchemeFileRow, XcodeSchemeRow, XcodeSchemeTargetRow, XcodeTargetFileRow, XcodeTargetRow,
    XcodeWorkspaceProjectRow, XcodeWorkspaceRow,
};

fn json_to_bolt(v: Value) -> BoltType {
    match v {
        Value::String(s) => BoltType::from(s),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                BoltType::from(i)
            } else if let Some(f) = n.as_f64() {
                BoltType::from(f)
            } else {
                BoltType::from(0i64)
            }
        }
        Value::Bool(b) => BoltType::from(b),
        Value::Null => BoltType::Null(neo4rs::BoltNull),
        Value::Array(arr) => BoltType::from(arr.into_iter().map(json_to_bolt).collect::<Vec<_>>()),
        Value::Object(map) => {
            let mut bolt_map = HashMap::new();
            for (k, val) in map {
                bolt_map.insert(k, json_to_bolt(val));
            }
            BoltType::from(bolt_map)
        }
    }
}

fn rows_to_bolt<T, F: Fn(&T) -> Value>(rows: &[T], f: F) -> BoltType {
    BoltType::from(rows.iter().map(|r| json_to_bolt(f(r))).collect::<Vec<_>>())
}

fn write_retry_attempts() -> usize {
    std::env::var("TS_PACK_NEO4J_WRITE_RETRY_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(4)
}

fn write_retry_base_ms() -> u64 {
    std::env::var("TS_PACK_NEO4J_WRITE_RETRY_BASE_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(200)
}

fn is_retryable_neo4j_write_error(err: &neo4rs::Error) -> bool {
    let msg = err.to_string().to_lowercase();
    msg.contains("deadlock")
        || msg.contains("deadlockdetected")
        || msg.contains("transienterror")
        || msg.contains("lockclient")
        || msg.contains("cannot acquire")
        || msg.contains("can't acquire")
}

fn build_project_rel_prune_cypher(left_pattern: &str, rel_type: &str, right_pattern: &str) -> String {
    format!(
        "MATCH ({left_pattern})-[r:{rel_type}]->({right_pattern}) \
         WHERE r.last_seen_run IS NULL OR r.last_seen_run <> $run_id \
         DELETE r"
    )
}

fn build_project_node_prune_cypher(label: &str) -> String {
    format!(
        "MATCH (n:{label} {{project_id: $pid}}) \
         WHERE n.last_seen_run IS NULL OR n.last_seen_run <> $run_id \
         DETACH DELETE n"
    )
}

fn build_file_to_file_edge_write_cypher(rel_name: &str) -> String {
    format!(
        "UNWIND $batch AS item \
         MATCH (a:File {{project_id: item.pid, filepath: item.src}}) \
         MATCH (b:File {{project_id: item.pid, filepath: item.tgt}}) \
         MERGE (a)-[r:{rel_name}]->(b) \
         SET r.last_seen_run = $run_id"
    )
}

pub(crate) async fn run_query_logged(graph: &Arc<Graph>, q: Query, label: &str) -> neo4rs::Result<()> {
    let attempts = write_retry_attempts();
    let base_ms = write_retry_base_ms();
    for attempt in 1..=attempts {
        match graph.run(q.clone()).await {
            Ok(()) => return Ok(()),
            Err(err) => {
                if is_retryable_neo4j_write_error(&err) && attempt < attempts {
                    let delay_ms = base_ms.saturating_mul(attempt as u64);
                    eprintln!(
                        "[ts-pack-index] neo4j write retry ({label}) attempt {attempt}/{attempts} after {delay_ms}ms: {err}"
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    continue;
                }
                eprintln!("[ts-pack-index] neo4j write failed ({label}): {err}");
                return Err(err);
            }
        }
    }
    Ok(())
}

pub(crate) async fn write_file_nodes(graph: &Arc<Graph>, batch: &[FileNode], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:File, \
                      n.stable_id  = item.stable_id, \
                       n.name       = item.name, \
                      n.filepath   = item.filepath, \
                      n.project_id = item.project_id, \
                      n.is_test    = item.is_test, \
                      n.last_seen_run = $run_id \
         ON MATCH SET  n.stable_id  = item.stable_id, \
                      n.name       = item.name, \
                      n.is_test    = item.is_test, \
                      n.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_file_nodes").await
}

pub(crate) async fn write_symbol_nodes(
    graph: &Arc<Graph>,
    batch: &[SymbolNode],
    label: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let cypher = format!(
        "UNWIND $batch AS item \
         MERGE (n:Node {{id: item.id}}) \
         ON CREATE SET n:{label}, \
                       n.stable_id   = item.stable_id, \
                       n.name        = item.name, \
                       n.kind        = item.kind, \
                       n.qualified_name = item.qualified_name, \
                       n.project_id  = item.project_id, \
                       n.filepath    = item.filepath, \
                       n.start_line  = item.start_line, \
                       n.end_line    = item.end_line, \
                       n.start_byte  = item.start_byte, \
                       n.end_byte    = item.end_byte, \
                       n.signature   = item.signature, \
                       n.visibility  = item.visibility, \
                       n.is_exported = item.is_exported, \
                       n.doc_comment = item.doc_comment, \
                       n.last_seen_run = $run_id \
         ON MATCH SET  n.stable_id   = item.stable_id, \
                       n.start_line  = item.start_line, \
                       n.end_line    = item.end_line, \
                       n.qualified_name = item.qualified_name, \
                       n.signature   = item.signature, \
                       n.visibility  = item.visibility, \
                       n.is_exported = item.is_exported, \
                       n.doc_comment = item.doc_comment, \
                       n.last_seen_run = $run_id \
         FOREACH (_ IN CASE WHEN item.kind = 'Method' THEN [1] ELSE [] END | SET n:Method)"
    );
    let q = Query::new(cypher)
        .param("batch", bolt)
        .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_symbol_nodes").await
}

pub(crate) async fn write_import_nodes(graph: &Arc<Graph>, batch: &[ImportNode], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:Import, \
                      n.stable_id   = item.stable_id, \
                      n.stable_file_id = item.stable_file_id, \
                      n.name        = item.name, \
                      n.source      = item.source, \
                      n.is_wildcard = item.is_wildcard, \
                      n.project_id  = item.project_id, \
                      n.filepath    = item.filepath, \
                      n.last_seen_run = $run_id \
         ON MATCH SET  n.stable_id   = item.stable_id, \
                      n.stable_file_id = item.stable_file_id, \
                      n.name        = item.name, \
                      n.source      = item.source, \
                      n.is_wildcard = item.is_wildcard, \
                      n.filepath    = item.filepath, \
                      n.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_import_nodes").await
}

pub(crate) async fn write_relationships(graph: &Arc<Graph>, batch: &[RelRow], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (p:Node {id: item.p}) \
         MATCH (c:Node {id: item.c}) \
         MERGE (p)-[r:CONTAINS]->(c) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_relationships").await
}

pub(crate) async fn write_calls(graph: &Arc<Graph>, batch: &[SymbolCallRow], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[r:CALLS]->(callee) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_calls").await
}

pub(crate) async fn write_inferred_calls(
    graph: &Arc<Graph>,
    batch: &[InferredCallRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND callee.qualified_name IS NOT NULL \
           AND callee.qualified_name STARTS WITH item.recv + '.' \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[r:CALLS_INFERRED]->(callee) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_inferred_calls").await
}

pub(crate) async fn write_python_inferred_calls(
    graph: &Arc<Graph>,
    batch: &[PythonInferredCallRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee, filepath: item.callee_fp}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[r:CALLS_INFERRED]->(callee) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_python_inferred_calls").await
}

pub(crate) async fn prune_stale_core_graph_data(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    for (label, query_label) in [
        ("CONTAINS", "prune_stale_contains"),
        ("CALLS", "prune_stale_calls"),
        ("CALLS_INFERRED", "prune_stale_calls_inferred"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":Node {project_id: $pid}",
            label,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, query_label).await?;
    }

    for (label, query_label) in [
        ("Import", "prune_stale_import_nodes"),
        ("File", "prune_stale_file_nodes"),
        ("Node", "prune_stale_generic_nodes"),
    ] {
        let q = Query::new(build_project_node_prune_cypher(label))
            .param("pid", project_id.to_string())
            .param("run_id", run_id.to_string());
        run_query_logged(graph, q, query_label).await?;
    }
    Ok(())
}

pub(crate) async fn write_db_edges(graph: &Arc<Graph>, batch: &[DbEdgeRow], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:File {id: item.tgt}) \
         MERGE (a)-[r:CALLS_DB]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_db_edges").await
}

pub(crate) async fn write_db_model_edges(
    graph: &Arc<Graph>,
    batch: &[DbModelEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (m:Model {id: item.pid + ':model:' + item.model}) \
         SET m.project_id = item.pid, m.name = item.model, m.last_seen_run = $run_id \
         WITH item, m \
         MATCH (a:File {id: item.src}) \
         MERGE (a)-[r:CALLS_DB_MODEL]->(m) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_db_model_edges").await
}

pub(crate) async fn prune_stale_db_data(graph: &Arc<Graph>, project_id: &str, run_id: &str) -> neo4rs::Result<()> {
    let delete_db_edges = Query::new(build_project_rel_prune_cypher(
        ":File {project_id: $pid}",
        "CALLS_DB",
        ":File {project_id: $pid}",
    ))
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_db_edges, "prune_stale_calls_db").await?;

    let delete_db_model_edges = Query::new(build_project_rel_prune_cypher(
        ":File {project_id: $pid}",
        "CALLS_DB_MODEL",
        ":Model {project_id: $pid}",
    ))
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_db_model_edges, "prune_stale_calls_db_model").await?;

    let delete_models = Query::new(build_project_node_prune_cypher("Model"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_models, "prune_stale_models").await
}

pub(crate) async fn write_external_api_nodes(
    graph: &Arc<Graph>,
    batch: &[ExternalApiNode],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (e:ExternalAPI {id: item.id}) \
         SET e.project_id = item.pid, \
             e.url = item.url, \
             e.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_external_api_nodes").await
}

pub(crate) async fn write_clone_groups(
    graph: &Arc<Graph>,
    batch: &[CloneGroupRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (g:CloneGroup {id: item.id}) \
         SET g.project_id = item.project_id, \
             g.size = item.size, \
             g.method = item.method, \
             g.score_min = item.score_min, \
             g.score_max = item.score_max, \
             g.score_avg = item.score_avg, \
             g.created_at = timestamp(), \
             g.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_clone_groups").await
}

pub(crate) async fn write_clone_members(
    graph: &Arc<Graph>,
    batch: &[CloneMemberRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (s)-[r:MEMBER_OF_CLONE_GROUP]->(g) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_clone_members").await
}

pub(crate) async fn write_clone_canon(graph: &Arc<Graph>, batch: &[CloneCanonRow], run_id: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (g)-[r:HAS_CANONICAL]->(s) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_clone_canon").await
}

pub(crate) async fn write_file_clone_groups(
    graph: &Arc<Graph>,
    batch: &[FileCloneGroupRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (g:FileCloneGroup {id: item.id}) \
         SET g.project_id = item.project_id, \
             g.size = item.size, \
             g.method = item.method, \
             g.score_min = item.score_min, \
             g.score_max = item.score_max, \
             g.score_avg = item.score_avg, \
             g.created_at = timestamp(), \
             g.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_file_clone_groups").await
}

pub(crate) async fn write_file_clone_members(
    graph: &Arc<Graph>,
    batch: &[FileCloneMemberRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (f)-[r:MEMBER_OF_FILE_CLONE_GROUP]->(g) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_file_clone_members").await
}

pub(crate) async fn write_file_clone_canon(
    graph: &Arc<Graph>,
    batch: &[FileCloneCanonRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (g)-[r:HAS_CANONICAL]->(f) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_file_clone_canon").await
}

pub(crate) async fn prune_stale_clone_data(graph: &Arc<Graph>, project_id: &str, run_id: &str) -> neo4rs::Result<()> {
    for (label, query_label) in [
        ("CloneGroup", "prune_stale_clone_groups"),
        ("FileCloneGroup", "prune_stale_file_clone_groups"),
    ] {
        let q = Query::new(build_project_node_prune_cypher(label))
            .param("pid", project_id.to_string())
            .param("run_id", run_id.to_string());
        run_query_logged(graph, q, query_label).await?;
    }
    Ok(())
}

pub(crate) async fn write_external_api_edges(
    graph: &Arc<Graph>,
    batch: &[ExternalApiEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:ExternalAPI {id: item.tgt}) \
         MERGE (a)-[r:CALLS_API_EXTERNAL]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_external_api_edges").await
}

pub(crate) async fn prune_stale_external_api_data(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let delete_edges = Query::new(build_project_rel_prune_cypher(
        ":File {project_id: $pid}",
        "CALLS_API_EXTERNAL",
        ":ExternalAPI {project_id: $pid}",
    ))
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_edges, "prune_stale_external_api_edges").await?;

    let delete_nodes = Query::new(build_project_node_prune_cypher("ExternalAPI"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_nodes, "prune_stale_external_api_nodes").await
}

pub(crate) async fn write_file_import_edges(
    graph: &Arc<Graph>,
    batch: &[FileImportEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.pid, filepath: item.src}) \
         MATCH (b:File {project_id: item.pid, filepath: item.tgt}) \
         MERGE (a)-[r:IMPORTS]->(b) \
         SET r.project_id = item.pid, \
             r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_file_import_edges").await
}

pub(crate) async fn prune_stale_file_import_edges(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let q = Query::new(build_project_rel_prune_cypher(
        ":File {project_id: $pid}",
        "IMPORTS",
        ":File {project_id: $pid}",
    ))
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "prune_stale_file_import_edges").await
}

pub(crate) async fn write_file_edges(
    graph: &Arc<Graph>,
    batch: &[FileEdgeRow],
    rel_name: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(build_file_to_file_edge_write_cypher(rel_name))
        .param("batch", bolt)
        .param("run_id", run_id.to_string());
    run_query_logged(graph, q, &format!("write_{rel_name}")).await
}

pub(crate) async fn prune_stale_file_edge_family(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
    rel_type: &str,
    label: &str,
) -> neo4rs::Result<()> {
    let q = Query::new(build_project_rel_prune_cypher(
        ":File {project_id: $pid}",
        rel_type,
        ":File {project_id: $pid}",
    ))
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, label).await
}

pub(crate) async fn write_api_route_calls(
    graph: &Arc<Graph>,
    batch: &[ApiRouteCallRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.project_id, filepath: item.src}) \
         MERGE (r:ApiRoute {project_id: item.project_id, path: item.path, method: item.method}) \
         ON CREATE SET r.name = item.method + ' ' + item.path, r.filepath = item.path \
         SET r.filepath = item.path, \
             r.last_seen_run = $run_id \
         MERGE (a)-[rel:CALLS_API_ROUTE]->(r) \
         SET rel.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_api_route_calls").await
}

pub(crate) async fn write_api_route_handlers(
    graph: &Arc<Graph>,
    batch: &[ApiRouteHandlerRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (r:ApiRoute {project_id: item.project_id, path: item.path, method: item.method}) \
         MATCH (b:File {project_id: item.project_id, filepath: item.tgt}) \
         SET r.last_seen_run = $run_id \
         MERGE (r)-[rel:HANDLED_BY]->(b) \
         SET rel.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_api_route_handlers").await
}

pub(crate) async fn prune_stale_api_route_data(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let delete_call_edges = Query::new(
        "MATCH (:File {project_id: $pid})-[r:CALLS_API_ROUTE]->(:ApiRoute {project_id: $pid}) \
         WHERE r.last_seen_run IS NULL OR r.last_seen_run <> $run_id \
         DELETE r"
            .to_string(),
    )
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_call_edges, "prune_stale_api_route_calls").await?;

    let delete_handler_edges = Query::new(
        "MATCH (:ApiRoute {project_id: $pid})-[r:HANDLED_BY]->(:File {project_id: $pid}) \
         WHERE r.last_seen_run IS NULL OR r.last_seen_run <> $run_id \
         DELETE r"
            .to_string(),
    )
    .param("pid", project_id.to_string())
    .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_handler_edges, "prune_stale_api_route_handlers").await?;

    let delete_nodes = Query::new(build_project_node_prune_cypher("ApiRoute"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_nodes, "prune_stale_api_routes").await
}

pub(crate) async fn write_resource_usage_edges(
    graph: &Arc<Graph>,
    batch: &[ResourceUsageRow],
    rel_name: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(format!(
        "UNWIND $batch AS item \
         MATCH (a:File {{project_id: item.project_id, filepath: item.src}}) \
         MERGE (res:Resource {{project_id: item.project_id, name: item.name, kind: item.kind}}) \
         ON CREATE SET res.filepath = item.name \
         SET res.filepath = coalesce(item.filepath, res.filepath), \
             res.last_seen_run = $run_id \
         MERGE (a)-[rel:{rel_name}]->(res) \
         SET rel.last_seen_run = $run_id"
    ))
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, &format!("write_{rel_name}")).await
}

pub(crate) async fn write_resource_backings(
    graph: &Arc<Graph>,
    batch: &[ResourceBackingRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (res:Resource {project_id: item.project_id, name: item.name, kind: item.kind}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         SET res.last_seen_run = $run_id \
         MERGE (res)-[rel:BACKED_BY_FILE]->(f) \
         SET rel.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_resource_backings").await
}

pub(crate) async fn write_xcode_targets(
    graph: &Arc<Graph>,
    batch: &[XcodeTargetRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         SET t.name = item.name, \
             t.project_file = item.project_file, \
             t.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_targets").await
}

pub(crate) async fn write_xcode_target_files(
    graph: &Arc<Graph>,
    batch: &[XcodeTargetFileRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         SET t.last_seen_run = $run_id \
         MERGE (t)-[r:BUNDLES_FILE]->(f) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_target_files").await
}

pub(crate) async fn write_xcode_target_resources(
    graph: &Arc<Graph>,
    batch: &[ResourceTargetEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (r:Resource {project_id: item.project_id, name: item.name, kind: item.kind}) \
         ON CREATE SET r.filepath = item.filepath \
         SET r.filepath = coalesce(item.filepath, r.filepath), \
             r.last_seen_run = $run_id \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         SET t.last_seen_run = $run_id \
         OPTIONAL MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         FOREACH (_ IN CASE WHEN f IS NULL THEN [] ELSE [1] END | \
             MERGE (r)-[back:BACKED_BY_FILE]->(f) \
             SET back.last_seen_run = $run_id) \
         MERGE (r)-[bundled:BUNDLED_IN_TARGET]->(t) \
         SET bundled.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_target_resources").await
}

pub(crate) async fn prune_stale_resource_data(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    for (label, rel_type) in [
        ("prune_stale_uses_asset", "USES_ASSET"),
        ("prune_stale_uses_color_asset", "USES_COLOR_ASSET"),
        ("prune_stale_uses_xib", "USES_XIB"),
        ("prune_stale_uses_storyboard", "USES_STORYBOARD"),
        ("prune_stale_backed_by_file", "BACKED_BY_FILE"),
        ("prune_stale_bundled_in_target", "BUNDLED_IN_TARGET"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":Node {project_id: $pid}",
            rel_type,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, label).await?;
    }

    let delete_nodes = Query::new(build_project_node_prune_cypher("Resource"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_nodes, "prune_stale_resources").await
}

pub(crate) async fn write_xcode_workspaces(
    graph: &Arc<Graph>,
    batch: &[XcodeWorkspaceRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (w:XcodeWorkspace {project_id: item.project_id, filepath: item.workspace_path}) \
         SET w.name = item.name, \
             w.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_workspaces").await
}

pub(crate) async fn write_xcode_workspace_projects(
    graph: &Arc<Graph>,
    batch: &[XcodeWorkspaceProjectRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (w:XcodeWorkspace {project_id: item.project_id, filepath: item.workspace_path}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         SET w.last_seen_run = $run_id \
         MERGE (w)-[r:REFERENCES_PROJECT]->(f) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_workspace_projects").await
}

pub(crate) async fn write_xcode_schemes(
    graph: &Arc<Graph>,
    batch: &[XcodeSchemeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         SET s.name = item.name, \
             s.container_path = item.container_path, \
             s.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_schemes").await
}

pub(crate) async fn write_xcode_scheme_targets(
    graph: &Arc<Graph>,
    batch: &[XcodeSchemeTargetRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         SET s.last_seen_run = $run_id, \
             t.last_seen_run = $run_id \
         MERGE (s)-[r:BUILDS_TARGET]->(t) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_scheme_targets").await
}

pub(crate) async fn write_xcode_scheme_files(
    graph: &Arc<Graph>,
    batch: &[XcodeSchemeFileRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         SET s.last_seen_run = $run_id \
         MERGE (s)-[r:DEFINED_IN_FILE]->(f) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_xcode_scheme_files").await
}

pub(crate) async fn prune_stale_xcode_data(graph: &Arc<Graph>, project_id: &str, run_id: &str) -> neo4rs::Result<()> {
    for (label, rel_type) in [
        ("prune_stale_xcode_target_files", "BUNDLES_FILE"),
        ("prune_stale_xcode_workspace_projects", "REFERENCES_PROJECT"),
        ("prune_stale_xcode_scheme_targets", "BUILDS_TARGET"),
        ("prune_stale_xcode_scheme_files", "DEFINED_IN_FILE"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":Node {project_id: $pid}",
            rel_type,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, label).await?;
    }

    let delete_targets = Query::new(build_project_node_prune_cypher("XcodeTarget"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_targets, "prune_stale_xcode_targets").await?;

    let delete_schemes = Query::new(build_project_node_prune_cypher("XcodeScheme"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_schemes, "prune_stale_xcode_schemes").await?;

    let delete_workspaces = Query::new(build_project_node_prune_cypher("XcodeWorkspace"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_workspaces, "prune_stale_xcode_workspaces").await
}

pub(crate) async fn write_cargo_crates(
    graph: &Arc<Graph>,
    batch: &[CargoCrateRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (c:CargoCrate {project_id: item.project_id, name: item.name}) \
         SET c.crate_name = item.crate_name, \
             c.manifest_path = coalesce(item.manifest_path, c.manifest_path), \
             c.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_cargo_crates").await
}

pub(crate) async fn write_cargo_workspaces(
    graph: &Arc<Graph>,
    batch: &[CargoWorkspaceRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (w:CargoWorkspace {project_id: item.project_id, filepath: item.manifest_path}) \
         SET w.name = item.name, \
             w.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_cargo_workspaces").await
}

pub(crate) async fn write_cargo_workspace_crates(
    graph: &Arc<Graph>,
    batch: &[CargoWorkspaceCrateRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (w:CargoWorkspace {project_id: item.project_id, filepath: item.workspace_manifest_path}) \
         MATCH (c:CargoCrate {project_id: item.project_id, name: item.crate_name}) \
         SET w.last_seen_run = $run_id, \
             c.last_seen_run = $run_id \
         MERGE (w)-[r:HAS_PACKAGE]->(c) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_cargo_workspace_crates").await
}

pub(crate) async fn write_cargo_crate_files(
    graph: &Arc<Graph>,
    batch: &[CargoCrateFileRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (c:CargoCrate {project_id: item.project_id, name: item.crate_name}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.manifest_path}) \
         SET c.last_seen_run = $run_id \
         MERGE (c)-[r:DEFINED_IN_FILE]->(f) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_cargo_crate_files").await
}

pub(crate) async fn write_cargo_dependency_edges(
    graph: &Arc<Graph>,
    batch: &[CargoDependencyEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (src:CargoCrate {project_id: item.project_id, name: item.src_crate_name}) \
         MATCH (tgt:CargoCrate {project_id: item.project_id, name: item.tgt_crate_name}) \
         SET src.last_seen_run = $run_id, \
             tgt.last_seen_run = $run_id \
         MERGE (src)-[r:DEPENDS_ON_PACKAGE]->(tgt) \
         SET r.section = item.section, \
             r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_cargo_dependency_edges").await
}

pub(crate) async fn prune_stale_cargo_data(graph: &Arc<Graph>, project_id: &str, run_id: &str) -> neo4rs::Result<()> {
    for (label, rel_type) in [
        ("prune_stale_cargo_workspace_crates", "HAS_PACKAGE"),
        ("prune_stale_cargo_crate_files", "DEFINED_IN_FILE"),
        ("prune_stale_cargo_dependencies", "DEPENDS_ON_PACKAGE"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":Node {project_id: $pid}",
            rel_type,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, label).await?;
    }

    let delete_workspaces = Query::new(build_project_node_prune_cypher("CargoWorkspace"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_workspaces, "prune_stale_cargo_workspaces").await?;

    let delete_crates = Query::new(build_project_node_prune_cypher("CargoCrate"))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
    run_query_logged(graph, delete_crates, "prune_stale_cargo_crates").await
}

pub(crate) async fn write_rust_impl_trait_edges(
    graph: &Arc<Graph>,
    batch: &[RustImplTraitEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (i:Impl {id: item.impl_id}) \
         MATCH (t:Trait {project_id: item.project_id, name: item.trait_name}) \
         MERGE (i)-[r:IMPLEMENTS_TRAIT]->(t) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_rust_impl_trait_edges").await
}

pub(crate) async fn write_rust_impl_type_edges(
    graph: &Arc<Graph>,
    batch: &[RustImplTypeEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (i:Impl {id: item.impl_id}) \
         MATCH (t:Node {project_id: item.project_id, name: item.type_name}) \
         WHERE (t:Struct OR t:Class OR t:Enum OR t:TypeAlias) \
         MERGE (i)-[r:IMPLEMENTS_TYPE]->(t) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_rust_impl_type_edges").await
}

pub(crate) async fn prune_stale_rust_impl_edges(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    for (label, rel_type) in [
        ("prune_stale_rust_impl_trait", "IMPLEMENTS_TRAIT"),
        ("prune_stale_rust_impl_type", "IMPLEMENTS_TYPE"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":Impl {project_id: $pid}",
            rel_type,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, label).await?;
    }
    Ok(())
}

pub(crate) async fn write_import_symbol_edges(
    graph: &Arc<Graph>,
    batch: &[ImportSymbolEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[r:IMPORTS_SYMBOL]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_import_symbol_edges").await
}

pub(crate) async fn write_implicit_import_symbol_edges(
    graph: &Arc<Graph>,
    batch: &[ImplicitImportSymbolEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[r:IMPLICIT_IMPORTS_SYMBOL]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_implicit_import_symbol_edges").await
}

pub(crate) async fn write_export_symbol_edges(
    graph: &Arc<Graph>,
    batch: &[ExportSymbolEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[r:EXPORTS_SYMBOL]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_export_symbol_edges").await
}

pub(crate) async fn write_export_alias_edges(
    graph: &Arc<Graph>,
    batch: &[ExportAliasEdgeRow],
    run_id: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[r:EXPORTS_SYMBOL_AS {name: item.exported_as}]->(b) \
         SET r.last_seen_run = $run_id"
            .to_string(),
    )
    .param("batch", bolt)
    .param("run_id", run_id.to_string());
    run_query_logged(graph, q, "write_export_alias_edges").await
}

pub(crate) async fn prune_stale_symbol_edge_data(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> neo4rs::Result<()> {
    for (label, rel_type) in [
        ("prune_stale_imports_symbol", "IMPORTS_SYMBOL"),
        ("prune_stale_implicit_imports_symbol", "IMPLICIT_IMPORTS_SYMBOL"),
        ("prune_stale_exports_symbol", "EXPORTS_SYMBOL"),
        ("prune_stale_exports_symbol_as", "EXPORTS_SYMBOL_AS"),
    ] {
        let q = Query::new(build_project_rel_prune_cypher(
            ":File {project_id: $pid}",
            rel_type,
            ":Node {project_id: $pid}",
        ))
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string());
        run_query_logged(graph, q, label).await?;
    }
    Ok(())
}

#[cfg(test)]
mod writer_consistency_tests {
    use super::{
        build_file_to_file_edge_write_cypher, build_project_node_prune_cypher, build_project_rel_prune_cypher,
    };
    use crate::{FileNode, ImportNode, SymbolNode};
    use std::sync::Arc;

    #[test]
    fn file_edge_write_cypher_stamps_run_id() {
        let cypher = build_file_to_file_edge_write_cypher("CALLS_SERVICE");
        assert!(cypher.contains("MERGE (a)-[r:CALLS_SERVICE]->(b)"));
        assert!(cypher.contains("SET r.last_seen_run = $run_id"));
        assert!(cypher.contains("MATCH (a:File {project_id: item.pid, filepath: item.src})"));
        assert!(cypher.contains("MATCH (b:File {project_id: item.pid, filepath: item.tgt})"));
    }

    #[test]
    fn project_rel_prune_cypher_is_project_scoped_and_run_scoped() {
        let cypher =
            build_project_rel_prune_cypher(":File {project_id: $pid}", "IMPORTS_SYMBOL", ":Node {project_id: $pid}");
        assert!(cypher.contains("MATCH (:File {project_id: $pid})-[r:IMPORTS_SYMBOL]->(:Node {project_id: $pid})"));
        assert!(cypher.contains("r.last_seen_run IS NULL OR r.last_seen_run <> $run_id"));
        assert!(cypher.contains("DELETE r"));
    }

    #[test]
    fn project_node_prune_cypher_targets_stale_project_nodes() {
        let cypher = build_project_node_prune_cypher("CargoCrate");
        assert!(cypher.contains("MATCH (n:CargoCrate {project_id: $pid})"));
        assert!(cypher.contains("n.last_seen_run IS NULL OR n.last_seen_run <> $run_id"));
        assert!(cypher.contains("DETACH DELETE n"));
    }

    #[test]
    fn project_core_rel_prune_cypher_is_safe_for_live_graph_cleanup() {
        let cypher = build_project_rel_prune_cypher(":Node {project_id: $pid}", "CALLS", ":Node {project_id: $pid}");
        assert!(cypher.contains("MATCH (:Node {project_id: $pid})-[r:CALLS]->(:Node {project_id: $pid})"));
        assert!(cypher.contains("r.last_seen_run IS NULL OR r.last_seen_run <> $run_id"));
        assert!(cypher.contains("DELETE r"));
    }

    #[test]
    fn core_nodes_serialize_stable_identity_fields() {
        let file = FileNode {
            id: "shadow:file:a.py".into(),
            stable_id: "canonical:file:a.py".into(),
            name: "a.py".into(),
            filepath: "a.py".into(),
            project_id: Arc::from("shadow"),
            is_test: false,
        };
        let file_json = file.to_value();
        assert_eq!(file_json["stable_id"], "canonical:file:a.py");

        let symbol = SymbolNode {
            id: "shadow:function:a.py:run".into(),
            stable_id: "canonical:function:a.py:run".into(),
            name: "run".into(),
            kind: "Function".into(),
            qualified_name: None,
            filepath: "a.py".into(),
            project_id: Arc::from("shadow"),
            start_line: 1,
            end_line: 1,
            start_byte: 0,
            end_byte: 3,
            signature: None,
            visibility: None,
            is_exported: true,
            doc_comment: None,
        };
        let symbol_json = symbol.to_value();
        assert_eq!(symbol_json["stable_id"], "canonical:function:a.py:run");

        let import_node = ImportNode {
            id: "shadow:import:a.py:pkg".into(),
            stable_id: "canonical:import:a.py:pkg".into(),
            file_id: "shadow:file:a.py".into(),
            stable_file_id: "canonical:file:a.py".into(),
            name: "pkg".into(),
            source: "pkg".into(),
            is_wildcard: false,
            project_id: Arc::from("shadow"),
            filepath: "a.py".into(),
        };
        let import_json = import_node.to_value();
        assert_eq!(import_json["stable_id"], "canonical:import:a.py:pkg");
        assert_eq!(import_json["stable_file_id"], "canonical:file:a.py");
    }
}

pub(crate) async fn write_launch_edges(graph: &Arc<Graph>, batch: &[LaunchEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.pid, filepath: item.src}) \
         MATCH (b:File {project_id: item.pid, filepath: item.tgt}) \
         MERGE (a)-[:LAUNCHES]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_launch_edges").await
}
