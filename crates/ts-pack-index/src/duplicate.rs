use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    WINNOW_FALLBACK_HASHES, WINNOW_FORCE_ALL_HASHES_MAX_FPS, WINNOW_KGRAM_SIM_THRESHOLD, WINNOW_LARGE_K,
    WINNOW_LARGE_W, WINNOW_MEDIUM_K, WINNOW_MEDIUM_W, WINNOW_MIN_FINGERPRINTS, WINNOW_MIN_OVERLAP, WINNOW_MIN_SCORE,
    WINNOW_MIN_TOKENS, WINNOW_SMALL_K, WINNOW_SMALL_TOKEN_THRESHOLD, WINNOW_SMALL_W, WINNOW_TOKEN_SIM_THRESHOLD,
};

#[derive(Clone, Copy, Debug)]
pub struct DuplicateCollapseConfig {
    pub min_tokens: usize,
    pub min_fingerprints: usize,
    pub min_overlap: f64,
    pub token_sim_threshold: f64,
    pub kgram_sim_threshold: f64,
    pub min_score: f64,
    pub fallback_hashes: usize,
    pub force_all_hashes_max_fps: usize,
    pub small_token_threshold: usize,
    pub small_k: usize,
    pub small_w: usize,
    pub medium_k: usize,
    pub medium_w: usize,
    pub large_k: usize,
    pub large_w: usize,
    pub normalize_numbers: bool,
    pub min_literal_token_jaccard: f64,
    pub min_literal_kgram_jaccard: f64,
    pub min_length_ratio: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct DuplicatePairSummary {
    pub left: usize,
    pub right: usize,
    pub relation: String,
    pub score: f64,
    pub duplicate: bool,
    pub exact_match: bool,
    pub normalized_match: bool,
    pub simhash_hamming: Option<u32>,
    pub length_ratio: f64,
    pub fingerprint_overlap: f64,
    pub token_jaccard: f64,
    pub literal_token_jaccard: f64,
    pub literal_kgram_jaccard: f64,
    pub kgram_jaccard: f64,
    pub structure_overlap: f64,
    pub symbol_overlap: f64,
    pub role_match: f64,
    pub boilerplate_score: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct DuplicateSelection {
    pub keep_indices: Vec<usize>,
    pub suppressed_indices: Vec<usize>,
    pub pairs: Vec<DuplicatePairSummary>,
    pub groups: Vec<DuplicateGroup>,
    pub mode: &'static str,
    pub suppression_policy: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct DuplicateGroup {
    pub group_id: usize,
    pub members: Vec<usize>,
    pub canonical_candidates: Vec<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DiverseSelection {
    pub mode: &'static str,
    pub keep_indices: Vec<usize>,
    pub suppressed_indices: Vec<usize>,
    pub exact_suppressed_indices: Vec<usize>,
    pub group_order: Vec<usize>,
    pub representative_indices: Vec<usize>,
    pub mmr_lambda: f64,
    pub aspect_lambda: f64,
    pub group_diversity_lambda: f64,
    pub selected_aspects: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ExperimentConfig {
    pub boilerplate_variant_suppression: bool,
    pub canonical_docs_mirror_suppression: bool,
    pub helper_clone_suppression: bool,
    pub threshold_struct: Option<f64>,
    pub threshold_lexical: Option<f64>,
    pub threshold_role: Option<f64>,
    pub min_length_ratio: Option<f64>,
    pub max_length_ratio: Option<f64>,
    pub threshold_query_distinction: Option<f64>,
    pub allow_cross_role_suppression: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct CandidateTrace {
    pub idx: usize,
    pub group_id: Option<usize>,
    pub base_relevance: f64,
    pub normalized_relevance: f64,
    pub final_score: f64,
    pub duplicate_relations: Vec<String>,
    pub redundancy_penalty: f64,
    pub aspect_coverage_gain: f64,
    pub group_diversity_bonus: f64,
    pub role_bonus: f64,
    pub source_bonus: f64,
    pub query_distinction_score: f64,
    pub representative_reason: String,
    pub decision_reason: String,
    pub kept: bool,
    pub exact_suppressed: bool,
    pub experimental_suppressed: bool,
    pub beaten_by: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DuplicateTelemetry {
    pub mode: &'static str,
    pub query_class: String,
    pub exact_suppressions: usize,
    pub experimental_suppressions: usize,
    pub relation_counts: BTreeMap<String, usize>,
    pub group_sizes: Vec<usize>,
    pub representative_selection_reasons: BTreeMap<String, usize>,
    pub topk_redundancy_before: f64,
    pub topk_redundancy_after: f64,
    pub kept_group_multi_member_count: usize,
    pub multi_representative_group_count: usize,
    pub query_distinct_multi_rep_count: usize,
    pub canonical_doc_preference_success: Option<bool>,
    pub version_sensitive_query: bool,
    pub best_answer_loss_suspect: bool,
    pub regression_alerts: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DiverseSelectionTrace {
    pub selection: DiverseSelection,
    pub candidates: Vec<CandidateTrace>,
    pub telemetry: DuplicateTelemetry,
    pub suppression_policy: &'static str,
    pub experiments: ExperimentConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CandidateContext {
    pub file_path: String,
    pub metadata: serde_json::Value,
}

impl Default for DuplicateCollapseConfig {
    fn default() -> Self {
        Self {
            min_tokens: WINNOW_MIN_TOKENS,
            min_fingerprints: WINNOW_MIN_FINGERPRINTS,
            min_overlap: WINNOW_MIN_OVERLAP,
            token_sim_threshold: WINNOW_TOKEN_SIM_THRESHOLD,
            kgram_sim_threshold: WINNOW_KGRAM_SIM_THRESHOLD,
            min_score: WINNOW_MIN_SCORE,
            fallback_hashes: WINNOW_FALLBACK_HASHES,
            force_all_hashes_max_fps: WINNOW_FORCE_ALL_HASHES_MAX_FPS,
            small_token_threshold: WINNOW_SMALL_TOKEN_THRESHOLD,
            small_k: WINNOW_SMALL_K,
            small_w: WINNOW_SMALL_W,
            medium_k: WINNOW_MEDIUM_K,
            medium_w: WINNOW_MEDIUM_W,
            large_k: WINNOW_LARGE_K,
            large_w: WINNOW_LARGE_W,
            normalize_numbers: true,
            min_literal_token_jaccard: 0.0,
            min_literal_kgram_jaccard: 0.0,
            min_length_ratio: 0.0,
        }
    }
}

impl DuplicateCollapseConfig {
    pub fn for_search_results() -> Self {
        Self {
            min_tokens: 12,
            min_fingerprints: 4,
            min_overlap: 0.5,
            token_sim_threshold: 0.55,
            kgram_sim_threshold: 0.6,
            min_score: 0.75,
            fallback_hashes: WINNOW_FALLBACK_HASHES,
            force_all_hashes_max_fps: WINNOW_FORCE_ALL_HASHES_MAX_FPS,
            small_token_threshold: WINNOW_SMALL_TOKEN_THRESHOLD,
            small_k: WINNOW_SMALL_K,
            small_w: WINNOW_SMALL_W,
            medium_k: WINNOW_MEDIUM_K,
            medium_w: WINNOW_MEDIUM_W,
            large_k: WINNOW_LARGE_K,
            large_w: WINNOW_LARGE_W,
            normalize_numbers: true,
            min_literal_token_jaccard: 0.7,
            min_literal_kgram_jaccard: 0.2,
            min_length_ratio: 0.7,
        }
    }
}

impl SuppressionConfig {
    fn from_experiments(experiments: &ExperimentConfig) -> Self {
        Self {
            threshold_struct: experiments.threshold_struct.unwrap_or(0.85),
            threshold_lexical: experiments.threshold_lexical.unwrap_or(0.72),
            threshold_role: experiments.threshold_role.unwrap_or(1.0),
            min_length_ratio: experiments.min_length_ratio.unwrap_or(0.8),
            max_length_ratio: experiments.max_length_ratio.unwrap_or(1.25),
            threshold_query_distinction: experiments.threshold_query_distinction.unwrap_or(0.22),
            allow_cross_role_suppression: experiments.allow_cross_role_suppression,
        }
    }
}

struct DuplicateSignature {
    token_count: usize,
    exact_hash: u64,
    normalized_hash: u64,
    simhash: u64,
    token_set: HashSet<u64>,
    literal_token_set: HashSet<u64>,
    literal_kgrams: HashSet<u64>,
    fingerprints: [HashSet<u64>; 3],
    kgrams: HashSet<u64>,
}

#[derive(Clone, Copy, Debug)]
struct DuplicateMetrics {
    score: f64,
    duplicate: bool,
    exact_match: bool,
    normalized_match: bool,
    simhash_hamming: Option<u32>,
    length_ratio: f64,
    fingerprint_overlap: f64,
    token_jaccard: f64,
    literal_token_jaccard: f64,
    literal_kgram_jaccard: f64,
    kgram_jaccard: f64,
    structure_overlap: f64,
    symbol_overlap: f64,
    role_match: f64,
    boilerplate_score: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DuplicateRelation {
    ExactDuplicate,
    NormalizedDuplicate,
    LexicalNearDuplicate,
    StructuralClone,
    BoilerplateVariant,
    SimilarButDistinct,
}

impl DuplicateRelation {
    fn as_str(self) -> &'static str {
        match self {
            Self::ExactDuplicate => "exact_duplicate",
            Self::NormalizedDuplicate => "normalized_duplicate",
            Self::LexicalNearDuplicate => "lexical_near_duplicate",
            Self::StructuralClone => "structural_clone",
            Self::BoilerplateVariant => "boilerplate_variant",
            Self::SimilarButDistinct => "similar_but_distinct",
        }
    }
}

fn build_signature(source: &[u8], cfg: DuplicateCollapseConfig) -> Option<DuplicateSignature> {
    let tokens = tokenize_with_options(source, true, cfg.normalize_numbers);
    if tokens.is_empty() {
        return None;
    }
    let literal_tokens = tokenize_with_options(source, false, cfg.normalize_numbers);
    let exact_hash = hash_token_stream(&tokenize_with_options(source, false, false));
    let normalized_hash = hash_token_stream(&tokens);
    let simhash = simhash_tokens(&literal_tokens);

    let (small_k, small_w, medium_k, medium_w, large_k, large_w) = if tokens.len() < cfg.small_token_threshold {
        (
            cfg.small_k,
            cfg.small_w,
            cfg.small_k.saturating_add(2),
            cfg.small_w.saturating_add(2),
            cfg.medium_k,
            cfg.medium_w,
        )
    } else {
        (
            cfg.small_k.max(9),
            cfg.small_w.max(5),
            cfg.medium_k,
            cfg.medium_w,
            cfg.large_k,
            cfg.large_w,
        )
    };

    let fingerprints = if tokens.len() < cfg.min_tokens {
        [HashSet::new(), HashSet::new(), HashSet::new()]
    } else {
        [
            winnow_fingerprints(&tokens, small_k, small_w),
            winnow_fingerprints(&tokens, medium_k, medium_w),
            winnow_fingerprints(&tokens, large_k, large_w),
        ]
    };

    let fps_total = fingerprints.iter().map(HashSet::len).sum::<usize>();
    if tokens.len() >= cfg.min_tokens && fps_total < cfg.min_fingerprints {
        return None;
    }

    let kgrams = kgram_hashes(&tokens, small_k);
    let literal_kgrams = kgram_hashes(&literal_tokens, small_k);
    Some(DuplicateSignature {
        token_count: tokens.len(),
        exact_hash,
        normalized_hash,
        simhash,
        token_set: tokens.iter().copied().collect(),
        literal_token_set: literal_tokens.iter().copied().collect(),
        literal_kgrams,
        fingerprints,
        kgrams,
    })
}

fn compute_similarity(
    lhs: &DuplicateSignature,
    rhs: &DuplicateSignature,
    cfg: DuplicateCollapseConfig,
    left_ctx: Option<&CandidateContext>,
    right_ctx: Option<&CandidateContext>,
) -> DuplicateMetrics {
    let min_len = lhs.token_count.min(rhs.token_count);
    let max_len = lhs.token_count.max(rhs.token_count);
    let length_ratio = if max_len > 0 {
        min_len as f64 / max_len as f64
    } else {
        0.0
    };
    let exact_match = lhs.exact_hash == rhs.exact_hash;
    let normalized_match = lhs.normalized_hash == rhs.normalized_hash;
    let simhash_hamming = Some((lhs.simhash ^ rhs.simhash).count_ones());
    let structure_overlap = structure_overlap(left_ctx, right_ctx);
    let symbol_overlap = symbol_overlap(left_ctx, right_ctx);
    let role_match = role_match(left_ctx, right_ctx);
    let boilerplate_score = boilerplate_score(left_ctx, right_ctx);

    let mut max_overlap = 0.0;
    for scale_idx in 0..lhs.fingerprints.len() {
        let fa = &lhs.fingerprints[scale_idx];
        let fb = &rhs.fingerprints[scale_idx];
        let min_den = fa.len().min(fb.len());
        if min_den == 0 {
            continue;
        }
        let shared = fa.intersection(fb).count();
        if shared == 0 {
            continue;
        }
        let overlap = shared as f64 / min_den as f64;
        if overlap > max_overlap {
            max_overlap = overlap;
        }
    }

    let token_jaccard = if lhs.token_set.is_empty() || rhs.token_set.is_empty() {
        0.0
    } else {
        let inter = lhs.token_set.intersection(&rhs.token_set).count();
        let uni = lhs.token_set.union(&rhs.token_set).count();
        inter as f64 / uni as f64
    };

    let literal_token_jaccard = if lhs.literal_token_set.is_empty() || rhs.literal_token_set.is_empty() {
        0.0
    } else {
        let inter = lhs.literal_token_set.intersection(&rhs.literal_token_set).count();
        let uni = lhs.literal_token_set.union(&rhs.literal_token_set).count();
        inter as f64 / uni as f64
    };

    let literal_kgram_jaccard = if lhs.literal_kgrams.is_empty() || rhs.literal_kgrams.is_empty() {
        0.0
    } else {
        let inter = lhs.literal_kgrams.intersection(&rhs.literal_kgrams).count();
        let uni = lhs.literal_kgrams.union(&rhs.literal_kgrams).count();
        inter as f64 / uni as f64
    };

    let kgram_jaccard = if lhs.kgrams.is_empty() || rhs.kgrams.is_empty() {
        0.0
    } else {
        let inter = lhs.kgrams.intersection(&rhs.kgrams).count();
        let uni = lhs.kgrams.union(&rhs.kgrams).count();
        inter as f64 / uni as f64
    };

    let normalized_duplicate = normalized_match
        && (literal_token_jaccard >= cfg.min_literal_token_jaccard
            || literal_kgram_jaccard >= cfg.min_literal_kgram_jaccard);
    let duplicate = exact_match
        || normalized_duplicate
        || (length_ratio >= cfg.min_length_ratio
            && !(max_overlap < cfg.min_overlap
                && token_jaccard < cfg.token_sim_threshold
                && kgram_jaccard < cfg.kgram_sim_threshold)
            && !(literal_token_jaccard < cfg.min_literal_token_jaccard
                && literal_kgram_jaccard < cfg.min_literal_kgram_jaccard))
        || (simhash_hamming.unwrap_or(u32::MAX) <= 3
            && length_ratio >= cfg.min_length_ratio
            && literal_token_jaccard >= cfg.min_literal_token_jaccard)
        || (structure_overlap >= 0.8 && symbol_overlap >= 0.6 && length_ratio >= cfg.min_length_ratio);
    let score = if exact_match {
        1.0
    } else if normalized_duplicate {
        0.98
    } else if duplicate {
        max_overlap.max(token_jaccard).max(kgram_jaccard)
    } else {
        0.0
    };
    DuplicateMetrics {
        score,
        duplicate: duplicate && score >= cfg.min_score,
        exact_match,
        normalized_match,
        simhash_hamming,
        length_ratio,
        fingerprint_overlap: max_overlap,
        token_jaccard,
        literal_token_jaccard,
        literal_kgram_jaccard,
        kgram_jaccard,
        structure_overlap,
        symbol_overlap,
        role_match,
        boilerplate_score,
    }
}

fn classify_relation(metrics: &DuplicateMetrics) -> DuplicateRelation {
    if metrics.exact_match {
        DuplicateRelation::ExactDuplicate
    } else if metrics.boilerplate_score >= 1.0
        && metrics.duplicate
        && metrics.structure_overlap >= 0.8
        && metrics.literal_token_jaccard >= 0.55
    {
        DuplicateRelation::BoilerplateVariant
    } else if metrics.normalized_match {
        DuplicateRelation::NormalizedDuplicate
    } else if metrics.structure_overlap >= 0.8 && metrics.symbol_overlap >= 0.6 && metrics.duplicate {
        DuplicateRelation::StructuralClone
    } else if metrics.duplicate {
        DuplicateRelation::LexicalNearDuplicate
    } else {
        DuplicateRelation::SimilarButDistinct
    }
}

pub fn analyze_duplicates(
    texts: &[String],
    cfg: DuplicateCollapseConfig,
    mode: &'static str,
    contexts: &[CandidateContext],
) -> DuplicateSelection {
    let mut pairs: Vec<DuplicatePairSummary> = Vec::new();
    let signatures: Vec<Option<DuplicateSignature>> =
        texts.iter().map(|text| build_signature(text.as_bytes(), cfg)).collect();

    for left in 0..texts.len() {
        let Some(lhs) = signatures[left].as_ref() else {
            continue;
        };
        for right in (left + 1)..texts.len() {
            let Some(rhs) = signatures[right].as_ref() else {
                continue;
            };
            let metrics = compute_similarity(lhs, rhs, cfg, contexts.get(left), contexts.get(right));
            let relation = classify_relation(&metrics);
            pairs.push(DuplicatePairSummary {
                left,
                right,
                relation: relation.as_str().to_string(),
                score: metrics.score,
                duplicate: metrics.duplicate,
                exact_match: metrics.exact_match,
                normalized_match: metrics.normalized_match,
                simhash_hamming: metrics.simhash_hamming,
                length_ratio: metrics.length_ratio,
                fingerprint_overlap: metrics.fingerprint_overlap,
                token_jaccard: metrics.token_jaccard,
                literal_token_jaccard: metrics.literal_token_jaccard,
                literal_kgram_jaccard: metrics.literal_kgram_jaccard,
                kgram_jaccard: metrics.kgram_jaccard,
                structure_overlap: metrics.structure_overlap,
                symbol_overlap: metrics.symbol_overlap,
                role_match: metrics.role_match,
                boilerplate_score: metrics.boilerplate_score,
            });
        }
    }

    let mut kept: Vec<usize> = Vec::new();
    let mut suppressed: Vec<usize> = Vec::new();
    for idx in 0..texts.len() {
        let mut exact_duplicate = false;
        for pair in &pairs {
            if !pair.exact_match {
                continue;
            }
            if pair.right == idx && kept.contains(&pair.left) {
                exact_duplicate = true;
                break;
            }
        }
        if exact_duplicate {
            suppressed.push(idx);
        } else {
            kept.push(idx);
        }
    }

    let groups = build_duplicate_groups(texts.len(), &pairs, &suppressed);

    DuplicateSelection {
        keep_indices: kept,
        suppressed_indices: suppressed,
        pairs,
        groups,
        mode,
        suppression_policy: "exact_only",
    }
}

pub fn analyze_duplicates_for_search(texts: &[String]) -> DuplicateSelection {
    analyze_duplicates(
        texts,
        DuplicateCollapseConfig::for_search_results(),
        "code_retrieval",
        &[],
    )
}

fn pair_redundancy(pair: &DuplicatePairSummary) -> f64 {
    if pair.exact_match {
        return 1.0;
    }
    if pair.normalized_match {
        return 0.8;
    }
    let composite =
        0.45 * pair.score + 0.2 * pair.fingerprint_overlap + 0.2 * pair.token_jaccard + 0.15 * pair.kgram_jaccard;
    composite.clamp(0.0, 0.95)
}

fn pair_redundancy_between(pairs: &[DuplicatePairSummary], left: usize, right: usize) -> f64 {
    let (a, b) = if left < right { (left, right) } else { (right, left) };
    pairs
        .iter()
        .find(|pair| pair.left == a && pair.right == b)
        .map(pair_redundancy)
        .unwrap_or(0.0)
}

fn format_reason(parts: &[(&str, f64)]) -> String {
    let mut out: Vec<String> = Vec::new();
    for (label, value) in parts {
        if *value > 0.0 {
            out.push(format!("{label}={value:.3}"));
        }
    }
    if out.is_empty() {
        "none".to_string()
    } else {
        out.join(", ")
    }
}

fn summarize_candidate_relations(idx: usize, pairs: &[DuplicatePairSummary]) -> Vec<String> {
    let mut relations: Vec<String> = pairs
        .iter()
        .filter(|pair| pair.left == idx || pair.right == idx)
        .filter(|pair| pair.duplicate || pair.exact_match)
        .map(|pair| {
            let other = if pair.left == idx { pair.right } else { pair.left };
            format!("{}:{other}", pair.relation)
        })
        .collect();
    relations.sort();
    relations.dedup();
    relations
}

fn query_class(query: &str, mode: &str) -> String {
    let lower = query.to_lowercase();
    if mode == "docs" {
        if lower.contains("version")
            || lower
                .split_whitespace()
                .any(|tok| tok.chars().any(|ch| ch.is_ascii_digit()))
        {
            return "version_specific_docs".to_string();
        }
        if lower.contains("reference") || lower.contains("api") {
            return "api_reference_search".to_string();
        }
        return "conceptual_docs_search".to_string();
    }
    if lower.contains("def ") || lower.contains("class ") || lower.contains("symbol") {
        return "symbol_lookup".to_string();
    }
    if lower.contains("implement") || lower.contains("how") {
        return "implementation_search".to_string();
    }
    "hybrid_code_search".to_string()
}

fn average_topk_redundancy(order: &[usize], pairs: &[DuplicatePairSummary], k: usize) -> f64 {
    let top: Vec<usize> = order.iter().copied().take(k).collect();
    if top.len() < 2 {
        return 0.0;
    }
    let mut total = 0.0;
    let mut count = 0usize;
    for i in 0..top.len() {
        for j in (i + 1)..top.len() {
            total += pair_redundancy_between(pairs, top[i], top[j]);
            count += 1;
        }
    }
    if count == 0 { 0.0 } else { total / count as f64 }
}

fn candidate_group_map(groups: &[DuplicateGroup]) -> HashMap<usize, usize> {
    let mut out = HashMap::new();
    for group in groups {
        for member in &group.members {
            out.insert(*member, group.group_id);
        }
    }
    out
}

fn should_experimentally_suppress(
    idx: usize,
    representative: usize,
    retrieval_mode: &str,
    texts: &[String],
    contexts: &[CandidateContext],
    analysis: &DuplicateSelection,
    rel_values: &[f64],
    query_features: &QueryFeatures,
    experiments: &ExperimentConfig,
    suppression: SuppressionConfig,
) -> Option<&'static str> {
    if idx == representative {
        return None;
    }
    let pair = analysis.pairs.iter().find(|pair| {
        (pair.left == idx && pair.right == representative) || (pair.left == representative && pair.right == idx)
    })?;
    let role = infer_role(contexts.get(idx), retrieval_mode);
    let rep_role = infer_role(contexts.get(representative), retrieval_mode);
    let ratio = pair.length_ratio;
    let lexical = pair
        .literal_token_jaccard
        .max(pair.literal_kgram_jaccard)
        .max(pair.fingerprint_overlap);
    let query_distinction = query_distinction_score(
        idx,
        representative,
        texts,
        contexts,
        rel_values,
        retrieval_mode,
        query_features,
    );
    let member_aspects = infer_candidate_aspects(
        texts.get(idx).map(String::as_str).unwrap_or(""),
        contexts.get(idx),
        retrieval_mode,
    );
    let rep_aspects = infer_candidate_aspects(
        texts.get(representative).map(String::as_str).unwrap_or(""),
        contexts.get(representative),
        retrieval_mode,
    );
    if retrieval_mode == "docs" && experiments.canonical_docs_mirror_suppression {
        let member_path = contexts
            .get(idx)
            .map(|ctx| ctx.file_path.to_lowercase())
            .unwrap_or_default();
        let rep_path = contexts
            .get(representative)
            .map(|ctx| ctx.file_path.to_lowercase())
            .unwrap_or_default();
        let version_match = query_features.version_tokens.is_empty()
            || query_features
                .version_tokens
                .iter()
                .all(|token| member_path.contains(token) && rep_path.contains(token));
        if rep_path.contains("neo4j.com")
            && !member_path.contains("neo4j.com")
            && version_match
            && pair.duplicate
            && lexical >= suppression.threshold_lexical
            && ratio >= suppression.min_length_ratio
            && ratio <= suppression.max_length_ratio
            && query_distinction <= suppression.threshold_query_distinction
            && !aspect_conflict(&member_aspects, &rep_aspects, query_features)
        {
            return Some("canonical_docs_mirror");
        }
    }
    if pair.structure_overlap < suppression.threshold_struct
        || lexical < suppression.threshold_lexical
        || pair.role_match < suppression.threshold_role
        || ratio < suppression.min_length_ratio
        || ratio > suppression.max_length_ratio
        || query_distinction > suppression.threshold_query_distinction
        || (!suppression.allow_cross_role_suppression && role_conflict(&role, &rep_role))
        || aspect_conflict(&member_aspects, &rep_aspects, query_features)
    {
        return None;
    }
    if retrieval_mode != "docs" && experiments.boilerplate_variant_suppression {
        if role == "helper" && rep_role == "helper" && pair.relation == "boilerplate_variant" {
            return Some("boilerplate_variant");
        }
    }
    if retrieval_mode != "docs" && experiments.helper_clone_suppression {
        if role == "helper"
            && rep_role == "helper"
            && (pair.relation == "normalized_duplicate" || pair.relation == "lexical_near_duplicate")
        {
            return Some("helper_clone");
        }
    }
    None
}

pub fn rerank_diverse_trace_for_search(
    texts: &[String],
    relevance_scores: &[f64],
    query: Option<&str>,
    mode: Option<&str>,
    contexts: &[CandidateContext],
) -> DiverseSelectionTrace {
    rerank_diverse_trace_for_search_with_experiments(
        texts,
        relevance_scores,
        query,
        mode,
        contexts,
        &ExperimentConfig::default(),
    )
}

pub fn rerank_diverse_trace_for_search_with_experiments(
    texts: &[String],
    relevance_scores: &[f64],
    query: Option<&str>,
    mode: Option<&str>,
    contexts: &[CandidateContext],
    experiments: &ExperimentConfig,
) -> DiverseSelectionTrace {
    let retrieval_mode = mode.unwrap_or("code");
    let analysis = analyze_duplicates(
        texts,
        DuplicateCollapseConfig::for_search_results(),
        if retrieval_mode == "docs" {
            "docs_retrieval"
        } else {
            "code_retrieval"
        },
        contexts,
    );
    let kept_set: HashSet<usize> = analysis.keep_indices.iter().copied().collect();
    if analysis.keep_indices.is_empty() {
        return DiverseSelectionTrace {
            selection: DiverseSelection {
                mode: analysis.mode,
                keep_indices: Vec::new(),
                suppressed_indices: analysis.suppressed_indices.clone(),
                exact_suppressed_indices: analysis.suppressed_indices.clone(),
                group_order: Vec::new(),
                representative_indices: Vec::new(),
                mmr_lambda: 0.78,
                aspect_lambda: 0.18,
                group_diversity_lambda: 0.08,
                selected_aspects: Vec::new(),
            },
            candidates: Vec::new(),
            telemetry: DuplicateTelemetry {
                mode: analysis.mode,
                query_class: query_class(query.unwrap_or(""), retrieval_mode),
                exact_suppressions: analysis.suppressed_indices.len(),
                experimental_suppressions: 0,
                relation_counts: BTreeMap::new(),
                group_sizes: Vec::new(),
                representative_selection_reasons: BTreeMap::new(),
                topk_redundancy_before: 0.0,
                topk_redundancy_after: 0.0,
                kept_group_multi_member_count: 0,
                multi_representative_group_count: 0,
                query_distinct_multi_rep_count: 0,
                canonical_doc_preference_success: None,
                version_sensitive_query: false,
                best_answer_loss_suspect: false,
                regression_alerts: Vec::new(),
            },
            suppression_policy: "exact_only",
            experiments: experiments.clone(),
        };
    }

    let lambda = 0.78;
    let aspect_lambda = 0.18;
    let group_lambda = 0.08;
    let suppression = SuppressionConfig::from_experiments(experiments);
    let query_features = infer_query_features(query.unwrap_or(""), retrieval_mode);
    let rel_values: Vec<f64> = if relevance_scores.len() == texts.len() {
        relevance_scores.to_vec()
    } else {
        (0..texts.len()).map(|idx| 1.0 / (idx as f64 + 1.0)).collect()
    };
    let max_rel = rel_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_rel = rel_values.iter().copied().fold(f64::INFINITY, f64::min);
    let normalize_rel = |idx: usize| -> f64 {
        let raw = *rel_values.get(idx).unwrap_or(&0.0);
        if !max_rel.is_finite() || !min_rel.is_finite() || (max_rel - min_rel).abs() < f64::EPSILON {
            return 1.0;
        }
        ((raw - min_rel) / (max_rel - min_rel)).clamp(0.0, 1.0)
    };

    let mut group_members: Vec<(usize, Vec<usize>)> = analysis
        .groups
        .iter()
        .map(|group| {
            let mut members: Vec<usize> = group
                .members
                .iter()
                .copied()
                .filter(|idx| kept_set.contains(idx))
                .collect();
            members.sort_by(|a, b| {
                let a_score = representative_priority(
                    *a,
                    texts.get(*a).map(String::as_str).unwrap_or(""),
                    contexts.get(*a),
                    rel_values.get(*a).copied().unwrap_or(0.0),
                    retrieval_mode,
                    &query_features,
                );
                let b_score = representative_priority(
                    *b,
                    texts.get(*b).map(String::as_str).unwrap_or(""),
                    contexts.get(*b),
                    rel_values.get(*b).copied().unwrap_or(0.0),
                    retrieval_mode,
                    &query_features,
                );
                b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
            });
            (group.group_id, members)
        })
        .filter(|(_, members)| !members.is_empty())
        .collect();
    if group_members.is_empty() {
        let fallback = analysis.keep_indices.clone();
        return DiverseSelectionTrace {
            selection: DiverseSelection {
                mode: analysis.mode,
                keep_indices: fallback.clone(),
                suppressed_indices: analysis.suppressed_indices.clone(),
                exact_suppressed_indices: analysis.suppressed_indices.clone(),
                group_order: Vec::new(),
                representative_indices: fallback,
                mmr_lambda: lambda,
                aspect_lambda,
                group_diversity_lambda: group_lambda,
                selected_aspects: Vec::new(),
            },
            candidates: Vec::new(),
            telemetry: DuplicateTelemetry {
                mode: analysis.mode,
                query_class: query_class(query.unwrap_or(""), retrieval_mode),
                exact_suppressions: analysis.suppressed_indices.len(),
                experimental_suppressions: 0,
                relation_counts: BTreeMap::new(),
                group_sizes: Vec::new(),
                representative_selection_reasons: BTreeMap::new(),
                topk_redundancy_before: 0.0,
                topk_redundancy_after: 0.0,
                kept_group_multi_member_count: 0,
                multi_representative_group_count: 0,
                query_distinct_multi_rep_count: 0,
                canonical_doc_preference_success: None,
                version_sensitive_query: !query_features.version_tokens.is_empty(),
                best_answer_loss_suspect: false,
                regression_alerts: Vec::new(),
            },
            suppression_policy: "exact_only",
            experiments: experiments.clone(),
        };
    }

    let mut selected_groups: Vec<usize> = Vec::new();
    let mut selected_reps: Vec<usize> = Vec::new();
    let mut selected_aspects: HashSet<String> = HashSet::new();
    let mut group_choice_scores: HashMap<usize, (f64, f64, f64, f64, f64, f64)> = HashMap::new();
    let mut representative_reason_counts: BTreeMap<String, usize> = BTreeMap::new();
    while !group_members.is_empty() {
        let mut best_pos = 0usize;
        let mut best_score = f64::NEG_INFINITY;
        let mut best_parts = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for (pos, (group_id, members)) in group_members.iter().enumerate() {
            let idx = members[0];
            let rel = normalize_rel(idx);
            let redundancy = selected_reps
                .iter()
                .map(|sel| {
                    let base = pair_redundancy_between(&analysis.pairs, idx, *sel);
                    let same_group = same_duplicate_group(&analysis.groups, idx, *sel);
                    if same_group { (base + 0.15).min(1.0) } else { base }
                })
                .fold(0.0, f64::max);
            let aspect_gain = candidate_aspect_gain(
                &infer_candidate_aspects(
                    texts.get(idx).map(String::as_str).unwrap_or(""),
                    contexts.get(idx),
                    retrieval_mode,
                ),
                &selected_aspects,
                &query_features,
            );
            let role_bonus = candidate_role_bonus(contexts.get(idx), &query_features, retrieval_mode);
            let source_bonus = source_diversity_bonus(
                contexts.get(idx),
                &selected_reps,
                contexts,
                retrieval_mode,
                &query_features,
            );
            let group_bonus = group_diversity_bonus(idx, &selected_reps, contexts, retrieval_mode);
            let score = lambda * rel - (1.0 - lambda) * redundancy
                + aspect_lambda * aspect_gain
                + role_bonus
                + source_bonus
                + group_lambda * group_bonus;
            let _ = group_id;
            if score > best_score {
                best_score = score;
                best_pos = pos;
                best_parts = (rel, redundancy, aspect_gain, role_bonus, source_bonus, group_bonus);
            }
        }
        let (group_id, members) = group_members.remove(best_pos);
        selected_groups.push(group_id);
        let rep = members[0];
        group_choice_scores.insert(group_id, best_parts);
        let reason = format_reason(&[
            ("relevance", lambda * best_parts.0),
            ("redundancy_penalty", (1.0 - lambda) * best_parts.1),
            ("aspect_gain", aspect_lambda * best_parts.2),
            ("role_bonus", best_parts.3),
            ("source_bonus", best_parts.4),
            ("group_bonus", group_lambda * best_parts.5),
        ]);
        *representative_reason_counts.entry(reason).or_insert(0) += 1;
        for aspect in infer_candidate_aspects(
            texts.get(rep).map(String::as_str).unwrap_or(""),
            contexts.get(rep),
            retrieval_mode,
        ) {
            selected_aspects.insert(aspect);
        }
        selected_reps.push(rep);
    }

    let mut final_order: Vec<usize> = Vec::new();
    let mut experimental_suppressed: Vec<usize> = Vec::new();
    let mut query_distinct_multi_rep_count = 0usize;
    let mut multi_representative_group_count = 0usize;
    for group_id in &selected_groups {
        if let Some(group) = analysis.groups.iter().find(|g| &g.group_id == group_id) {
            let mut members: Vec<usize> = group
                .members
                .iter()
                .copied()
                .filter(|idx| kept_set.contains(idx))
                .collect();
            members.sort_by(|a, b| {
                let a_score = representative_priority(
                    *a,
                    texts.get(*a).map(String::as_str).unwrap_or(""),
                    contexts.get(*a),
                    rel_values.get(*a).copied().unwrap_or(0.0),
                    retrieval_mode,
                    &query_features,
                );
                let b_score = representative_priority(
                    *b,
                    texts.get(*b).map(String::as_str).unwrap_or(""),
                    contexts.get(*b),
                    rel_values.get(*b).copied().unwrap_or(0.0),
                    retrieval_mode,
                    &query_features,
                );
                b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
            });
            let representative = *members.first().unwrap_or(&usize::MAX);
            let representative_aspects = infer_candidate_aspects(
                texts.get(representative).map(String::as_str).unwrap_or(""),
                contexts.get(representative),
                retrieval_mode,
            );
            let representative_term = query_term_coverage(&representative_aspects, &query_features);
            let mut kept_in_group = 0usize;
            for member in members {
                let member_aspects = infer_candidate_aspects(
                    texts.get(member).map(String::as_str).unwrap_or(""),
                    contexts.get(member),
                    retrieval_mode,
                );
                let distinction = query_distinction_score(
                    member,
                    representative,
                    texts,
                    contexts,
                    &rel_values,
                    retrieval_mode,
                    &query_features,
                );
                let member_term = query_term_coverage(&member_aspects, &query_features);
                let aspect_gain_diff = (member_term - representative_term).abs();
                let keep_as_additional_representative = member != representative
                    && (distinction > suppression.threshold_query_distinction || aspect_gain_diff > 0.34);
                if let Some(reason) = should_experimentally_suppress(
                    member,
                    representative,
                    retrieval_mode,
                    texts,
                    contexts,
                    &analysis,
                    &rel_values,
                    &query_features,
                    experiments,
                    suppression,
                ) {
                    if keep_as_additional_representative && reason != "canonical_docs_mirror" {
                        final_order.push(member);
                        kept_in_group += 1;
                        query_distinct_multi_rep_count += 1;
                        continue;
                    }
                    experimental_suppressed.push(member);
                    continue;
                }
                final_order.push(member);
                kept_in_group += 1;
            }
            if kept_in_group > 1 {
                multi_representative_group_count += 1;
            }
        }
    }

    let group_map = candidate_group_map(&analysis.groups);
    let exact_suppressed_set: HashSet<usize> = analysis.suppressed_indices.iter().copied().collect();
    let experimental_suppressed_set: HashSet<usize> = experimental_suppressed.iter().copied().collect();
    let final_set: HashSet<usize> = final_order.iter().copied().collect();
    let rep_set: HashSet<usize> = selected_reps.iter().copied().collect();
    let mut traces: Vec<CandidateTrace> = Vec::new();
    for idx in 0..texts.len() {
        let base = rel_values.get(idx).copied().unwrap_or(0.0);
        let normalized = normalize_rel(idx);
        let duplicate_relations = summarize_candidate_relations(idx, &analysis.pairs);
        let group_id = group_map.get(&idx).copied();
        let mut redundancy = 0.0;
        let mut aspect_gain = 0.0;
        let mut group_bonus = 0.0;
        let mut role_bonus = 0.0;
        let mut source_bonus = 0.0;
        let mut query_distinction = 0.0;
        let mut final_score = base;
        let mut beaten_by = None;
        let representative_reason = if let Some(group_id) = group_id {
            if let Some(parts) = group_choice_scores.get(&group_id) {
                format_reason(&[
                    ("relevance", lambda * parts.0),
                    ("redundancy_penalty", (1.0 - lambda) * parts.1),
                    ("aspect_gain", aspect_lambda * parts.2),
                    ("role_bonus", parts.3),
                    ("source_bonus", parts.4),
                    ("group_bonus", group_lambda * parts.5),
                ])
            } else {
                "group_fallback".to_string()
            }
        } else {
            "ungrouped".to_string()
        };
        if !exact_suppressed_set.contains(&idx) {
            let candidate_aspects = infer_candidate_aspects(
                texts.get(idx).map(String::as_str).unwrap_or(""),
                contexts.get(idx),
                retrieval_mode,
            );
            aspect_gain = candidate_aspect_gain(&candidate_aspects, &selected_aspects, &query_features);
            role_bonus = candidate_role_bonus(contexts.get(idx), &query_features, retrieval_mode);
            source_bonus = source_diversity_bonus(
                contexts.get(idx),
                &selected_reps,
                contexts,
                retrieval_mode,
                &query_features,
            );
            group_bonus = group_diversity_bonus(idx, &selected_reps, contexts, retrieval_mode);
            redundancy = selected_reps
                .iter()
                .filter(|sel| **sel != idx)
                .map(|sel| pair_redundancy_between(&analysis.pairs, idx, *sel))
                .fold(0.0, f64::max);
            final_score = lambda * normalized - (1.0 - lambda) * redundancy
                + aspect_lambda * aspect_gain
                + role_bonus
                + source_bonus
                + group_lambda * group_bonus;
            if !rep_set.contains(&idx) {
                beaten_by = selected_reps
                    .iter()
                    .find(|rep| same_duplicate_group(&analysis.groups, idx, **rep))
                    .copied();
                if let Some(rep) = beaten_by {
                    query_distinction = query_distinction_score(
                        idx,
                        rep,
                        texts,
                        contexts,
                        &rel_values,
                        retrieval_mode,
                        &query_features,
                    );
                }
            }
        }
        let decision_reason = if exact_suppressed_set.contains(&idx) {
            "exact_duplicate_suppressed".to_string()
        } else if experimental_suppressed_set.contains(&idx) {
            "experimental_non_exact_suppressed".to_string()
        } else if rep_set.contains(&idx) {
            "group_representative_kept".to_string()
        } else if final_set.contains(&idx) {
            "group_member_kept_for_query_coverage".to_string()
        } else {
            "not_selected".to_string()
        };
        traces.push(CandidateTrace {
            idx,
            group_id,
            base_relevance: base,
            normalized_relevance: normalized,
            final_score,
            duplicate_relations,
            redundancy_penalty: redundancy,
            aspect_coverage_gain: aspect_gain,
            group_diversity_bonus: group_bonus,
            role_bonus,
            source_bonus,
            query_distinction_score: query_distinction,
            representative_reason,
            decision_reason,
            kept: final_set.contains(&idx),
            exact_suppressed: exact_suppressed_set.contains(&idx),
            experimental_suppressed: experimental_suppressed_set.contains(&idx),
            beaten_by,
        });
    }

    let mut relation_counts = BTreeMap::new();
    for pair in &analysis.pairs {
        *relation_counts.entry(pair.relation.clone()).or_insert(0) += 1;
    }
    let canonical_success = if retrieval_mode == "docs" && !final_order.is_empty() {
        let first = final_order[0];
        contexts
            .get(first)
            .map(|ctx| ctx.file_path.to_lowercase().contains("neo4j.com"))
    } else {
        None
    };
    let raw_best = rel_values
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(idx, _)| idx);
    let best_answer_loss_suspect = raw_best.is_some_and(|idx| {
        if !experimental_suppressed_set.contains(&idx) {
            return false;
        }
        if retrieval_mode == "docs" {
            let suppressed_is_noncanonical = !is_canonical_docs_source(contexts.get(idx));
            let kept_has_canonical = final_order
                .first()
                .and_then(|winner| contexts.get(*winner))
                .map(|ctx| is_canonical_docs_source(Some(ctx)))
                .unwrap_or(false);
            if suppressed_is_noncanonical && kept_has_canonical {
                return false;
            }
        }
        true
    });
    let mut regression_alerts: Vec<String> = Vec::new();
    let topk_before = average_topk_redundancy(&analysis.keep_indices, &analysis.pairs, 5);
    let topk_after = average_topk_redundancy(&final_order, &analysis.pairs, 5);
    if !experimental_suppressed.is_empty() && topk_after >= topk_before {
        regression_alerts.push("suppression_without_redundancy_gain".to_string());
    }
    if canonical_success == Some(false) {
        regression_alerts.push("canonical_preference_failed".to_string());
    }
    if best_answer_loss_suspect {
        regression_alerts.push("best_answer_loss_suspect".to_string());
    }
    let telemetry = DuplicateTelemetry {
        mode: analysis.mode,
        query_class: query_class(query.unwrap_or(""), retrieval_mode),
        exact_suppressions: analysis.suppressed_indices.len(),
        experimental_suppressions: experimental_suppressed.len(),
        relation_counts,
        group_sizes: analysis.groups.iter().map(|group| group.members.len()).collect(),
        representative_selection_reasons: representative_reason_counts,
        topk_redundancy_before: topk_before,
        topk_redundancy_after: topk_after,
        kept_group_multi_member_count: analysis
            .groups
            .iter()
            .filter(|group| group.members.iter().filter(|idx| final_set.contains(idx)).count() > 1)
            .count(),
        multi_representative_group_count,
        query_distinct_multi_rep_count,
        canonical_doc_preference_success: canonical_success,
        version_sensitive_query: !query_features.version_tokens.is_empty(),
        best_answer_loss_suspect,
        regression_alerts,
    };

    DiverseSelectionTrace {
        selection: DiverseSelection {
            mode: analysis.mode,
            keep_indices: final_order,
            suppressed_indices: analysis
                .suppressed_indices
                .iter()
                .copied()
                .chain(experimental_suppressed.iter().copied())
                .collect(),
            exact_suppressed_indices: analysis.suppressed_indices.clone(),
            group_order: selected_groups,
            representative_indices: selected_reps,
            mmr_lambda: lambda,
            aspect_lambda,
            group_diversity_lambda: group_lambda,
            selected_aspects: selected_aspects.into_iter().collect(),
        },
        candidates: traces,
        telemetry,
        suppression_policy: "exact_only",
        experiments: experiments.clone(),
    }
}

pub fn rerank_diverse_for_search(
    texts: &[String],
    relevance_scores: &[f64],
    query: Option<&str>,
    mode: Option<&str>,
    contexts: &[CandidateContext],
) -> DiverseSelection {
    rerank_diverse_trace_for_search(texts, relevance_scores, query, mode, contexts).selection
}

pub fn select_non_duplicate_indices(texts: &[String], cfg: DuplicateCollapseConfig) -> Vec<usize> {
    analyze_duplicates(texts, cfg, "custom", &[]).keep_indices
}

fn hash_token_stream(tokens: &[u64]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    for token in tokens {
        h ^= *token;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

fn simhash_tokens(tokens: &[u64]) -> u64 {
    let mut bits = [0i32; 64];
    for token in tokens {
        let mut h = *token;
        for bit in 0..64 {
            if (h & 1) == 1 {
                bits[bit] += 1;
            } else {
                bits[bit] -= 1;
            }
            h >>= 1;
        }
    }
    let mut out = 0u64;
    for bit in 0..64 {
        if bits[bit] > 0 {
            out |= 1u64 << bit;
        }
    }
    out
}

fn metadata_list<'a>(ctx: Option<&'a CandidateContext>, key: &str) -> Vec<String> {
    ctx.and_then(|c| c.metadata.get(key))
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn structure_overlap(left_ctx: Option<&CandidateContext>, right_ctx: Option<&CandidateContext>) -> f64 {
    let left = metadata_list(left_ctx, "node_types");
    let right = metadata_list(right_ctx, "node_types");
    jaccard_strings(&left, &right)
}

fn symbol_overlap(left_ctx: Option<&CandidateContext>, right_ctx: Option<&CandidateContext>) -> f64 {
    let left = metadata_list(left_ctx, "file_symbols");
    let right = metadata_list(right_ctx, "file_symbols");
    jaccard_strings(&left, &right)
}

fn infer_role(ctx: Option<&CandidateContext>, mode: &str) -> String {
    let file_path = ctx.map(|c| c.file_path.to_lowercase()).unwrap_or_default();
    if file_path.contains("test") {
        return "test".to_string();
    }
    if mode == "docs" {
        if file_path.contains("example") {
            return "example".to_string();
        }
        if file_path.contains("reference") || file_path.contains("api") {
            return "reference".to_string();
        }
        if file_path.contains("tutorial") || file_path.contains("guide") {
            return "tutorial".to_string();
        }
        return "docs".to_string();
    }
    if file_path.contains("routes") || file_path.contains("controller") || file_path.contains("/api/") {
        return "api".to_string();
    }
    if file_path.contains("helper") || file_path.contains("util") {
        return "helper".to_string();
    }
    if file_path.contains("service") || file_path.contains("impl") {
        return "implementation".to_string();
    }
    "definition".to_string()
}

fn role_match(left_ctx: Option<&CandidateContext>, right_ctx: Option<&CandidateContext>) -> f64 {
    let left = infer_role(left_ctx, "code");
    let right = infer_role(right_ctx, "code");
    if left == right { 1.0 } else { 0.0 }
}

fn boilerplate_score(left_ctx: Option<&CandidateContext>, right_ctx: Option<&CandidateContext>) -> f64 {
    let left = infer_role(left_ctx, "code");
    let right = infer_role(right_ctx, "code");
    if left == "helper" && right == "helper" {
        1.0
    } else {
        0.0
    }
}

fn jaccard_strings(left: &[String], right: &[String]) -> f64 {
    if left.is_empty() || right.is_empty() {
        return 0.0;
    }
    let left_set: HashSet<&String> = left.iter().collect();
    let right_set: HashSet<&String> = right.iter().collect();
    let inter = left_set.intersection(&right_set).count();
    let union = left_set.union(&right_set).count();
    if union == 0 { 0.0 } else { inter as f64 / union as f64 }
}

#[derive(Default)]
struct QueryFeatures {
    aspects: HashSet<String>,
    role_targets: HashSet<String>,
    version_tokens: HashSet<String>,
}

#[derive(Clone, Copy, Debug)]
struct SuppressionConfig {
    threshold_struct: f64,
    threshold_lexical: f64,
    threshold_role: f64,
    min_length_ratio: f64,
    max_length_ratio: f64,
    threshold_query_distinction: f64,
    allow_cross_role_suppression: bool,
}

fn infer_query_features(query: &str, mode: &str) -> QueryFeatures {
    let lower = query.to_lowercase();
    let mut features = QueryFeatures::default();
    for token in lower.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_' && ch != '.') {
        if token.is_empty() {
            continue;
        }
        if token.chars().any(|ch| ch.is_ascii_digit()) && token.contains('.') {
            features.version_tokens.insert(token.to_string());
        }
        features.aspects.insert(token.to_string());
    }
    let roles: &[&str] = if mode == "docs" {
        &["reference", "tutorial", "example", "docs"]
    } else {
        &["api", "definition", "implementation", "helper", "test"]
    };
    for role in roles {
        if lower.contains(role) {
            features.role_targets.insert(role.to_string());
        }
    }
    features
}

fn infer_candidate_aspects(text: &str, ctx: Option<&CandidateContext>, mode: &str) -> HashSet<String> {
    let mut aspects = HashSet::new();
    if let Some(ctx) = ctx {
        for part in ctx.file_path.split('/') {
            let part = part.trim().to_lowercase();
            if !part.is_empty() {
                aspects.insert(part);
            }
        }
        for key in ["context_path", "file_symbols"] {
            if let Some(values) = ctx.metadata.get(key).and_then(|v| v.as_array()) {
                for value in values {
                    if let Some(s) = value.as_str() {
                        aspects.insert(s.to_lowercase());
                    }
                }
            }
        }
    }
    for hint in [
        "parse",
        "route",
        "validate",
        "serialize",
        "index",
        "delete",
        "create",
        "example",
        "reference",
        "tutorial",
    ] {
        if text.to_lowercase().contains(hint) {
            aspects.insert(hint.to_string());
        }
    }
    aspects.insert(infer_role(ctx, mode));
    aspects
}

fn candidate_aspect_gain(
    candidate_aspects: &HashSet<String>,
    selected_aspects: &HashSet<String>,
    query: &QueryFeatures,
) -> f64 {
    let query_relevant: HashSet<&String> = candidate_aspects.intersection(&query.aspects).collect();
    let unseen = query_relevant
        .iter()
        .filter(|aspect| !selected_aspects.contains((*aspect).as_str()))
        .count();
    if query_relevant.is_empty() {
        0.0
    } else {
        unseen as f64 / query_relevant.len() as f64
    }
}

fn query_term_coverage(candidate_aspects: &HashSet<String>, query: &QueryFeatures) -> f64 {
    if query.aspects.is_empty() {
        return 0.0;
    }
    let matched = candidate_aspects.intersection(&query.aspects).count();
    matched as f64 / query.aspects.len() as f64
}

fn symbol_path_match_score(ctx: Option<&CandidateContext>, query: &QueryFeatures) -> f64 {
    let Some(ctx) = ctx else {
        return 0.0;
    };
    if query.aspects.is_empty() {
        return 0.0;
    }
    let mut matched = 0usize;
    let file_path = ctx.file_path.to_lowercase();
    for aspect in &query.aspects {
        if file_path.contains(aspect) {
            matched += 1;
            continue;
        }
        if let Some(symbols) = ctx.metadata.get("file_symbols").and_then(|v| v.as_array()) {
            if symbols
                .iter()
                .filter_map(|value| value.as_str())
                .any(|symbol| symbol.to_lowercase().contains(aspect))
            {
                matched += 1;
            }
        }
    }
    matched as f64 / query.aspects.len() as f64
}

fn split_identifier_tokens(value: &str) -> Vec<String> {
    value
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .flat_map(|part| {
            let mut tokens = Vec::new();
            let mut current = String::new();
            let mut prev_lower = false;
            for ch in part.chars() {
                if ch == '_' || ch == '-' {
                    if !current.is_empty() {
                        tokens.push(current.to_lowercase());
                        current.clear();
                    }
                    prev_lower = false;
                    continue;
                }
                let is_upper = ch.is_ascii_uppercase();
                if is_upper && prev_lower && !current.is_empty() {
                    tokens.push(current.to_lowercase());
                    current.clear();
                }
                current.push(ch);
                prev_lower = ch.is_ascii_lowercase();
            }
            if !current.is_empty() {
                tokens.push(current.to_lowercase());
            }
            tokens
        })
        .filter(|token| !token.is_empty())
        .collect()
}

fn helper_focus_tokens(ctx: Option<&CandidateContext>) -> HashSet<String> {
    let mut tokens = HashSet::new();
    let Some(ctx) = ctx else {
        return tokens;
    };
    if let Some(name) = ctx.file_path.rsplit('/').next() {
        let stem = name.split('.').next().unwrap_or(name);
        for token in split_identifier_tokens(stem) {
            if token != "helper" && token != "util" && token != "utils" {
                tokens.insert(token);
            }
        }
    }
    if let Some(symbols) = ctx.metadata.get("file_symbols").and_then(|v| v.as_array()) {
        for symbol in symbols.iter().filter_map(|value| value.as_str()).take(3) {
            for token in split_identifier_tokens(symbol) {
                if token != "helper" && token != "util" && token != "utils" {
                    tokens.insert(token);
                }
            }
        }
    }
    tokens
}

fn helper_query_distinction(
    left_ctx: Option<&CandidateContext>,
    right_ctx: Option<&CandidateContext>,
    query: &QueryFeatures,
) -> f64 {
    if query.aspects.is_empty() {
        return 0.0;
    }
    let left_tokens = helper_focus_tokens(left_ctx);
    let right_tokens = helper_focus_tokens(right_ctx);
    if left_tokens.is_empty() && right_tokens.is_empty() {
        return 0.0;
    }
    let left_query_matches: HashSet<&String> = left_tokens.intersection(&query.aspects).collect();
    let right_query_matches: HashSet<&String> = right_tokens.intersection(&query.aspects).collect();
    if left_query_matches.is_empty() && right_query_matches.is_empty() {
        return 0.0;
    }
    if left_query_matches != right_query_matches {
        return 0.32;
    }
    let overlap = jaccard_strings(
        &left_tokens.iter().cloned().collect::<Vec<_>>(),
        &right_tokens.iter().cloned().collect::<Vec<_>>(),
    );
    (1.0 - overlap).clamp(0.0, 0.18)
}

fn role_conflict(left_role: &str, right_role: &str) -> bool {
    if left_role == right_role {
        return false;
    }
    matches!(
        (left_role, right_role),
        ("api", "helper")
            | ("helper", "api")
            | ("reference", "tutorial")
            | ("tutorial", "reference")
            | ("definition", "helper")
            | ("helper", "definition")
    )
}

fn aspect_conflict(left_aspects: &HashSet<String>, right_aspects: &HashSet<String>, query: &QueryFeatures) -> bool {
    let left_query: HashSet<&String> = left_aspects.intersection(&query.aspects).collect();
    let right_query: HashSet<&String> = right_aspects.intersection(&query.aspects).collect();
    !left_query.is_empty() && !right_query.is_empty() && left_query != right_query
}

fn query_distinction_score(
    left_idx: usize,
    right_idx: usize,
    texts: &[String],
    contexts: &[CandidateContext],
    rel_values: &[f64],
    mode: &str,
    query: &QueryFeatures,
) -> f64 {
    let left_text = texts.get(left_idx).map(String::as_str).unwrap_or("");
    let right_text = texts.get(right_idx).map(String::as_str).unwrap_or("");
    let left_aspects = infer_candidate_aspects(left_text, contexts.get(left_idx), mode);
    let right_aspects = infer_candidate_aspects(right_text, contexts.get(right_idx), mode);
    let left_term = query_term_coverage(&left_aspects, query);
    let right_term = query_term_coverage(&right_aspects, query);
    let left_path = symbol_path_match_score(contexts.get(left_idx), query);
    let right_path = symbol_path_match_score(contexts.get(right_idx), query);
    let left_gain = candidate_aspect_gain(&left_aspects, &HashSet::new(), query);
    let right_gain = candidate_aspect_gain(&right_aspects, &HashSet::new(), query);
    let left_rel = *rel_values.get(left_idx).unwrap_or(&0.0);
    let right_rel = *rel_values.get(right_idx).unwrap_or(&0.0);
    let left_role = infer_role(contexts.get(left_idx), mode);
    let right_role = infer_role(contexts.get(right_idx), mode);
    let role_component = if role_conflict(&left_role, &right_role) {
        0.22
    } else {
        0.0
    };
    let helper_component = if left_role == "helper" && right_role == "helper" {
        helper_query_distinction(contexts.get(left_idx), contexts.get(right_idx), query)
    } else {
        0.0
    };
    let aspect_component = if aspect_conflict(&left_aspects, &right_aspects, query) {
        0.2
    } else {
        0.0
    };
    (0.35 * (left_rel - right_rel).abs()
        + 0.25 * (left_term - right_term).abs()
        + 0.2 * (left_path - right_path).abs()
        + 0.2 * (left_gain - right_gain).abs()
        + role_component
        + helper_component
        + aspect_component)
        .clamp(0.0, 1.0)
}

fn candidate_role_bonus(ctx: Option<&CandidateContext>, query: &QueryFeatures, mode: &str) -> f64 {
    let role = infer_role(ctx, mode);
    let mut bonus = if query.role_targets.contains(&role) { 0.08 } else { 0.0 };
    if mode != "docs" && role == "api" && query.aspects.iter().any(|a| a == "api" || a == "endpoint") {
        bonus += 0.05;
    }
    bonus
}

fn source_diversity_bonus(
    ctx: Option<&CandidateContext>,
    selected_reps: &[usize],
    contexts: &[CandidateContext],
    mode: &str,
    query: &QueryFeatures,
) -> f64 {
    if mode != "docs" {
        return 0.0;
    }
    let Some(ctx) = ctx else {
        return 0.0;
    };
    let file_path = ctx.file_path.to_lowercase();
    let is_canonical = file_path.contains("neo4j.com");
    let mut bonus = if is_canonical { 0.12 } else { 0.0 };
    if !query.version_tokens.is_empty() {
        for token in &query.version_tokens {
            if file_path.contains(token) {
                bonus += 0.05;
            }
        }
    }
    let selected_sources: HashSet<String> = selected_reps
        .iter()
        .filter_map(|idx| contexts.get(*idx))
        .map(|ctx| ctx.file_path.split('/').take(3).collect::<Vec<_>>().join("/"))
        .collect();
    let source_key = file_path.split('/').take(3).collect::<Vec<_>>().join("/");
    if !selected_sources.contains(&source_key) {
        bonus += 0.03;
    }
    bonus
}

fn group_diversity_bonus(idx: usize, selected_reps: &[usize], contexts: &[CandidateContext], mode: &str) -> f64 {
    let role = infer_role(contexts.get(idx), mode);
    let file_path = contexts
        .get(idx)
        .map(|ctx| ctx.file_path.to_lowercase())
        .unwrap_or_default();
    let parent = file_path.split('/').take(3).collect::<Vec<_>>().join("/");
    let seen_role = selected_reps
        .iter()
        .filter_map(|sel| contexts.get(*sel))
        .any(|ctx| infer_role(Some(ctx), mode) == role);
    let seen_parent = selected_reps
        .iter()
        .filter_map(|sel| contexts.get(*sel))
        .map(|ctx| ctx.file_path.to_lowercase())
        .any(|path| path.split('/').take(3).collect::<Vec<_>>().join("/") == parent);
    let mut bonus = 0.0;
    if !seen_role {
        bonus += 0.04;
    }
    if !seen_parent {
        bonus += 0.04;
    }
    bonus
}

fn same_duplicate_group(groups: &[DuplicateGroup], left: usize, right: usize) -> bool {
    groups
        .iter()
        .any(|group| group.members.contains(&left) && group.members.contains(&right))
}

fn is_canonical_docs_source(ctx: Option<&CandidateContext>) -> bool {
    ctx.map(|candidate| candidate.file_path.to_lowercase().contains("neo4j.com"))
        .unwrap_or(false)
}

fn representative_priority(
    idx: usize,
    text: &str,
    ctx: Option<&CandidateContext>,
    relevance: f64,
    mode: &str,
    query: &QueryFeatures,
) -> f64 {
    let _ = idx;
    let aspects = infer_candidate_aspects(text, ctx, mode);
    let aspect_overlap = aspects.intersection(&query.aspects).count() as f64;
    let role_bonus = candidate_role_bonus(ctx, query, mode);
    let completeness = (text.len() as f64 / 200.0).min(0.1);
    let source_bonus = source_diversity_bonus(ctx, &[], &[], mode, query);
    let version_bonus = if mode == "docs" {
        let path = ctx
            .map(|candidate| candidate.file_path.to_lowercase())
            .unwrap_or_default();
        query
            .version_tokens
            .iter()
            .filter(|token| path.contains(*token))
            .count() as f64
            * 0.03
    } else {
        0.0
    };
    relevance + 0.08 * aspect_overlap + role_bonus + completeness + source_bonus + version_bonus
}

fn build_duplicate_groups(
    total_items: usize,
    pairs: &[DuplicatePairSummary],
    suppressed: &[usize],
) -> Vec<DuplicateGroup> {
    let suppressed_set: HashSet<usize> = suppressed.iter().copied().collect();
    let mut parent: Vec<usize> = (0..total_items).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        let mut node = x;
        while parent[node] != node {
            parent[node] = parent[parent[node]];
            node = parent[node];
        }
        node
    }

    fn union(parent: &mut [usize], a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent[rb] = ra;
        }
    }

    for pair in pairs {
        if pair.duplicate {
            union(&mut parent, pair.left, pair.right);
        }
    }

    let mut groups_map: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for idx in 0..total_items {
        if suppressed_set.contains(&idx) {
            continue;
        }
        let root = find(&mut parent, idx);
        groups_map.entry(root).or_default().push(idx);
    }

    groups_map
        .into_iter()
        .enumerate()
        .map(|(group_id, (_root, mut members))| {
            members.sort_unstable();
            DuplicateGroup {
                group_id,
                canonical_candidates: members.clone(),
                members,
            }
        })
        .collect()
}

pub(crate) fn tokenize_normalized(source: &[u8]) -> Vec<u64> {
    tokenize_with_options(source, true, true)
}

fn tokenize_with_options(source: &[u8], normalize_identifiers: bool, normalize_numbers: bool) -> Vec<u64> {
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
            if normalize_identifiers {
                for ch in b"<id>" {
                    h ^= *ch as u64;
                    h = h.wrapping_mul(FNV_PRIME);
                }
            } else {
                for ch in &source[i..j] {
                    h ^= ch.to_ascii_lowercase() as u64;
                    h = h.wrapping_mul(FNV_PRIME);
                }
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
            if normalize_numbers {
                for ch in b"<num>" {
                    h ^= *ch as u64;
                    h = h.wrapping_mul(FNV_PRIME);
                }
            } else {
                for ch in &source[i..j] {
                    h ^= *ch as u64;
                    h = h.wrapping_mul(FNV_PRIME);
                }
            }
            tokens.push(h);
            i = j;
            continue;
        }

        let punct = match b {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'.' | b':' | b'+' | b'-' | b'*' | b'/' | b'%'
            | b'<' | b'>' | b'=' => Some(b),
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

#[cfg(test)]
mod tests {
    use super::{
        CandidateContext, DuplicateCollapseConfig, ExperimentConfig, analyze_duplicates_for_search, kgram_hashes,
        rerank_diverse_for_search, rerank_diverse_trace_for_search, rerank_diverse_trace_for_search_with_experiments,
        select_non_duplicate_indices, tokenize_normalized, winnow_fingerprints,
    };

    #[test]
    fn normalize_identifiers_and_numbers() {
        let tokens = tokenize_normalized(b"count = total + 42");
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], tokens[2]);
        assert_ne!(tokens[0], tokens[3]);
    }

    #[test]
    fn winnow_is_stable_on_repeated_sequence() {
        let tokens = tokenize_normalized(b"count = total + 42");
        let repeated = tokens
            .iter()
            .copied()
            .cycle()
            .take(tokens.len() * 3)
            .collect::<Vec<_>>();
        let fps = winnow_fingerprints(&repeated, 3, 2);
        assert!(!fps.is_empty());
    }

    #[test]
    fn kgrams_exist_for_short_token_sequences() {
        let tokens = tokenize_normalized(b"foo(bar)");
        let grams = kgram_hashes(&tokens, 3);
        assert!(!grams.is_empty());
    }

    #[test]
    fn collapse_keeps_first_of_exact_duplicate_snippets() {
        let rows = vec![
            r#"
            fn add_user(user_id: i64) {
                let normalized = normalize_user_id(user_id);
                if normalized > 0 {
                    insert_user(normalized);
                    audit_user(normalized);
                }
            }
            "#
            .to_string(),
            r#"
            fn add_user(user_id: i64) {
                let normalized = normalize_user_id(user_id);
                if normalized > 0 {
                    insert_user(normalized);
                    audit_user(normalized);
                }
            }
            "#
            .to_string(),
            r#"
            fn add_user(user_id: i64) {
                let normalized = normalize_user_id(user_id);
                if normalized > 0 {
                    insert_user(normalized);
                    audit_user(normalized);
                }
            }
            "#
            .to_string(),
        ];
        let kept = select_non_duplicate_indices(&rows, DuplicateCollapseConfig::for_search_results());
        assert_eq!(kept, vec![0]);
    }

    #[test]
    fn search_mode_does_not_collapse_semantically_different_snippets() {
        let rows = vec![
            "fn create_user() { insert_user(); audit_user(); }".to_string(),
            "fn delete_user() { remove_user(); }".to_string(),
        ];
        let kept = select_non_duplicate_indices(&rows, DuplicateCollapseConfig::for_search_results());
        assert_eq!(kept, vec![0, 1]);
    }

    #[test]
    fn search_analysis_reports_exact_match_and_exact_only_suppression() {
        let rows = vec![
            "fn add_user() { insert_user(); }".to_string(),
            "fn add_user() { insert_user(); }".to_string(),
            "fn delete_user() { remove_user(); }".to_string(),
        ];
        let analysis = analyze_duplicates_for_search(&rows);
        assert_eq!(analysis.keep_indices, vec![0, 2]);
        assert_eq!(analysis.suppressed_indices, vec![1]);
        assert_eq!(analysis.suppression_policy, "exact_only");
        assert!(analysis.pairs.iter().any(|pair| pair.exact_match));
        assert_eq!(analysis.groups.len(), 2);
    }

    #[test]
    fn mmr_reranker_keeps_distinct_result_and_suppresses_only_exact_duplicate() {
        let rows = vec![
            "fn add_user() { insert_user(); }".to_string(),
            "fn add_user() { insert_user(); }".to_string(),
            "fn delete_user() { remove_user(); }".to_string(),
        ];
        let relevance = vec![1.0, 0.95, 0.7];
        let selection = rerank_diverse_for_search(&rows, &relevance, None, None, &[]);
        assert_eq!(selection.keep_indices, vec![0, 2]);
        assert_eq!(selection.exact_suppressed_indices, vec![1]);
        assert_eq!(selection.representative_indices, vec![0, 2]);
    }

    #[test]
    fn duplicate_groups_cluster_non_exact_duplicate_like_results() {
        let rows = vec![
            "fn add_user(user_id: i64) { insert_user(user_id); audit_user(user_id); }".to_string(),
            "fn add_customer(customer_id: i64) { insert_user(customer_id); audit_user(customer_id); }".to_string(),
            "fn delete_user(user_id: i64) { remove_user(user_id); }".to_string(),
        ];
        let analysis = analyze_duplicates_for_search(&rows);
        assert_eq!(analysis.groups.len(), 2);
        assert!(analysis.groups.iter().any(|group| group.members == vec![0, 1]));
    }

    #[test]
    fn query_targeting_api_prefers_api_representative_over_helper() {
        let rows = vec![
            "fn add_user(user_id: i64) { insert_user(user_id); audit_user(user_id); }".to_string(),
            "fn add_customer(customer_id: i64) { insert_user(customer_id); audit_user(customer_id); }".to_string(),
        ];
        let contexts = vec![
            CandidateContext {
                file_path: "src/api/users.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["add_user"],"node_types":["function_item"],"context_path":["Api","Users"]}),
            },
            CandidateContext {
                file_path: "src/helpers/customer_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["add_customer"],"node_types":["function_item"],"context_path":["Helper","Customer"]}),
            },
        ];
        let relevance = vec![0.9, 0.95];
        let selection = rerank_diverse_for_search(&rows, &relevance, Some("user api"), Some("code"), &contexts);
        assert_eq!(selection.representative_indices[0], 0);
    }

    #[test]
    fn docs_mode_prefers_canonical_source_and_version_match() {
        let rows = vec![
            "Transactions guide for Neo4j version 5.26".to_string(),
            "Transactions guide mirror for Neo4j version 5.26".to_string(),
            "Transactions guide for Neo4j version 4.4".to_string(),
        ];
        let contexts = vec![
            CandidateContext {
                file_path: "https://neo4j.com/docs/python-manual/5.26/transactions".to_string(),
                metadata: serde_json::json!({"context_path":["Transactions"],"file_symbols":[],"source_type":"driver-manual"}),
            },
            CandidateContext {
                file_path: "https://mirror.example.com/docs/python-manual/5.26/transactions".to_string(),
                metadata: serde_json::json!({"context_path":["Transactions"],"file_symbols":[],"source_type":"mirror"}),
            },
            CandidateContext {
                file_path: "https://neo4j.com/docs/python-manual/4.4/transactions".to_string(),
                metadata: serde_json::json!({"context_path":["Transactions"],"file_symbols":[],"source_type":"driver-manual"}),
            },
        ];
        let relevance = vec![0.82, 0.86, 0.81];
        let selection = rerank_diverse_for_search(
            &rows,
            &relevance,
            Some("neo4j 5.26 transactions"),
            Some("docs"),
            &contexts,
        );
        assert_eq!(selection.representative_indices[0], 0);
        assert_eq!(selection.keep_indices[0], 0);
        assert_eq!(selection.keep_indices[1], 1);
        assert_eq!(selection.keep_indices[2], 2);
    }

    #[test]
    fn trace_reports_telemetry_and_candidate_reasons() {
        let rows = vec![
            "fn add_user() { insert_user(); }".to_string(),
            "fn add_user() { insert_user(); }".to_string(),
            "fn delete_user() { remove_user(); }".to_string(),
        ];
        let relevance = vec![1.0, 0.95, 0.7];
        let trace = rerank_diverse_trace_for_search(&rows, &relevance, Some("delete user"), Some("code"), &[]);
        assert_eq!(trace.selection.exact_suppressed_indices, vec![1]);
        assert!(trace.telemetry.exact_suppressions >= 1);
        assert!(
            trace
                .candidates
                .iter()
                .any(|candidate| candidate.decision_reason == "exact_duplicate_suppressed")
        );
    }

    #[test]
    fn experimental_docs_mirror_suppression_is_gated() {
        let rows = vec![
            "Transactions guide for Neo4j version 5.26".to_string(),
            "Transactions guide mirror for Neo4j version 5.26".to_string(),
        ];
        let contexts = vec![
            CandidateContext {
                file_path: "https://neo4j.com/docs/python-manual/5.26/transactions".to_string(),
                metadata: serde_json::json!({"context_path":["Transactions"],"file_symbols":[],"source_type":"driver-manual"}),
            },
            CandidateContext {
                file_path: "https://mirror.example.com/docs/python-manual/5.26/transactions".to_string(),
                metadata: serde_json::json!({"context_path":["Transactions"],"file_symbols":[],"source_type":"mirror"}),
            },
        ];
        let relevance = vec![0.82, 0.86];
        let default_trace = rerank_diverse_trace_for_search(
            &rows,
            &relevance,
            Some("neo4j 5.26 transactions"),
            Some("docs"),
            &contexts,
        );
        assert_eq!(default_trace.selection.keep_indices, vec![0, 1]);

        let experimental_trace = rerank_diverse_trace_for_search_with_experiments(
            &rows,
            &relevance,
            Some("neo4j 5.26 transactions"),
            Some("docs"),
            &contexts,
            &ExperimentConfig {
                canonical_docs_mirror_suppression: true,
                ..ExperimentConfig::default()
            },
        );
        assert_eq!(experimental_trace.selection.keep_indices, vec![0]);
        assert_eq!(experimental_trace.telemetry.experimental_suppressions, 1);
    }

    #[test]
    fn helper_clone_experiment_respects_query_distinction() {
        let rows = vec![
            "fn create_user(user_id: i64) { insert_user(user_id); audit_user(user_id); }".to_string(),
            "fn create_customer(customer_id: i64) { insert_user(customer_id); audit_user(customer_id); }".to_string(),
            "fn delete_user(user_id: i64) { remove_user(user_id); }".to_string(),
        ];
        let contexts = vec![
            CandidateContext {
                file_path: "src/helpers/user_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_user"],"node_types":["function_item"],"context_path":["Helper","Users"]}),
            },
            CandidateContext {
                file_path: "src/helpers/customer_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_customer"],"node_types":["function_item"],"context_path":["Helper","Customer"]}),
            },
            CandidateContext {
                file_path: "src/api/users.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["delete_user"],"node_types":["function_item"],"context_path":["Api","Users"]}),
            },
        ];
        let relevance = vec![0.97, 0.9, 0.95];
        let broad = rerank_diverse_trace_for_search_with_experiments(
            &rows,
            &relevance,
            Some("user customer helper"),
            Some("code"),
            &contexts,
            &ExperimentConfig {
                helper_clone_suppression: true,
                ..ExperimentConfig::default()
            },
        );
        assert!(broad.selection.keep_indices.contains(&0));
        assert!(broad.selection.keep_indices.contains(&1));

        let narrow = rerank_diverse_trace_for_search_with_experiments(
            &rows,
            &relevance,
            Some("user customer helper"),
            Some("code"),
            &contexts,
            &ExperimentConfig {
                helper_clone_suppression: true,
                threshold_query_distinction: Some(0.1),
                ..ExperimentConfig::default()
            },
        );
        assert!(narrow.selection.keep_indices.contains(&0));
        assert!(narrow.selection.keep_indices.contains(&1));
        assert_eq!(narrow.telemetry.experimental_suppressions, 0);
    }

    #[test]
    fn helper_query_distinction_detects_entity_specific_helpers() {
        let query = super::infer_query_features("user customer helper", "code");
        let contexts = vec![
            CandidateContext {
                file_path: "src/helpers/user_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_user"],"node_types":["function_item"],"context_path":["Helper","Users"]}),
            },
            CandidateContext {
                file_path: "src/helpers/customer_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_customer"],"node_types":["function_item"],"context_path":["Helper","Customer"]}),
            },
        ];
        let score = super::helper_query_distinction(contexts.get(0), contexts.get(1), &query);
        assert!(score >= 0.3, "expected helper-specific query distinction, got {score}");
    }

    #[test]
    fn helper_clone_suppression_stays_off_for_query_targeted_helper_entities() {
        let rows = vec![
            "fn create_user(user_id: i64) { insert_user(user_id); audit_user(user_id); }".to_string(),
            "fn create_customer(customer_id: i64) { insert_user(customer_id); audit_user(customer_id); }".to_string(),
        ];
        let contexts = vec![
            CandidateContext {
                file_path: "src/helpers/user_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_user"],"node_types":["function_item"],"context_path":["Helper","Users"]}),
            },
            CandidateContext {
                file_path: "src/helpers/customer_helper.rs".to_string(),
                metadata: serde_json::json!({"file_symbols":["create_customer"],"node_types":["function_item"],"context_path":["Helper","Customer"]}),
            },
        ];
        let relevance = vec![0.97, 0.96];
        let trace = rerank_diverse_trace_for_search_with_experiments(
            &rows,
            &relevance,
            Some("user customer helper"),
            Some("code"),
            &contexts,
            &ExperimentConfig {
                helper_clone_suppression: true,
                threshold_query_distinction: Some(0.22),
                ..ExperimentConfig::default()
            },
        );
        assert_eq!(trace.telemetry.experimental_suppressions, 0);
        assert!(trace.selection.keep_indices.contains(&0));
        assert!(trace.selection.keep_indices.contains(&1));
    }
}
