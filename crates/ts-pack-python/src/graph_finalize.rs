use neo4rs::{BoltType, ConfigBuilder, Graph, Query, query};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};
use ts_pack_index::graph_schema;

use crate::swift_semantic;

mod file_graph;
mod gds;
pub(crate) mod provenance;
mod reporting;

const FILE_GRAPH_SOURCE_RELS: &[&str] = &[
    graph_schema::REL_IMPORTS,
    graph_schema::REL_ASSET_LINKS,
    graph_schema::REL_CALLS_API,
    graph_schema::REL_CALLS_SERVICE,
    graph_schema::REL_CALLS_DB,
    graph_schema::REL_CALLS_FILE,
];

const FILE_GRAPH_PROJECTION_RELS: &[&str] = &[graph_schema::REL_FILE_GRAPH_LINK];

fn graph_name(prefix: &str, project_id: &str) -> String {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{prefix}-{project_id}-{stamp:x}")
}

fn canonical_project_id(project_id: &str) -> &str {
    project_id
        .split_once("::shadow::")
        .map(|(canonical, _)| canonical)
        .unwrap_or(project_id)
}

fn strings_to_bolt(items: &[String]) -> BoltType {
    BoltType::from(items.iter().cloned().map(BoltType::from).collect::<Vec<_>>())
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

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(default)
}

fn env_i64(name: &str, default: i64) -> i64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(default)
}

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(default)
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

async fn run(graph: &Arc<Graph>, q: Query) -> Result<(), Box<dyn std::error::Error>> {
    let attempts = write_retry_attempts();
    let base_ms = write_retry_base_ms();
    for attempt in 1..=attempts {
        match graph.run(q.clone()).await {
            Ok(()) => return Ok(()),
            Err(err) => {
                if is_retryable_neo4j_write_error(&err) && attempt < attempts {
                    let delay_ms = base_ms.saturating_mul(attempt as u64);
                    eprintln!(
                        "[ts-pack-python] neo4j finalize retry attempt {attempt}/{attempts} after {delay_ms}ms: {err}"
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    continue;
                }
                return Err(Box::new(err));
            }
        }
    }
    Ok(())
}

async fn one_i64(graph: &Arc<Graph>, q: Query, key: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut result = graph.execute(q).await?;
    if let Some(row) = result.next().await? {
        if let Ok(value) = row.get::<i64>(key) {
            return Ok(value);
        }
        if let Ok(value) = row.get::<i32>(key) {
            return Ok(value as i64);
        }
        if let Ok(value) = row.to::<i64>() {
            return Ok(value);
        }
        if let Ok(value) = row.to::<i32>() {
            return Ok(value as i64);
        }
    }
    Ok(0)
}

async fn one_bool(graph: &Arc<Graph>, q: Query, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut result = graph.execute(q).await?;
    if let Some(row) = result.next().await? {
        if let Ok(value) = row.get::<bool>(key) {
            return Ok(value);
        }
        if let Ok(value) = row.get::<i64>(key) {
            return Ok(value != 0);
        }
        if let Ok(value) = row.get::<i32>(key) {
            return Ok(value != 0);
        }
    }
    Ok(false)
}

async fn estimate_bytes_max(graph: &Arc<Graph>, q: Query) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    let mut result = graph.execute(q).await?;
    if let Some(row) = result.next().await? {
        if let Ok(value) = row.get::<i64>("bytesMax") {
            return Ok(Some(value));
        }
        if let Ok(value) = row.get::<i32>("bytesMax") {
            return Ok(Some(value as i64));
        }
    }
    Ok(None)
}

async fn ensure_manifest_file_nodes(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
    manifest_file: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let manifest_text = match fs::read_to_string(manifest_file) {
        Ok(text) => text,
        Err(_) => return Ok(0),
    };
    let manifest: Vec<Value> = serde_json::from_str(&manifest_text)?;
    if manifest.is_empty() {
        return Ok(0);
    }

    let mut manifest_ids = Vec::new();
    let mut manifest_rows = Vec::new();
    let stable_project_id = canonical_project_id(project_id);
    for entry in &manifest {
        let Some(fp) = entry.get("rel_path").and_then(Value::as_str) else {
            continue;
        };
        let filepath = fp.replace('\\', "/");
        let node_id = format!("{project_id}:file:{filepath}");
        manifest_ids.push(node_id.clone());
        let name = Path::new(&filepath)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        manifest_rows.push(json!({
            "id": node_id,
            "stable_id": format!("{stable_project_id}:file:{filepath}"),
            "filepath": filepath,
            "name": name,
        }));
    }
    if manifest_ids.is_empty() {
        return Ok(0);
    }
    let mut result = graph
        .execute(
            query(
                "MATCH (f:File)
                 WHERE f.id IN $ids
                 RETURN f.id AS file_id",
            )
            .param("ids", strings_to_bolt(&manifest_ids)),
        )
        .await?;
    let mut existing_ids = std::collections::HashSet::new();
    while let Some(row) = result.next().await? {
        let file_id: String = row.get("file_id").unwrap_or_default();
        if !file_id.is_empty() {
            existing_ids.insert(file_id);
        }
    }

    let mut existing_rows = Vec::new();
    let mut missing_rows = Vec::new();
    for row in manifest_rows {
        let is_existing = row
            .get("id")
            .and_then(Value::as_str)
            .map(|id| existing_ids.contains(id))
            .unwrap_or(false);
        if is_existing {
            existing_rows.push(row);
        } else {
            missing_rows.push(row);
        }
    }

    for chunk in existing_rows.chunks(200) {
        run(
            graph,
            query(
                "UNWIND $batch AS row
                 MATCH (f:File {id:row.id})
                 SET f:Node,
                     f.project_id = $pid,
                     f.filepath = row.filepath,
                     f.file_path = row.filepath,
                     f.last_seen_run = $run_id",
            )
            .param("pid", project_id.to_string())
            .param("run_id", run_id.to_string())
            .param(
                "batch",
                BoltType::from(
                    chunk
                        .iter()
                        .cloned()
                        .map(crate::swift_semantic::json_to_bolt)
                        .collect::<Vec<_>>(),
                ),
            ),
        )
        .await?;
    }

    let added = missing_rows.len() as i64;
    for chunk in missing_rows.chunks(200) {
        run(
            graph,
            query(
                "UNWIND $batch AS row
                 MERGE (f:File {id: row.id})
                 ON CREATE SET
                    f:Node,
                    f.stable_id = row.stable_id,
                    f.project_id = $pid,
                    f.filepath = row.filepath,
                    f.file_path = row.filepath,
                    f.name = row.name,
                    f.indexed_at = timestamp(),
                    f.parsed = false,
                    f.last_seen_run = $run_id
                 SET f:Node,
                     f:File,
                     f.stable_id = row.stable_id,
                     f.project_id = $pid,
                     f.filepath = row.filepath,
                    f.file_path = row.filepath,
                     f.last_seen_run = $run_id",
            )
            .param("pid", project_id.to_string())
            .param("run_id", run_id.to_string())
            .param(
                "batch",
                BoltType::from(
                    chunk
                        .iter()
                        .cloned()
                        .map(crate::swift_semantic::json_to_bolt)
                        .collect::<Vec<_>>(),
                ),
            ),
        )
        .await?;
    }
    Ok(added)
}

async fn mark_manifest_parsed(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
    parsed_paths: &[String],
) -> Result<i64, Box<dyn std::error::Error>> {
    if parsed_paths.is_empty() {
        return Ok(0);
    }
    one_i64(
        graph,
        query(
            "MATCH (f:File {project_id:$pid})
             WHERE f.filepath IN $paths AND (f.parsed IS NULL OR f.parsed = false)
             SET f.parsed = true, f.last_seen_run = $run_id
             RETURN count(f) AS updated",
        )
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string())
        .param("paths", strings_to_bolt(parsed_paths)),
        "updated",
    )
    .await
}

async fn sync_file_path_alias(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    one_i64(
        graph,
        query(
            "MATCH (f:File {project_id:$pid})
             WHERE f.filepath IS NOT NULL
             SET f.file_path = f.filepath, f.last_seen_run = $run_id
             RETURN count(f) AS updated",
        )
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string()),
        "updated",
    )
    .await
}

async fn count_rel(graph: &Arc<Graph>, project_id: &str, rel: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let cypher =
        format!("MATCH (a:File {{project_id: $pid}})-[:{rel}]->(b:File {{project_id: $pid}}) RETURN count(*) AS n");
    one_i64(graph, query(&cypher).param("pid", project_id.to_string()), "n").await
}

async fn count_file_nodes(graph: &Arc<Graph>, project_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
    one_i64(
        graph,
        query("MATCH (f:File {project_id: $pid}) RETURN count(f) AS n").param("pid", project_id.to_string()),
        "n",
    )
    .await
}

pub(super) async fn drop_graph(graph: &Arc<Graph>, graph_name: &str) {
    let _ = run(
        graph,
        query("CALL gds.graph.drop($name, false) YIELD graphName").param("name", graph_name.to_string()),
    )
    .await;
}

pub async fn finalize_struct_graph_async(
    project_path: &str,
    project_id: &str,
    manifest_file: &str,
    indexed_files: &[String],
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    neo4j_db: &str,
    run_id: Option<&str>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let finalize_started_at = Instant::now();
    let neo4j_config = ConfigBuilder::default()
        .uri(neo4j_uri)
        .user(neo4j_user)
        .password(neo4j_pass)
        .db(neo4j_db)
        .max_connections(8)
        .fetch_size(500)
        .build()?;
    let graph = Arc::new(Graph::connect(neo4j_config).await?);
    let current_run_id = if let Some(run_id) = run_id {
        run_id.to_string()
    } else {
        let mut result = graph
            .execute(
                query("MATCH (p:Project {id:$pid}) RETURN p.struct_index_run_id AS run_id")
                    .param("pid", project_id.to_string()),
            )
            .await?;
        if let Some(row) = result.next().await? {
            row.get::<String>("run_id").unwrap_or_default()
        } else {
            String::new()
        }
    };
    if current_run_id.is_empty() {
        return Err(format!("missing struct_index_run_id for project {project_id}").into());
    }

    let ensure_manifest_started_at = Instant::now();
    let added_manifest = ensure_manifest_file_nodes(&graph, project_id, &current_run_id, manifest_file).await?;
    eprintln!(
        "[ts-pack-finalize] ensure_manifest_file_nodes done in {:.2}s (added={})",
        ensure_manifest_started_at.elapsed().as_secs_f64(),
        added_manifest,
    );
    let root = fs::canonicalize(project_path).unwrap_or_else(|_| Path::new(project_path).to_path_buf());
    let parsed_paths = indexed_files
        .iter()
        .filter_map(|path| fs::canonicalize(path).ok())
        .filter_map(|path| path.strip_prefix(&root).ok().map(|p| p.to_path_buf()))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    let mark_parsed_started_at = Instant::now();
    let parsed_marked = mark_manifest_parsed(&graph, project_id, &current_run_id, &parsed_paths).await?;
    eprintln!(
        "[ts-pack-finalize] mark_manifest_parsed done in {:.2}s (updated={})",
        mark_parsed_started_at.elapsed().as_secs_f64(),
        parsed_marked,
    );
    let alias_started_at = Instant::now();
    let aliased = sync_file_path_alias(&graph, project_id, &current_run_id).await?;
    eprintln!(
        "[ts-pack-finalize] sync_file_path_alias done in {:.2}s (updated={})",
        alias_started_at.elapsed().as_secs_f64(),
        aliased,
    );
    let swift_enrichment_started_at = Instant::now();
    let swift_enrichment = swift_semantic::enrich_swift_graph_async(
        project_path,
        project_id,
        indexed_files,
        neo4j_uri,
        neo4j_user,
        neo4j_pass,
        neo4j_db,
        &current_run_id,
    )
    .await?;
    eprintln!(
        "[ts-pack-finalize] enrich_swift_graph_async done in {:.2}s",
        swift_enrichment_started_at.elapsed().as_secs_f64(),
    );
    let file_calls_started_at = Instant::now();
    let file_call_edges = file_graph::build_file_calls_from_symbol_graph(&graph, project_id, &current_run_id).await?;
    eprintln!(
        "[ts-pack-finalize] build_file_calls_from_symbol_graph done in {:.2}s (edges={})",
        file_calls_started_at.elapsed().as_secs_f64(),
        file_call_edges,
    );
    let file_graph_links_started_at = Instant::now();
    let file_graph_links = file_graph::build_file_graph_links(&graph, project_id, &current_run_id).await?;
    eprintln!(
        "[ts-pack-finalize] build_file_graph_links done in {:.2}s (edges={})",
        file_graph_links_started_at.elapsed().as_secs_f64(),
        file_graph_links,
    );
    let rels = reporting::file_graph_projection_rels(FILE_GRAPH_PROJECTION_RELS);
    let pagerank_started_at = Instant::now();
    let pagerank = gds::run_pagerank(&graph, project_id).await.unwrap_or(0);
    eprintln!(
        "[ts-pack-finalize] run_pagerank done in {:.2}s (updated={})",
        pagerank_started_at.elapsed().as_secs_f64(),
        pagerank,
    );
    let standard_gds_started_at = Instant::now();
    let gds::StandardFileGdsResults {
        louvain,
        betweenness,
        isolated,
    } = gds::run_standard_file_gds_jobs(&graph, project_id, &rels).await;
    eprintln!(
        "[ts-pack-finalize] run_standard_file_gds_jobs done in {:.2}s",
        standard_gds_started_at.elapsed().as_secs_f64(),
    );
    eprintln!(
        "[ts-pack-finalize] finalize_struct_graph total {:.2}s",
        finalize_started_at.elapsed().as_secs_f64(),
    );

    Ok(reporting::finalize_payload(
        added_manifest,
        parsed_marked,
        aliased,
        file_call_edges,
        file_graph_links,
        swift_enrichment,
        pagerank,
        louvain,
        betweenness,
        isolated,
    ))
}
