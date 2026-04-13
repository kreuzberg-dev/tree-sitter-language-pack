use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use futures::{TryStreamExt, stream};
use neo4rs::Graph;

use crate::writers;
use crate::{
    CloneCandidate, CloneCanonRow, CloneGroupRow, CloneMemberRow, FileCloneCanonRow, FileCloneGroupRow,
    FileCloneMemberRow,
};

pub(crate) struct CloneConfig {
    pub min_overlap: f64,
    pub token_sim_threshold: f64,
    pub kgram_sim_threshold: f64,
    pub min_score: f64,
    pub bucket_limit: usize,
    pub fallback_hashes: usize,
    pub force_all_hashes_max_fps: usize,
}

pub(crate) fn stable_hash_hex(input: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    for b in input.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    format!("{:016x}", h)
}

fn ok_chunks<'a, T>(
    items: &'a [T],
    chunk_size: usize,
) -> impl futures::stream::Stream<Item = Result<&'a [T], neo4rs::Error>> + 'a {
    stream::iter(items.chunks(chunk_size).map(Ok::<_, neo4rs::Error>))
}

pub(crate) async fn write_clone_enrichment(
    graph: &Arc<Graph>,
    project_id: &str,
    run_id: &str,
    clone_candidates: &[CloneCandidate],
    rel_batch_size: usize,
    rel_concurrency: usize,
    cfg: &CloneConfig,
) -> neo4rs::Result<()> {
    if clone_candidates.is_empty() {
        return Ok(());
    }

    let mut fp_counts: Vec<HashMap<u64, usize>> = vec![HashMap::new(); 3];
    for cand in clone_candidates {
        for (scale_idx, fps) in cand.fingerprints.iter().enumerate() {
            for fp in fps {
                *fp_counts[scale_idx].entry(*fp).or_insert(0) += 1;
            }
        }
    }

    let mut fp_index_selected: Vec<HashMap<u64, Vec<usize>>> = vec![HashMap::new(); 3];
    for (idx, cand) in clone_candidates.iter().enumerate() {
        for (scale_idx, fps) in cand.fingerprints.iter().enumerate() {
            if fps.is_empty() {
                continue;
            }
            let mut filtered: Vec<u64> = if fps.len() <= cfg.force_all_hashes_max_fps {
                fps.iter().copied().collect()
            } else {
                fps.iter()
                    .filter(|h| fp_counts[scale_idx].get(h).copied().unwrap_or(0) <= cfg.bucket_limit)
                    .copied()
                    .collect()
            };
            if filtered.is_empty() && cfg.fallback_hashes > 0 {
                let mut sorted: Vec<u64> = fps.iter().copied().collect();
                sorted.sort();
                filtered = sorted.into_iter().take(cfg.fallback_hashes).collect();
            }
            for fp in filtered {
                fp_index_selected[scale_idx].entry(fp).or_default().push(idx);
            }
        }
    }

    let mut pair_infos: HashMap<(usize, usize), [usize; 3]> = HashMap::new();
    for (scale_idx, index) in fp_index_selected.iter().enumerate() {
        for ids in index.values() {
            if ids.len() < 2 {
                continue;
            }
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    let a = ids[i];
                    let b = ids[j];
                    let key = if a < b { (a, b) } else { (b, a) };
                    let entry = pair_infos.entry(key).or_insert([0usize; 3]);
                    entry[scale_idx] += 1;
                }
            }
        }
    }

    let mut kgram_index: HashMap<u64, Vec<usize>> = HashMap::new();
    for (idx, cand) in clone_candidates.iter().enumerate() {
        if cand.kgrams.is_empty() {
            continue;
        }
        for gram in &cand.kgrams {
            kgram_index.entry(*gram).or_default().push(idx);
        }
    }
    let mut kgram_pairs: HashSet<(usize, usize)> = HashSet::new();
    for ids in kgram_index.values() {
        if ids.len() < 2 {
            continue;
        }
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = ids[i];
                let b = ids[j];
                let key = if a < b { (a, b) } else { (b, a) };
                kgram_pairs.insert(key);
                pair_infos.entry(key).or_insert([0usize; 3]);
            }
        }
    }

    let mut parent: Vec<usize> = (0..clone_candidates.len()).collect();
    let find = |parent: &mut Vec<usize>, x: usize| -> usize {
        let mut x = x;
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };
    let union = |parent: &mut Vec<usize>, a: usize, b: usize| {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent[rb] = ra;
        }
    };

    for ((a, b), shared_counts) in pair_infos {
        let cand_a = &clone_candidates[a];
        let cand_b = &clone_candidates[b];
        let mut max_overlap = 0.0;
        for (scale_idx, shared) in shared_counts.iter().enumerate() {
            let fa = &cand_a.fingerprints[scale_idx];
            let fb = &cand_b.fingerprints[scale_idx];
            let min_den = fa.len().min(fb.len());
            if min_den == 0 || *shared == 0 {
                continue;
            }
            let overlap = *shared as f64 / min_den as f64;
            if overlap > max_overlap {
                max_overlap = overlap;
            }
        }

        let ta = &cand_a.token_set;
        let tb = &cand_b.token_set;
        let token_jaccard = if ta.is_empty() || tb.is_empty() {
            0.0
        } else {
            let inter = ta.intersection(tb).count();
            let uni = ta.union(tb).count();
            inter as f64 / uni as f64
        };

        let kgram_jaccard = if kgram_pairs.contains(&(a, b)) {
            let ka = &cand_a.kgrams;
            let kb = &cand_b.kgrams;
            if ka.is_empty() || kb.is_empty() {
                0.0
            } else {
                let inter = ka.intersection(kb).count();
                let uni = ka.union(kb).count();
                inter as f64 / uni as f64
            }
        } else {
            0.0
        };

        if max_overlap < cfg.min_overlap
            && token_jaccard < cfg.token_sim_threshold
            && kgram_jaccard < cfg.kgram_sim_threshold
        {
            continue;
        }
        let score = max_overlap.max(token_jaccard).max(kgram_jaccard);
        if score >= cfg.min_score {
            union(&mut parent, a, b);
        }
    }

    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..clone_candidates.len() {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push(i);
    }

    let mut clone_group_rows = Vec::new();
    let mut clone_member_rows = Vec::new();
    let mut clone_canon_rows = Vec::new();
    let mut file_group_map: HashMap<String, Vec<String>> = HashMap::new();

    for members in groups.values() {
        if members.len() < 2 {
            continue;
        }
        let mut ids: Vec<String> = members
            .iter()
            .map(|idx| clone_candidates[*idx].symbol_id.clone())
            .collect();
        ids.sort();
        let gid = stable_hash_hex(&ids.join("|"));
        let mut canon = members[0];
        for idx in members {
            let cand = &clone_candidates[*idx];
            let canon_cand = &clone_candidates[canon];
            if cand.span_len > canon_cand.span_len
                || (cand.span_len == canon_cand.span_len && cand.symbol_id < canon_cand.symbol_id)
            {
                canon = *idx;
            }
        }
        clone_group_rows.push(CloneGroupRow {
            id: gid.clone(),
            project_id: project_id.to_string(),
            size: members.len(),
            method: "winnow+tokens".to_string(),
            score_min: cfg.min_score,
            score_max: 1.0,
            score_avg: cfg.min_score,
        });
        for idx in members {
            let cand = &clone_candidates[*idx];
            clone_member_rows.push(CloneMemberRow {
                gid: gid.clone(),
                sid: cand.symbol_id.clone(),
            });
            file_group_map
                .entry(cand.filepath.clone())
                .or_default()
                .push(gid.clone());
        }
        clone_canon_rows.push(CloneCanonRow {
            gid: gid.clone(),
            sid: clone_candidates[canon].symbol_id.clone(),
        });
    }

    let mut file_group_rows = Vec::new();
    let mut file_member_rows = Vec::new();
    let mut file_canon_rows = Vec::new();
    let mut file_groups: HashMap<String, Vec<String>> = HashMap::new();
    for (fp, gids) in &mut file_group_map {
        gids.sort();
        gids.dedup();
        if gids.is_empty() {
            continue;
        }
        let fgid = stable_hash_hex(&gids.join("|"));
        file_groups.entry(fgid).or_default().push(fp.clone());
    }
    for (fgid, files) in file_groups {
        if files.len() < 2 {
            continue;
        }
        let mut files_sorted = files.clone();
        files_sorted.sort();
        file_group_rows.push(FileCloneGroupRow {
            id: fgid.clone(),
            project_id: project_id.to_string(),
            size: files_sorted.len(),
            method: "function-overlap".to_string(),
            score_min: cfg.min_score,
            score_max: 1.0,
            score_avg: cfg.min_score,
        });
        let canon = files_sorted[0].clone();
        for fp in &files_sorted {
            file_member_rows.push(FileCloneMemberRow {
                gid: fgid.clone(),
                filepath: fp.clone(),
                project_id: project_id.to_string(),
            });
        }
        file_canon_rows.push(FileCloneCanonRow {
            gid: fgid.clone(),
            filepath: canon,
            project_id: project_id.to_string(),
        });
    }

    let clone_write_concurrency = 1usize.min(rel_concurrency.max(1));
    clone_group_rows.sort_by(|a, b| a.id.cmp(&b.id));
    clone_member_rows.sort_by(|a, b| a.gid.cmp(&b.gid).then_with(|| a.sid.cmp(&b.sid)));
    clone_canon_rows.sort_by(|a, b| a.gid.cmp(&b.gid).then_with(|| a.sid.cmp(&b.sid)));
    file_group_rows.sort_by(|a, b| a.id.cmp(&b.id));
    file_member_rows.sort_by(|a, b| a.gid.cmp(&b.gid).then_with(|| a.filepath.cmp(&b.filepath)));
    file_canon_rows.sort_by(|a, b| a.gid.cmp(&b.gid).then_with(|| a.filepath.cmp(&b.filepath)));

    let t_clone = Instant::now();
    if !clone_group_rows.is_empty() {
        ok_chunks(&clone_group_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_clone_groups(&g, chunk, &run_id).await }
            })
            .await?;
        ok_chunks(&clone_member_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_clone_members(&g, chunk, &run_id).await }
            })
            .await?;
        ok_chunks(&clone_canon_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_clone_canon(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !file_group_rows.is_empty() {
        ok_chunks(&file_group_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_file_clone_groups(&g, chunk, &run_id).await }
            })
            .await?;
        ok_chunks(&file_member_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_file_clone_members(&g, chunk, &run_id).await }
            })
            .await?;
        ok_chunks(&file_canon_rows, rel_batch_size)
            .try_for_each_concurrent(clone_write_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.to_string();
                async move { writers::write_file_clone_canon(&g, chunk, &run_id).await }
            })
            .await?;
    }
    eprintln!(
        "[ts-pack-index] CLONE writes done in {:.2}s (symbol_groups={}, file_groups={})",
        t_clone.elapsed().as_secs_f64(),
        clone_group_rows.len(),
        file_group_rows.len(),
    );
    Ok(())
}
