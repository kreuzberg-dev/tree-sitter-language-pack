use neo4rs::{query, BoltType, ConfigBuilder, Graph, Query};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use ts_pack_index::graph_schema;

use crate::swift_semantic;

mod file_graph;
mod gds;
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

    let mut result = graph
        .execute(query("MATCH (f:File {project_id:$pid}) RETURN f.filepath AS fp").param("pid", project_id.to_string()))
        .await?;
    let mut existing_paths = HashSet::new();
    while let Some(row) = result.next().await? {
        let fp: String = row.get("fp").unwrap_or_default();
        if !fp.is_empty() {
            existing_paths.insert(fp);
        }
    }

    let mut missing_rows = Vec::new();
    let mut manifest_paths = Vec::new();
    let stable_project_id = canonical_project_id(project_id);
    for entry in &manifest {
        let Some(fp) = entry.get("rel_path").and_then(Value::as_str) else {
            continue;
        };
        let filepath = fp.replace('\\', "/");
        manifest_paths.push(filepath.clone());
        if existing_paths.contains(&filepath) {
            continue;
        }
        let name = Path::new(&filepath)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        missing_rows.push(json!({
            "id": format!("{project_id}:file:{filepath}"),
            "stable_id": format!("{stable_project_id}:file:{filepath}"),
            "filepath": filepath,
            "name": name,
        }));
    }

    let mut added = 0i64;
    for chunk in missing_rows.chunks(200) {
        run(
            graph,
            query(
                "UNWIND $batch AS row
                 MERGE (f {id: row.id})
                 ON CREATE SET
                    f:Node,
                    f:File,
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
        added += chunk.len() as i64;
    }

    run(
        graph,
        query(
            "MATCH (f:File {project_id:$pid})
             WHERE f.filepath IN $paths
             SET f:Node, f.last_seen_run = $run_id",
        )
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string())
        .param("paths", strings_to_bolt(&manifest_paths)),
    )
    .await?;

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

    let added_manifest = ensure_manifest_file_nodes(&graph, project_id, &current_run_id, manifest_file).await?;
    let root = fs::canonicalize(project_path).unwrap_or_else(|_| Path::new(project_path).to_path_buf());
    let parsed_paths = indexed_files
        .iter()
        .filter_map(|path| fs::canonicalize(path).ok())
        .filter_map(|path| path.strip_prefix(&root).ok().map(|p| p.to_path_buf()))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    let parsed_marked = mark_manifest_parsed(&graph, project_id, &current_run_id, &parsed_paths).await?;
    let aliased = sync_file_path_alias(&graph, project_id, &current_run_id).await?;
    let swift_enrichment = swift_semantic::enrich_swift_graph_async(
        project_path,
        project_id,
        indexed_files,
        neo4j_uri,
        neo4j_user,
        neo4j_pass,
        neo4j_db,
    )
    .await?;
    let file_call_edges = file_graph::build_file_calls_from_symbol_graph(&graph, project_id, &current_run_id).await?;
    let file_graph_links = file_graph::build_file_graph_links(&graph, project_id, &current_run_id).await?;
    let rels = reporting::file_graph_projection_rels(FILE_GRAPH_PROJECTION_RELS);
    let pagerank = gds::run_pagerank(&graph, project_id).await.unwrap_or(0);
    let louvain = gds::run_file_gds(
        &graph,
        "louvain",
        project_id,
        &rels,
        "CALL gds.leiden.write($name, { writeProperty: 'louvainCommunity', gamma: 1.0 })",
        "MATCH (f:File {project_id: $pid}) WHERE f.louvainCommunity IS NOT NULL RETURN count(f) AS updated",
        "updated",
    )
    .await
    .unwrap_or_else(|err| json!({"status": "failed", "updated": 0, "error": err.to_string()}));
    let betweenness = gds::run_betweenness_gds(&graph, project_id, &rels)
        .await
        .unwrap_or_else(|err| json!({"status": "failed", "updated": 0, "error": err.to_string()}));
    let isolated = gds::run_isolated_file_gds(&graph, project_id, &rels).await;

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
