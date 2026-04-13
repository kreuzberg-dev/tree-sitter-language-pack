use neo4rs::{BoltType, ConfigBuilder, Graph, Query, query};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

use crate::swift_semantic;

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

async fn build_file_calls_from_symbol_graph(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    one_i64(
        graph,
        query(
            "MATCH (src:File {project_id:$pid})-[:CONTAINS]->(caller:Node {project_id:$pid})
             MATCH (caller)-[:CALLS|CALLS_INFERRED]->(callee:Node {project_id:$pid})
             MATCH (dst:File {project_id:$pid})-[:CONTAINS]->(callee)
             WHERE src <> dst
             MERGE (src)-[r:CALLS_FILE]->(dst)
             SET r.project_id = $pid,
                 r.last_seen_run = $run_id
             RETURN count(DISTINCT r) AS updated",
        )
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string()),
        "updated",
    )
    .await
}

async fn build_file_graph_links(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut total = 0i64;

    for rel in [
        "IMPORTS",
        "ASSET_LINKS",
        "CALLS_API",
        "CALLS_SERVICE",
        "CALLS_DB",
        "CALLS_FILE",
    ] {
        let cypher = format!(
            "MATCH (src:File {{project_id:$pid}})-[:{rel}]->(dst:File {{project_id:$pid}})
             WHERE src <> dst
             MERGE (src)-[r:FILE_GRAPH_LINK]->(dst)
             SET r.project_id = $pid,
                 r.last_seen_run = $run_id
             RETURN count(DISTINCT r) AS updated"
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
        query(
            "MATCH (src:File {project_id:$pid})-[:CALLS_API_ROUTE]->(:ApiRoute {project_id:$pid})-[:HANDLED_BY]->(dst:File {project_id:$pid})
             WHERE src <> dst
             MERGE (src)-[r:FILE_GRAPH_LINK]->(dst)
             SET r.project_id = $pid,
                 r.last_seen_run = $run_id
             RETURN count(DISTINCT r) AS updated",
        )
        .param("pid", project_id.to_string())
        .param("run_id", run_id.to_string()),
        "updated",
    )
    .await?;

    Ok(total)
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

fn rel_count_map(rels: &[String], counts: &[i64]) -> Value {
    let mut obj = serde_json::Map::new();
    for (rel, count) in rels.iter().zip(counts.iter()) {
        obj.insert(rel.clone(), json!(count));
    }
    Value::Object(obj)
}

async fn project_file_graph(
    graph: &Arc<Graph>,
    graph_name: &str,
    project_id: &str,
    rels: &[String],
) -> Result<(bool, Vec<i64>), Box<dyn std::error::Error>> {
    let mut counts = Vec::with_capacity(rels.len());
    let mut sum = 0i64;
    for rel in rels {
        let count = count_rel(graph, project_id, rel).await?;
        counts.push(count);
        sum += count;
    }
    if sum < 2 {
        return Ok((false, counts));
    }
    let _ = run(
        graph,
        query("CALL gds.graph.drop($name, false) YIELD graphName").param("name", graph_name.to_string()),
    )
    .await;
    let mut projection_result = graph
        .execute(
            query(
                "MATCH (f:File {project_id: $pid})
                 OPTIONAL MATCH (f)-[r]-(g:File {project_id: $pid})
                 WHERE r IS NULL OR type(r) IN $rels
                 WITH gds.graph.project($name, f, g, {}, {undirectedRelationshipTypes: ['*']}) AS proj
                 RETURN proj.graphName AS graphName, proj.nodeCount AS nodes, proj.relationshipCount AS rels",
            )
            .param("name", graph_name.to_string())
            .param("pid", project_id.to_string())
            .param("rels", strings_to_bolt(rels)),
        )
        .await?;
    let mut created = false;
    while let Some(_row) = projection_result.next().await? {
        created = true;
        break;
    }
    if !created {
        return Err("gds.graph.project returned no rows".into());
    }
    let exists = one_bool(
        graph,
        query("CALL gds.graph.exists($name) YIELD exists RETURN exists").param("name", graph_name.to_string()),
        "exists",
    )
    .await?;
    if !exists {
        return Err(format!("projected graph `{graph_name}` missing from GDS catalog").into());
    }
    Ok((true, counts))
}

async fn drop_graph(graph: &Arc<Graph>, graph_name: &str) {
    let _ = run(
        graph,
        query("CALL gds.graph.drop($name, false) YIELD graphName").param("name", graph_name.to_string()),
    )
    .await;
}

async fn run_pagerank(graph: &Arc<Graph>, project_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let name = graph_name("calls", project_id);
    let _ = run(
        graph,
        query("CALL gds.graph.drop($name, false) YIELD graphName").param("name", name.clone()),
    )
    .await;
    let mut projection_result = graph
        .execute(
            query(
                "MATCH (n {project_id: $pid})
                 WHERE n:Function OR n:Class OR n:Struct OR n:Trait OR n:Enum
                 OPTIONAL MATCH (n)-[:CALLS]->(m {project_id: $pid})
                 WHERE m IS NULL OR m:Function OR m:Class OR m:Struct OR m:Trait OR m:Enum
                 WITH gds.graph.project($name, n, m) AS g
                 RETURN g.graphName AS graph, g.nodeCount AS nodes, g.relationshipCount AS rels",
            )
            .param("name", name.clone())
            .param("pid", project_id.to_string()),
        )
        .await?;
    let mut created = false;
    while let Some(_row) = projection_result.next().await? {
        created = true;
        break;
    }
    if !created {
        return Err("pagerank graph projection returned no rows".into());
    }
    let exists = one_bool(
        graph,
        query("CALL gds.graph.exists($name) YIELD exists RETURN exists").param("name", name.clone()),
        "exists",
    )
    .await?;
    if !exists {
        return Err(format!("projected pagerank graph `{name}` missing from GDS catalog").into());
    }
    run(
        graph,
        query(
            "CALL gds.pageRank.write($name, {
                writeProperty: 'pagerank',
                dampingFactor: 0.85,
                maxIterations: 20,
                tolerance: 0.0000001
            })",
        )
        .param("name", name.clone()),
    )
    .await?;
    run(
        graph,
        query(
            "MATCH (f:File {project_id: $pid})-[:CONTAINS]->(s)
             WHERE s:Function OR s:Class OR s:Struct OR s:Trait OR s:Enum
               AND s.pagerank IS NOT NULL
             WITH f, max(s.pagerank) AS top_pr, sum(s.pagerank) AS sum_pr
             SET f.pagerank = top_pr, f.pagerank_sum = sum_pr",
        )
        .param("pid", project_id.to_string()),
    )
    .await?;
    let updated = one_i64(
        graph,
        query("MATCH (f:File {project_id: $pid}) WHERE f.pagerank IS NOT NULL RETURN count(f) AS updated")
            .param("pid", project_id.to_string()),
        "updated",
    )
    .await?;
    drop_graph(graph, &name).await;
    Ok(updated)
}

async fn run_file_gds(
    graph: &Arc<Graph>,
    prefix: &str,
    project_id: &str,
    rels: &[String],
    write_cypher: &str,
    count_cypher: &str,
    count_key: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let name = graph_name(prefix, project_id);
    let file_count = count_file_nodes(graph, project_id).await.unwrap_or(0);
    let (projected, rel_counts) = project_file_graph(graph, &name, project_id, rels).await?;
    let total_rel_count: i64 = rel_counts.iter().sum();
    if !projected {
        return Ok(json!({
            "status": "skipped",
            "updated": 0,
            "file_nodes": file_count,
            "file_rel_total": total_rel_count,
            "file_rel_counts": rel_count_map(rels, &rel_counts),
            "reason": if total_rel_count < 2 { "insufficient_file_relationships" } else { "projection_skipped" },
        }));
    }
    let write_result = run(graph, query(write_cypher).param("name", name.clone())).await;
    if let Err(err) = write_result {
        drop_graph(graph, &name).await;
        return Ok(json!({
            "status": "failed",
            "updated": 0,
            "file_nodes": file_count,
            "file_rel_total": total_rel_count,
            "file_rel_counts": rel_count_map(rels, &rel_counts),
            "error": err.to_string(),
        }));
    }
    let updated = one_i64(
        graph,
        query(count_cypher).param("pid", project_id.to_string()),
        count_key,
    )
    .await?;
    drop_graph(graph, &name).await;
    Ok(json!({
        "status": "ok",
        "updated": updated,
        "file_nodes": file_count,
        "file_rel_total": total_rel_count,
        "file_rel_counts": rel_count_map(rels, &rel_counts),
    }))
}

async fn run_betweenness_gds(
    graph: &Arc<Graph>,
    project_id: &str,
    rels: &[String],
) -> Result<Value, Box<dyn std::error::Error>> {
    let enabled = env_bool("TS_PACK_GDS_ENABLE_BETWEENNESS", true);
    if !enabled {
        return Ok(json!({
            "status": "skipped",
            "updated": 0,
            "reason": "disabled_by_env",
        }));
    }

    let file_count = count_file_nodes(graph, project_id).await.unwrap_or(0);
    let max_file_nodes = env_i64("TS_PACK_GDS_BETWEENNESS_MAX_FILE_NODES", 2500);
    if file_count > max_file_nodes {
        return Ok(json!({
            "status": "skipped",
            "updated": 0,
            "file_nodes": file_count,
            "reason": "file_count_above_threshold",
            "threshold": max_file_nodes,
        }));
    }

    let sample_cap = env_usize("TS_PACK_GDS_BETWEENNESS_SAMPLING_CAP", 256);
    let sampling_size = usize::min(file_count.max(1) as usize, sample_cap);
    let max_estimated_bytes = env_i64("TS_PACK_GDS_BETWEENNESS_MAX_ESTIMATED_BYTES", 512 * 1024 * 1024);
    let concurrency = env_usize("TS_PACK_GDS_BETWEENNESS_CONCURRENCY", 1);

    let name = graph_name("betweenness", project_id);
    let (projected, rel_counts) = project_file_graph(graph, &name, project_id, rels).await?;
    let total_rel_count: i64 = rel_counts.iter().sum();
    if !projected {
        return Ok(json!({
            "status": "skipped",
            "updated": 0,
            "file_nodes": file_count,
            "file_rel_total": total_rel_count,
            "file_rel_counts": rel_count_map(rels, &rel_counts),
            "reason": if total_rel_count < 2 { "insufficient_file_relationships" } else { "projection_skipped" },
        }));
    }

    let estimated_bytes = estimate_bytes_max(
        graph,
        query(
            "CALL gds.betweenness.write.estimate($name, {
                writeProperty: 'betweenness',
                samplingSize: $samplingSize,
                concurrency: $concurrency
            })
            YIELD bytesMax
            RETURN bytesMax",
        )
        .param("name", name.clone())
        .param("samplingSize", sampling_size as i64)
        .param("concurrency", concurrency as i64),
    )
    .await?;

    if let Some(bytes_max) = estimated_bytes {
        if bytes_max > max_estimated_bytes {
            drop_graph(graph, &name).await;
            return Ok(json!({
                "status": "skipped",
                "updated": 0,
                "file_nodes": file_count,
                "file_rel_total": total_rel_count,
                "file_rel_counts": rel_count_map(rels, &rel_counts),
                "reason": "estimate_exceeds_threshold",
                "estimated_bytes_max": bytes_max,
                "threshold_bytes_max": max_estimated_bytes,
                "sampling_size": sampling_size,
            }));
        }
    }

    let write_result = run(
        graph,
        query(
            "CALL gds.betweenness.write($name, {
                writeProperty: 'betweenness',
                samplingSize: $samplingSize,
                concurrency: $concurrency
            })",
        )
        .param("name", name.clone())
        .param("samplingSize", sampling_size as i64)
        .param("concurrency", concurrency as i64),
    )
    .await;
    if let Err(err) = write_result {
        drop_graph(graph, &name).await;
        return Ok(json!({
            "status": "failed",
            "updated": 0,
            "file_nodes": file_count,
            "file_rel_total": total_rel_count,
            "file_rel_counts": rel_count_map(rels, &rel_counts),
            "sampling_size": sampling_size,
            "error": err.to_string(),
        }));
    }

    let updated = one_i64(
        graph,
        query("MATCH (f:File {project_id: $pid}) WHERE f.betweenness IS NOT NULL RETURN count(f) AS updated")
            .param("pid", project_id.to_string()),
        "updated",
    )
    .await?;
    drop_graph(graph, &name).await;
    Ok(json!({
        "status": "ok",
        "updated": updated,
        "file_nodes": file_count,
        "file_rel_total": total_rel_count,
        "file_rel_counts": rel_count_map(rels, &rel_counts),
        "estimated_bytes_max": estimated_bytes,
        "sampling_size": sampling_size,
    }))
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
    let file_call_edges = build_file_calls_from_symbol_graph(&graph, project_id, &current_run_id).await?;
    let file_graph_links = build_file_graph_links(&graph, project_id, &current_run_id).await?;
    let rels = vec!["FILE_GRAPH_LINK".to_string()];
    let pagerank = run_pagerank(&graph, project_id).await.unwrap_or(0);
    let louvain = run_file_gds(
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
    let betweenness = run_betweenness_gds(&graph, project_id, &rels)
        .await
        .unwrap_or_else(|err| json!({"status": "failed", "updated": 0, "error": err.to_string()}));
    let wcc_graph = graph_name("wcc", project_id);
    let isolated = match project_file_graph(&graph, &wcc_graph, project_id, &rels).await {
        Ok((true, rel_counts)) => {
            let total_rel_count: i64 = rel_counts.iter().sum();
            let file_count = count_file_nodes(&graph, project_id).await.unwrap_or(0);
            let write_result = run(
                &graph,
                query("CALL gds.wcc.write($name, { writeProperty: 'wccComponent' })").param("name", wcc_graph.clone()),
            )
            .await;
            if let Err(err) = write_result {
                drop_graph(&graph, &wcc_graph).await;
                return Ok(json!({
                    "manifest_added": added_manifest,
                    "parsed_marked": parsed_marked,
                    "file_path_aliased": aliased,
                    "file_call_edges": file_call_edges,
                    "file_graph_links": file_graph_links,
                    "swift_enrichment": swift_enrichment,
                    "pagerank": pagerank,
                    "louvain": louvain,
                    "betweenness": betweenness,
                    "isolated": {
                        "status": "failed",
                        "updated": 0,
                        "file_nodes": file_count,
                        "file_rel_total": total_rel_count,
                        "file_rel_counts": rel_count_map(&rels, &rel_counts),
                        "error": err.to_string(),
                    },
                }));
            }
            let mark_result = run(
                &graph,
                query(
                    "MATCH (f:File {project_id: $pid})
                     WHERE f.wccComponent IS NOT NULL
                     WITH f.wccComponent AS comp, collect(f) AS members
                     WITH members, size(members) AS sz
                     FOREACH (f IN members | SET f.isolated = (sz = 1))",
                )
                .param("pid", project_id.to_string()),
            )
            .await;
            if let Err(err) = mark_result {
                drop_graph(&graph, &wcc_graph).await;
                return Ok(json!({
                    "manifest_added": added_manifest,
                    "parsed_marked": parsed_marked,
                    "file_path_aliased": aliased,
                    "file_call_edges": file_call_edges,
                    "file_graph_links": file_graph_links,
                    "swift_enrichment": swift_enrichment,
                    "pagerank": pagerank,
                    "louvain": louvain,
                    "betweenness": betweenness,
                    "isolated": {
                        "status": "failed",
                        "updated": 0,
                        "file_nodes": file_count,
                        "file_rel_total": total_rel_count,
                        "file_rel_counts": rel_count_map(&rels, &rel_counts),
                        "error": err.to_string(),
                    },
                }));
            }
            let count = one_i64(
                &graph,
                query("MATCH (f:File {project_id: $pid}) WHERE f.isolated = true RETURN count(f) AS isolated")
                    .param("pid", project_id.to_string()),
                "isolated",
            )
            .await
            .unwrap_or(0);
            drop_graph(&graph, &wcc_graph).await;
            json!({
                "status": "ok",
                "updated": count,
                "file_nodes": file_count,
                "file_rel_total": total_rel_count,
                "file_rel_counts": rel_count_map(&rels, &rel_counts),
            })
        }
        Ok((false, rel_counts)) => {
            let total_rel_count: i64 = rel_counts.iter().sum();
            let file_count = count_file_nodes(&graph, project_id).await.unwrap_or(0);
            json!({
                "status": "skipped",
                "updated": 0,
                "file_nodes": file_count,
                "file_rel_total": total_rel_count,
                "file_rel_counts": rel_count_map(&rels, &rel_counts),
                "reason": if total_rel_count < 2 { "insufficient_file_relationships" } else { "projection_skipped" },
            })
        }
        Err(err) => json!({
            "status": "failed",
            "updated": 0,
            "error": err.to_string(),
        }),
    };

    Ok(json!({
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
    }))
}
