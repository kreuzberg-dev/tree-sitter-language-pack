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

pub(crate) async fn write_file_nodes(graph: &Arc<Graph>, batch: &[FileNode]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:File, \
                       n.name       = item.name, \
                       n.filepath   = item.filepath, \
                       n.project_id = item.project_id, \
                       n.is_test    = item.is_test \
         ON MATCH SET  n.name       = item.name, \
                       n.is_test    = item.is_test"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_file_nodes").await
}

pub(crate) async fn write_symbol_nodes(graph: &Arc<Graph>, batch: &[SymbolNode], label: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let cypher = format!(
        "UNWIND $batch AS item \
         MERGE (n:Node {{id: item.id}}) \
         ON CREATE SET n:{label}, \
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
                       n.doc_comment = item.doc_comment \
         ON MATCH SET  n.start_line  = item.start_line, \
                       n.end_line    = item.end_line, \
                       n.qualified_name = item.qualified_name, \
                       n.signature   = item.signature, \
                       n.visibility  = item.visibility, \
                       n.is_exported = item.is_exported, \
                       n.doc_comment = item.doc_comment \
         FOREACH (_ IN CASE WHEN item.kind = 'Method' THEN [1] ELSE [] END | SET n:Method)"
    );
    let q = Query::new(cypher).param("batch", bolt);
    run_query_logged(graph, q, "write_symbol_nodes").await
}

pub(crate) async fn write_import_nodes(graph: &Arc<Graph>, batch: &[ImportNode]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:Import, \
                       n.name        = item.name, \
                       n.source      = item.source, \
                       n.is_wildcard = item.is_wildcard, \
                       n.project_id  = item.project_id, \
                       n.filepath    = item.filepath"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_import_nodes").await
}

pub(crate) async fn write_relationships(graph: &Arc<Graph>, batch: &[RelRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (p:Node {id: item.p}) \
         MATCH (c:Node {id: item.c}) \
         MERGE (p)-[:CONTAINS]->(c)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_relationships").await
}

pub(crate) async fn write_calls(graph: &Arc<Graph>, batch: &[SymbolCallRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[:CALLS]->(callee)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_calls").await
}

pub(crate) async fn write_inferred_calls(graph: &Arc<Graph>, batch: &[InferredCallRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND callee.qualified_name IS NOT NULL \
           AND callee.qualified_name STARTS WITH item.recv + '.' \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[:CALLS_INFERRED]->(callee)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_inferred_calls").await
}

pub(crate) async fn write_python_inferred_calls(
    graph: &Arc<Graph>,
    batch: &[PythonInferredCallRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee, filepath: item.callee_fp}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[:CALLS_INFERRED]->(callee)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_python_inferred_calls").await
}

pub(crate) async fn write_db_edges(graph: &Arc<Graph>, batch: &[DbEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:File {id: item.tgt}) \
         MERGE (a)-[:CALLS_DB]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_db_edges").await
}

pub(crate) async fn write_db_model_edges(graph: &Arc<Graph>, batch: &[DbModelEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (m:Model {id: item.pid + ':model:' + item.model}) \
         SET m.project_id = item.pid, m.name = item.model \
         WITH item, m \
         MATCH (a:File {id: item.src}) \
         MERGE (a)-[:CALLS_DB_MODEL]->(m)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_db_model_edges").await
}

pub(crate) async fn write_external_api_nodes(graph: &Arc<Graph>, batch: &[ExternalApiNode]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (e:ExternalAPI {id: item.id}) \
         SET e.project_id = item.pid, \
             e.url = item.url"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_external_api_nodes").await
}

pub(crate) async fn write_clone_groups(graph: &Arc<Graph>, batch: &[CloneGroupRow]) -> neo4rs::Result<()> {
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
             g.created_at = timestamp()"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_clone_groups").await
}

pub(crate) async fn write_clone_members(graph: &Arc<Graph>, batch: &[CloneMemberRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (s)-[:MEMBER_OF_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_clone_members").await
}

pub(crate) async fn write_clone_canon(graph: &Arc<Graph>, batch: &[CloneCanonRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (g)-[:HAS_CANONICAL]->(s)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_clone_canon").await
}

pub(crate) async fn write_file_clone_groups(graph: &Arc<Graph>, batch: &[FileCloneGroupRow]) -> neo4rs::Result<()> {
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
             g.created_at = timestamp()"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_file_clone_groups").await
}

pub(crate) async fn write_file_clone_members(graph: &Arc<Graph>, batch: &[FileCloneMemberRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (f)-[:MEMBER_OF_FILE_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_file_clone_members").await
}

pub(crate) async fn write_file_clone_canon(graph: &Arc<Graph>, batch: &[FileCloneCanonRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (g)-[:HAS_CANONICAL]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_file_clone_canon").await
}

pub(crate) async fn write_external_api_edges(graph: &Arc<Graph>, batch: &[ExternalApiEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:ExternalAPI {id: item.tgt}) \
         MERGE (a)-[:CALLS_API_EXTERNAL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_external_api_edges").await
}

pub(crate) async fn write_file_import_edges(graph: &Arc<Graph>, batch: &[FileImportEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.pid, filepath: item.src}) \
         MATCH (b:File {project_id: item.pid, filepath: item.tgt}) \
         MERGE (a)-[:IMPORTS]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_file_import_edges").await
}

pub(crate) async fn write_file_edges(graph: &Arc<Graph>, batch: &[FileEdgeRow], rel_name: &str) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(format!(
        "UNWIND $batch AS item \
         MATCH (a:File {{project_id: item.pid, filepath: item.src}}) \
         MATCH (b:File {{project_id: item.pid, filepath: item.tgt}}) \
         MERGE (a)-[:{rel_name}]->(b)"
    ))
    .param("batch", bolt);
    run_query_logged(graph, q, &format!("write_{rel_name}")).await
}

pub(crate) async fn write_api_route_calls(graph: &Arc<Graph>, batch: &[ApiRouteCallRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.project_id, filepath: item.src}) \
         MERGE (r:ApiRoute {project_id: item.project_id, path: item.path, method: item.method}) \
         ON CREATE SET r.name = item.method + ' ' + item.path, r.filepath = item.path \
         SET r.filepath = item.path \
         MERGE (a)-[:CALLS_API_ROUTE]->(r)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_api_route_calls").await
}

pub(crate) async fn write_api_route_handlers(graph: &Arc<Graph>, batch: &[ApiRouteHandlerRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (r:ApiRoute {project_id: item.project_id, path: item.path, method: item.method}) \
         MATCH (b:File {project_id: item.project_id, filepath: item.tgt}) \
         MERGE (r)-[:HANDLED_BY]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_api_route_handlers").await
}

pub(crate) async fn write_resource_usage_edges(
    graph: &Arc<Graph>,
    batch: &[ResourceUsageRow],
    rel_name: &str,
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(format!(
        "UNWIND $batch AS item \
         MATCH (a:File {{project_id: item.project_id, filepath: item.src}}) \
         MERGE (res:Resource {{project_id: item.project_id, name: item.name, kind: item.kind}}) \
         ON CREATE SET res.filepath = item.name \
         SET res.filepath = coalesce(item.filepath, res.filepath) \
         MERGE (a)-[:{rel_name}]->(res)"
    ))
    .param("batch", bolt);
    run_query_logged(graph, q, &format!("write_{rel_name}")).await
}

pub(crate) async fn write_resource_backings(graph: &Arc<Graph>, batch: &[ResourceBackingRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (res:Resource {project_id: item.project_id, name: item.name, kind: item.kind}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (res)-[:BACKED_BY_FILE]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_resource_backings").await
}

pub(crate) async fn write_xcode_targets(graph: &Arc<Graph>, batch: &[XcodeTargetRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         SET t.name = item.name, t.project_file = item.project_file"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_targets").await
}

pub(crate) async fn write_xcode_target_files(graph: &Arc<Graph>, batch: &[XcodeTargetFileRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (t)-[:BUNDLES_FILE]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_target_files").await
}

pub(crate) async fn write_xcode_target_resources(
    graph: &Arc<Graph>,
    batch: &[ResourceTargetEdgeRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (r:Resource {project_id: item.project_id, name: item.name, kind: item.kind}) \
         ON CREATE SET r.filepath = item.filepath \
         SET r.filepath = coalesce(item.filepath, r.filepath) \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         OPTIONAL MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         FOREACH (_ IN CASE WHEN f IS NULL THEN [] ELSE [1] END | MERGE (r)-[:BACKED_BY_FILE]->(f)) \
         MERGE (r)-[:BUNDLED_IN_TARGET]->(t)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_target_resources").await
}

pub(crate) async fn write_xcode_workspaces(graph: &Arc<Graph>, batch: &[XcodeWorkspaceRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (w:XcodeWorkspace {project_id: item.project_id, filepath: item.workspace_path}) \
         SET w.name = item.name"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_workspaces").await
}

pub(crate) async fn write_xcode_workspace_projects(
    graph: &Arc<Graph>,
    batch: &[XcodeWorkspaceProjectRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (w:XcodeWorkspace {project_id: item.project_id, filepath: item.workspace_path}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (w)-[:REFERENCES_PROJECT]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_workspace_projects").await
}

pub(crate) async fn write_xcode_schemes(graph: &Arc<Graph>, batch: &[XcodeSchemeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         SET s.name = item.name, s.container_path = item.container_path"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_schemes").await
}

pub(crate) async fn write_xcode_scheme_targets(
    graph: &Arc<Graph>,
    batch: &[XcodeSchemeTargetRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         MATCH (t:XcodeTarget {project_id: item.project_id, target_id: item.target_id}) \
         MERGE (s)-[:BUILDS_TARGET]->(t)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_scheme_targets").await
}

pub(crate) async fn write_xcode_scheme_files(graph: &Arc<Graph>, batch: &[XcodeSchemeFileRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (s:XcodeScheme {project_id: item.project_id, filepath: item.scheme_path}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (s)-[:DEFINED_IN_FILE]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_xcode_scheme_files").await
}

pub(crate) async fn write_cargo_crates(graph: &Arc<Graph>, batch: &[CargoCrateRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (c:CargoCrate {project_id: item.project_id, name: item.name}) \
         SET c.crate_name = item.crate_name, \
             c.manifest_path = coalesce(item.manifest_path, c.manifest_path)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_cargo_crates").await
}

pub(crate) async fn write_cargo_workspaces(graph: &Arc<Graph>, batch: &[CargoWorkspaceRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (w:CargoWorkspace {project_id: item.project_id, filepath: item.manifest_path}) \
         SET w.name = item.name"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_cargo_workspaces").await
}

pub(crate) async fn write_cargo_workspace_crates(
    graph: &Arc<Graph>,
    batch: &[CargoWorkspaceCrateRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (w:CargoWorkspace {project_id: item.project_id, filepath: item.workspace_manifest_path}) \
         MATCH (c:CargoCrate {project_id: item.project_id, name: item.crate_name}) \
         MERGE (w)-[:HAS_PACKAGE]->(c)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_cargo_workspace_crates").await
}

pub(crate) async fn write_cargo_crate_files(graph: &Arc<Graph>, batch: &[CargoCrateFileRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (c:CargoCrate {project_id: item.project_id, name: item.crate_name}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.manifest_path}) \
         MERGE (c)-[:DEFINED_IN_FILE]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_cargo_crate_files").await
}

pub(crate) async fn write_cargo_dependency_edges(
    graph: &Arc<Graph>,
    batch: &[CargoDependencyEdgeRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (src:CargoCrate {project_id: item.project_id, name: item.src_crate_name}) \
         MATCH (tgt:CargoCrate {project_id: item.project_id, name: item.tgt_crate_name}) \
         MERGE (src)-[r:DEPENDS_ON_PACKAGE]->(tgt) \
         SET r.section = item.section"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_cargo_dependency_edges").await
}

pub(crate) async fn write_rust_impl_trait_edges(
    graph: &Arc<Graph>,
    batch: &[RustImplTraitEdgeRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (i:Impl {id: item.impl_id}) \
         MATCH (t:Trait {project_id: item.project_id, name: item.trait_name}) \
         MERGE (i)-[:IMPLEMENTS_TRAIT]->(t)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_rust_impl_trait_edges").await
}

pub(crate) async fn write_rust_impl_type_edges(
    graph: &Arc<Graph>,
    batch: &[RustImplTypeEdgeRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (i:Impl {id: item.impl_id}) \
         MATCH (t:Node {project_id: item.project_id, name: item.type_name}) \
         WHERE (t:Struct OR t:Class OR t:Enum OR t:TypeAlias) \
         MERGE (i)-[:IMPLEMENTS_TYPE]->(t)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_rust_impl_type_edges").await
}

pub(crate) async fn write_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImportSymbolEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_import_symbol_edges").await
}

pub(crate) async fn write_implicit_import_symbol_edges(
    graph: &Arc<Graph>,
    batch: &[ImplicitImportSymbolEdgeRow],
) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPLICIT_IMPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_implicit_import_symbol_edges").await
}

pub(crate) async fn write_export_symbol_edges(graph: &Arc<Graph>, batch: &[ExportSymbolEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:EXPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_export_symbol_edges").await
}

pub(crate) async fn write_export_alias_edges(graph: &Arc<Graph>, batch: &[ExportAliasEdgeRow]) -> neo4rs::Result<()> {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:EXPORTS_SYMBOL_AS {name: item.exported_as}]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    run_query_logged(graph, q, "write_export_alias_edges").await
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
