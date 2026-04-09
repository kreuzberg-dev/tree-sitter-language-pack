use neo4rs::{BoltType, ConfigBuilder, Graph, Query, query};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::swift_semantic;

fn graph_name(prefix: &str, project_id: &str) -> String {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{prefix}-{project_id}-{stamp:x}")
}

fn strings_to_bolt(items: &[String]) -> BoltType {
    BoltType::from(items.iter().cloned().map(BoltType::from).collect::<Vec<_>>())
}

async fn run(graph: &Arc<Graph>, q: Query) -> Result<(), Box<dyn std::error::Error>> {
    graph.run(q).await?;
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

async fn ensure_manifest_file_nodes(
    graph: &Arc<Graph>,
    project_id: &str,
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
                    f.project_id = $pid,
                    f.filepath = row.filepath,
                    f.file_path = row.filepath,
                    f.name = row.name,
                    f.indexed_at = timestamp(),
                    f.parsed = false
                 SET f:Node, f:File, f.file_path = row.filepath",
            )
            .param("pid", project_id.to_string())
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
             SET f:Node",
        )
        .param("pid", project_id.to_string())
        .param("paths", strings_to_bolt(&manifest_paths)),
    )
    .await?;

    Ok(added)
}

async fn mark_manifest_parsed(
    graph: &Arc<Graph>,
    project_id: &str,
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
             SET f.parsed = true
             RETURN count(f) AS updated",
        )
        .param("pid", project_id.to_string())
        .param("paths", strings_to_bolt(parsed_paths)),
        "updated",
    )
    .await
}

async fn sync_file_path_alias(graph: &Arc<Graph>, project_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
    one_i64(
        graph,
        query(
            "MATCH (f:File {project_id:$pid})
             WHERE f.filepath IS NOT NULL
             SET f.file_path = f.filepath
             RETURN count(f) AS updated",
        )
        .param("pid", project_id.to_string()),
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
    run(
        graph,
        query(
            "MATCH (f:File {project_id: $pid})
             OPTIONAL MATCH (f)-[r]-(g:File {project_id: $pid})
             WHERE type(r) IN $rels
             WITH gds.graph.project($name, f, g, {}, {undirectedRelationshipTypes: ['*']}) AS proj
             RETURN proj.nodeCount AS nodes",
        )
        .param("name", graph_name.to_string())
        .param("pid", project_id.to_string())
        .param("rels", strings_to_bolt(rels)),
    )
    .await?;
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
    run(
        graph,
        query(
            "MATCH (n {project_id: $pid})
             WHERE n:Function OR n:Class OR n:Struct OR n:Trait OR n:Enum
             OPTIONAL MATCH (n)-[:CALLS]->(m {project_id: $pid})
             WHERE m:Function OR m:Class OR m:Struct OR m:Trait OR m:Enum
             WITH gds.graph.project($name, n, m) AS g
             RETURN g.graphName AS graph, g.nodeCount AS nodes, g.relationshipCount AS rels",
        )
        .param("name", name.clone())
        .param("pid", project_id.to_string()),
    )
    .await?;
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

pub async fn finalize_struct_graph_async(
    project_path: &str,
    project_id: &str,
    manifest_file: &str,
    indexed_files: &[String],
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    neo4j_db: &str,
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

    let added_manifest = ensure_manifest_file_nodes(&graph, project_id, manifest_file).await?;
    let root = fs::canonicalize(project_path).unwrap_or_else(|_| Path::new(project_path).to_path_buf());
    let parsed_paths = indexed_files
        .iter()
        .filter_map(|path| fs::canonicalize(path).ok())
        .filter_map(|path| path.strip_prefix(&root).ok().map(|p| p.to_path_buf()))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    let parsed_marked = mark_manifest_parsed(&graph, project_id, &parsed_paths).await?;
    let aliased = sync_file_path_alias(&graph, project_id).await?;
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
    let rels = vec![
        "CALLS".to_string(),
        "IMPORTS".to_string(),
        "ASSET_LINKS".to_string(),
        "CALLS_API".to_string(),
        "CALLS_SERVICE".to_string(),
        "CALLS_DB".to_string(),
    ];
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
    let betweenness = run_file_gds(
        &graph,
        "betweenness",
        project_id,
        &rels,
        "CALL gds.betweenness.write($name, { writeProperty: 'betweenness', samplingSize: null })",
        "MATCH (f:File {project_id: $pid}) WHERE f.betweenness IS NOT NULL RETURN count(f) AS updated",
        "updated",
    )
    .await
    .unwrap_or_else(|err| json!({"status": "failed", "updated": 0, "error": err.to_string()}));
    let wcc_graph = graph_name("wcc", project_id);
    let isolated = match project_file_graph(&graph, &wcc_graph, project_id, &rels).await {
        Ok((true, rel_counts)) => {
            let total_rel_count: i64 = rel_counts.iter().sum();
            let file_count = count_file_nodes(&graph, project_id).await.unwrap_or(0);
            let write_result = run(
                &graph,
                query("CALL gds.wcc.write($name, { writeProperty: 'wccComponent' })")
                    .param("name", wcc_graph.clone()),
            )
            .await;
            if let Err(err) = write_result {
                drop_graph(&graph, &wcc_graph).await;
                return Ok(json!({
                    "manifest_added": added_manifest,
                    "parsed_marked": parsed_marked,
                    "file_path_aliased": aliased,
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
        "swift_enrichment": swift_enrichment,
        "pagerank": pagerank,
        "louvain": louvain,
        "betweenness": betweenness,
        "isolated": isolated,
    }))
}
