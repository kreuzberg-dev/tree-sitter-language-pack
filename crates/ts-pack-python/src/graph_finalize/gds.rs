use neo4rs::{query, Graph};
use serde_json::{json, Value};
use std::sync::Arc;

use super::reporting::rel_count_map;
use super::{
    count_file_nodes, count_rel, drop_graph, env_bool, env_i64, env_usize, estimate_bytes_max, graph_name, one_bool,
    one_i64, run,
};

pub(super) async fn project_file_graph(
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
            .param("rels", super::strings_to_bolt(rels)),
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

pub(super) async fn run_pagerank(graph: &Arc<Graph>, project_id: &str) -> Result<i64, Box<dyn std::error::Error>> {
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

pub(super) async fn run_file_gds(
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

pub(super) async fn run_betweenness_gds(
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

pub(super) async fn run_isolated_file_gds(graph: &Arc<Graph>, project_id: &str, rels: &[String]) -> Value {
    let wcc_graph = graph_name("wcc", project_id);
    match project_file_graph(graph, &wcc_graph, project_id, rels).await {
        Ok((true, rel_counts)) => {
            let total_rel_count: i64 = rel_counts.iter().sum();
            let file_count = count_file_nodes(graph, project_id).await.unwrap_or(0);
            let write_result = run(
                graph,
                query("CALL gds.wcc.write($name, { writeProperty: 'wccComponent' })").param("name", wcc_graph.clone()),
            )
            .await;
            if let Err(err) = write_result {
                drop_graph(graph, &wcc_graph).await;
                return json!({
                    "status": "failed",
                    "updated": 0,
                    "file_nodes": file_count,
                    "file_rel_total": total_rel_count,
                    "file_rel_counts": rel_count_map(rels, &rel_counts),
                    "error": err.to_string(),
                });
            }
            let mark_result = run(
                graph,
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
                drop_graph(graph, &wcc_graph).await;
                return json!({
                    "status": "failed",
                    "updated": 0,
                    "file_nodes": file_count,
                    "file_rel_total": total_rel_count,
                    "file_rel_counts": rel_count_map(rels, &rel_counts),
                    "error": err.to_string(),
                });
            }
            let count = one_i64(
                graph,
                query("MATCH (f:File {project_id: $pid}) WHERE f.isolated = true RETURN count(f) AS isolated")
                    .param("pid", project_id.to_string()),
                "isolated",
            )
            .await
            .unwrap_or(0);
            drop_graph(graph, &wcc_graph).await;
            json!({
                "status": "ok",
                "updated": count,
                "file_nodes": file_count,
                "file_rel_total": total_rel_count,
                "file_rel_counts": rel_count_map(rels, &rel_counts),
            })
        }
        Ok((false, rel_counts)) => {
            let total_rel_count: i64 = rel_counts.iter().sum();
            let file_count = count_file_nodes(graph, project_id).await.unwrap_or(0);
            json!({
                "status": "skipped",
                "updated": 0,
                "file_nodes": file_count,
                "file_rel_total": total_rel_count,
                "file_rel_counts": rel_count_map(rels, &rel_counts),
                "reason": if total_rel_count < 2 { "insufficient_file_relationships" } else { "projection_skipped" },
            })
        }
        Err(err) => json!({
            "status": "failed",
            "updated": 0,
            "error": err.to_string(),
        }),
    }
}
