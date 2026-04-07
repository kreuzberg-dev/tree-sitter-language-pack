use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use futures::{stream, StreamExt};
use neo4rs::{Graph, Query};

use crate::{
    CloneCandidate, CloneCanonRow, CloneGroupRow, CloneMemberRow, FileCloneCanonRow,
    FileCloneGroupRow, FileCloneMemberRow,
};
use crate::writers;

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

pub(crate) fn tokenize_normalized(source: &[u8]) -> Vec<u64> {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < source.len() {
        let b = source[i];
        if (b as char).is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if (b as char).is_ascii_alphabetic() || b == b'_' {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_alphanumeric() || c == b'_' {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<id>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }
        if (b as char).is_ascii_digit() {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_digit() {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<num>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }

        let punct = match b {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'.' | b':' | b'+'
            | b'-' | b'*' | b'/' | b'%' | b'<' | b'>' | b'=' => Some(b),
            _ => None,
        };
        if let Some(p) = punct {
            let mut h = FNV_OFFSET;
            h ^= p as u64;
            h = h.wrapping_mul(FNV_PRIME);
            tokens.push(h);
            i += 1;
            continue;
        }

        i += 1;
    }
    tokens
}

pub(crate) fn winnow_fingerprints(tokens: &[u64], k: usize, window: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hashes = Vec::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        hashes.push(h);
    }
    if hashes.is_empty() {
        return HashSet::new();
    }
    if hashes.len() <= window {
        return [*hashes.iter().min().unwrap()].into_iter().collect();
    }
    let mut fps = HashSet::new();
    for i in 0..=hashes.len() - window {
        let mut min = hashes[i];
        for value in hashes.iter().skip(i).take(window) {
            if *value < min {
                min = *value;
            }
        }
        fps.insert(min);
    }
    fps
}

pub(crate) fn kgram_hashes(tokens: &[u64], k: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut out = HashSet::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        out.insert(h);
    }
    out
}

pub(crate) async fn write_clone_enrichment(
    graph: &Arc<Graph>,
    project_id: &str,
    clone_candidates: &[CloneCandidate],
    rel_batch_size: usize,
    rel_concurrency: usize,
    cfg: &CloneConfig,
) {
    if clone_candidates.is_empty() {
        return;
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

    let _ = graph
        .run(
            Query::new("MATCH (g:CloneGroup {project_id:$pid}) DETACH DELETE g".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;
    let _ = graph
        .run(
            Query::new("MATCH (g:FileCloneGroup {project_id:$pid}) DETACH DELETE g".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;

    let t_clone = Instant::now();
    if !clone_group_rows.is_empty() {
        stream::iter(clone_group_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_clone_groups(&g, chunk).await }
            })
            .await;
        stream::iter(clone_member_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_clone_members(&g, chunk).await }
            })
            .await;
        stream::iter(clone_canon_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_clone_canon(&g, chunk).await }
            })
            .await;
    }
    if !file_group_rows.is_empty() {
        stream::iter(file_group_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_file_clone_groups(&g, chunk).await }
            })
            .await;
        stream::iter(file_member_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_file_clone_members(&g, chunk).await }
            })
            .await;
        stream::iter(file_canon_rows.chunks(rel_batch_size))
            .for_each_concurrent(rel_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_file_clone_canon(&g, chunk).await }
            })
            .await;
    }
    eprintln!(
        "[ts-pack-index] CLONE writes done in {:.2}s (symbol_groups={}, file_groups={})",
        t_clone.elapsed().as_secs_f64(),
        clone_group_rows.len(),
        file_group_rows.len(),
    );
}
