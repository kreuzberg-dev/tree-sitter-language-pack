use std::collections::HashMap;
use std::sync::Arc;

use neo4rs::{BoltType, Graph, Query};
use serde_json::Value;

use crate::{
    CloneCanonRow, CloneGroupRow, CloneMemberRow, DbEdgeRow, DbModelEdgeRow, ExportSymbolEdgeRow,
    ExternalApiEdgeRow, ExternalApiNode, FileCloneCanonRow, FileCloneGroupRow, FileCloneMemberRow,
    FileNode, ImportNode, ImportSymbolEdgeRow, ImplicitImportSymbolEdgeRow, InferredCallRow,
    LaunchEdgeRow, PythonInferredCallRow, RelRow, SymbolCallRow, SymbolNode,
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

pub(crate) async fn write_file_nodes(graph: &Arc<Graph>, batch: &[FileNode]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:File, \
                       n.name       = item.name, \
                       n.filepath   = item.filepath, \
                       n.project_id = item.project_id \
         ON MATCH SET  n.name       = item.name"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_symbol_nodes(graph: &Arc<Graph>, batch: &[SymbolNode], label: &str) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_import_nodes(graph: &Arc<Graph>, batch: &[ImportNode]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_relationships(graph: &Arc<Graph>, batch: &[RelRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (p:Node {id: item.p}) \
         MATCH (c:Node {id: item.c}) \
         MERGE (p)-[:CONTAINS]->(c)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_calls(graph: &Arc<Graph>, batch: &[SymbolCallRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_inferred_calls(graph: &Arc<Graph>, batch: &[InferredCallRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_python_inferred_calls(graph: &Arc<Graph>, batch: &[PythonInferredCallRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_db_edges(graph: &Arc<Graph>, batch: &[DbEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:File {id: item.tgt}) \
         MERGE (a)-[:CALLS_DB]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_db_model_edges(graph: &Arc<Graph>, batch: &[DbModelEdgeRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_external_api_nodes(graph: &Arc<Graph>, batch: &[ExternalApiNode]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (e:ExternalAPI {id: item.id}) \
         SET e.project_id = item.pid, \
             e.url = item.url"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_clone_groups(graph: &Arc<Graph>, batch: &[CloneGroupRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_clone_members(graph: &Arc<Graph>, batch: &[CloneMemberRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (s)-[:MEMBER_OF_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_clone_canon(graph: &Arc<Graph>, batch: &[CloneCanonRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (g)-[:HAS_CANONICAL]->(s)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_file_clone_groups(graph: &Arc<Graph>, batch: &[FileCloneGroupRow]) {
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
    let _ = graph.run(q).await;
}

pub(crate) async fn write_file_clone_members(graph: &Arc<Graph>, batch: &[FileCloneMemberRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (f)-[:MEMBER_OF_FILE_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_file_clone_canon(graph: &Arc<Graph>, batch: &[FileCloneCanonRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (g)-[:HAS_CANONICAL]->(f)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_external_api_edges(graph: &Arc<Graph>, batch: &[ExternalApiEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:ExternalAPI {id: item.tgt}) \
         MERGE (a)-[:CALLS_API_EXTERNAL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImportSymbolEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_implicit_import_symbol_edges(
    graph: &Arc<Graph>,
    batch: &[ImplicitImportSymbolEdgeRow],
) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPLICIT_IMPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_export_symbol_edges(graph: &Arc<Graph>, batch: &[ExportSymbolEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:EXPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

pub(crate) async fn write_launch_edges(graph: &Arc<Graph>, batch: &[LaunchEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.pid, filepath: item.src}) \
         MATCH (b:File {project_id: item.pid, filepath: item.tgt}) \
         MERGE (a)-[:LAUNCHES]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}
