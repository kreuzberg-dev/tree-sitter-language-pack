use std::collections::HashSet;
use std::sync::Arc;

use serde_json::Value;

use crate::tags;

pub(crate) struct FileNode {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) filepath: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) is_test: bool,
}

pub(crate) struct SymbolNode {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) qualified_name: Option<String>,
    pub(crate) filepath: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) start_line: u32,
    pub(crate) end_line: u32,
    pub(crate) start_byte: usize,
    pub(crate) end_byte: usize,
    pub(crate) signature: Option<String>,
    pub(crate) visibility: Option<String>,
    pub(crate) is_exported: bool,
    pub(crate) doc_comment: Option<String>,
}

pub(crate) struct RelRow {
    pub(crate) parent: String,
    pub(crate) child: String,
}

pub(crate) struct SymbolCallRow {
    pub(crate) caller_id: String,
    pub(crate) callee: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String,
    pub(crate) allow_same_file: bool,
}

pub(crate) struct InferredCallRow {
    pub(crate) caller_id: String,
    pub(crate) callee: String,
    pub(crate) receiver_type: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String,
    pub(crate) allow_same_file: bool,
}

pub(crate) struct PythonInferredCallRow {
    pub(crate) caller_id: String,
    pub(crate) callee: String,
    pub(crate) callee_filepath: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String,
    pub(crate) allow_same_file: bool,
}

pub(crate) struct DbEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct DbModelEdgeRow {
    pub(crate) src: String,
    pub(crate) model: String,
    pub(crate) project_id: Arc<str>,
}

pub(crate) struct ExternalApiNode {
    pub(crate) id: String,
    pub(crate) url: String,
    pub(crate) project_id: Arc<str>,
}

pub(crate) struct ExternalApiEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct FileEdgeRow {
    pub(crate) src_filepath: String,
    pub(crate) tgt_filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct ApiRouteCallRow {
    pub(crate) src_filepath: String,
    pub(crate) path: String,
    pub(crate) method: String,
    pub(crate) project_id: String,
}

pub(crate) struct ApiRouteHandlerRow {
    pub(crate) path: String,
    pub(crate) method: String,
    pub(crate) tgt_filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct ResourceUsageRow {
    pub(crate) src_filepath: String,
    pub(crate) rel_name: String,
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) filepath: Option<String>,
    pub(crate) project_id: String,
}

pub(crate) struct ResourceBackingRow {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeTargetRow {
    pub(crate) target_id: String,
    pub(crate) name: String,
    pub(crate) project_file: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeTargetFileRow {
    pub(crate) target_id: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct ResourceTargetEdgeRow {
    pub(crate) target_id: String,
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) filepath: Option<String>,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeWorkspaceRow {
    pub(crate) workspace_path: String,
    pub(crate) name: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeWorkspaceProjectRow {
    pub(crate) workspace_path: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeSchemeRow {
    pub(crate) scheme_path: String,
    pub(crate) name: String,
    pub(crate) container_path: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeSchemeTargetRow {
    pub(crate) scheme_path: String,
    pub(crate) target_id: String,
    pub(crate) project_id: String,
}

pub(crate) struct XcodeSchemeFileRow {
    pub(crate) scheme_path: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct CargoCrateRow {
    pub(crate) name: String,
    pub(crate) crate_name: String,
    pub(crate) manifest_path: Option<String>,
    pub(crate) project_id: String,
}

pub(crate) struct CargoWorkspaceRow {
    pub(crate) manifest_path: String,
    pub(crate) name: String,
    pub(crate) project_id: String,
}

pub(crate) struct CargoWorkspaceCrateRow {
    pub(crate) workspace_manifest_path: String,
    pub(crate) crate_name: String,
    pub(crate) project_id: String,
}

pub(crate) struct CargoCrateFileRow {
    pub(crate) crate_name: String,
    pub(crate) manifest_path: String,
    pub(crate) project_id: String,
}

pub(crate) struct CargoDependencyEdgeRow {
    pub(crate) src_crate_name: String,
    pub(crate) tgt_crate_name: String,
    pub(crate) section: String,
    pub(crate) project_id: String,
}

pub(crate) struct RustImplTraitEdgeRow {
    pub(crate) impl_id: String,
    pub(crate) trait_name: String,
    pub(crate) project_id: String,
}

pub(crate) struct RustImplTypeEdgeRow {
    pub(crate) impl_id: String,
    pub(crate) type_name: String,
    pub(crate) project_id: String,
}

pub(crate) struct CloneGroupRow {
    pub(crate) id: String,
    pub(crate) project_id: String,
    pub(crate) size: usize,
    pub(crate) method: String,
    pub(crate) score_min: f64,
    pub(crate) score_max: f64,
    pub(crate) score_avg: f64,
}

pub(crate) struct CloneMemberRow {
    pub(crate) gid: String,
    pub(crate) sid: String,
}

pub(crate) struct CloneCanonRow {
    pub(crate) gid: String,
    pub(crate) sid: String,
}

pub(crate) struct FileCloneGroupRow {
    pub(crate) id: String,
    pub(crate) project_id: String,
    pub(crate) size: usize,
    pub(crate) method: String,
    pub(crate) score_min: f64,
    pub(crate) score_max: f64,
    pub(crate) score_avg: f64,
}

pub(crate) struct FileCloneMemberRow {
    pub(crate) gid: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct FileCloneCanonRow {
    pub(crate) gid: String,
    pub(crate) filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct LaunchEdgeRow {
    pub(crate) src_filepath: String,
    pub(crate) tgt_filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct FileImportEdgeRow {
    pub(crate) src_filepath: String,
    pub(crate) tgt_filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct ImportSymbolRequest {
    pub(crate) src_id: String,
    pub(crate) src_filepath: String,
    pub(crate) module: String,
    pub(crate) items: Vec<String>,
}

pub(crate) struct ImportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct ImplicitImportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct ExportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct SwiftFileContext {
    pub(crate) file_id: String,
    pub(crate) filepath: String,
    pub(crate) symbol_spans: Vec<(usize, usize, String)>,
    pub(crate) extension_spans: Vec<(usize, usize, String)>,
    pub(crate) type_spans: Vec<(usize, usize, String)>,
    pub(crate) call_sites: Vec<tags::CallSite>,
    pub(crate) var_types: std::collections::HashMap<String, String>,
}

pub(crate) struct PythonFileContext {
    pub(crate) file_id: String,
    pub(crate) filepath: String,
    pub(crate) symbol_spans: Vec<(usize, usize, String)>,
    pub(crate) call_sites: Vec<tags::CallSite>,
    pub(crate) module_aliases: std::collections::HashMap<String, String>,
}

pub(crate) struct CloneCandidate {
    pub(crate) symbol_id: String,
    pub(crate) filepath: String,
    pub(crate) span_len: u32,
    pub(crate) token_set: HashSet<u64>,
    pub(crate) fingerprints: Vec<HashSet<u64>>,
    pub(crate) kgrams: HashSet<u64>,
}

pub(crate) struct ImportNode {
    pub(crate) id: String,
    pub(crate) file_id: String,
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) is_wildcard: bool,
    pub(crate) project_id: Arc<str>,
    pub(crate) filepath: String,
}

impl SymbolCallRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl InferredCallRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("recv".into(), Value::String(self.receiver_type.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl PythonInferredCallRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("callee_fp".into(), Value::String(self.callee_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl FileImportEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("tgt".into(), Value::String(self.tgt_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl DbEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl DbModelEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("model".into(), Value::String(self.model.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m
        })
    }
}

impl ExternalApiNode {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("url".into(), Value::String(self.url.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m
        })
    }
}

impl ExternalApiEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl FileEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("tgt".into(), Value::String(self.tgt_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ApiRouteCallRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("path".into(), Value::String(self.path.clone()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ApiRouteHandlerRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("path".into(), Value::String(self.path.clone()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert("tgt".into(), Value::String(self.tgt_filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ResourceUsageRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("rel_name".into(), Value::String(self.rel_name.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("kind".into(), Value::String(self.kind.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert(
                "filepath".into(),
                self.filepath
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m
        })
    }
}

impl ResourceBackingRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("kind".into(), Value::String(self.kind.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeTargetRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("target_id".into(), Value::String(self.target_id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("project_file".into(), Value::String(self.project_file.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeTargetFileRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("target_id".into(), Value::String(self.target_id.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ResourceTargetEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("target_id".into(), Value::String(self.target_id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("kind".into(), Value::String(self.kind.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert(
                "filepath".into(),
                self.filepath
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m
        })
    }
}

impl XcodeWorkspaceRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("workspace_path".into(), Value::String(self.workspace_path.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeWorkspaceProjectRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("workspace_path".into(), Value::String(self.workspace_path.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeSchemeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("scheme_path".into(), Value::String(self.scheme_path.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("container_path".into(), Value::String(self.container_path.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeSchemeTargetRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("scheme_path".into(), Value::String(self.scheme_path.clone()));
            m.insert("target_id".into(), Value::String(self.target_id.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl XcodeSchemeFileRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("scheme_path".into(), Value::String(self.scheme_path.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl CargoCrateRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("crate_name".into(), Value::String(self.crate_name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert(
                "manifest_path".into(),
                self.manifest_path
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m
        })
    }
}

impl CargoWorkspaceRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("manifest_path".into(), Value::String(self.manifest_path.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl CargoWorkspaceCrateRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert(
                "workspace_manifest_path".into(),
                Value::String(self.workspace_manifest_path.clone()),
            );
            m.insert("crate_name".into(), Value::String(self.crate_name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl CargoCrateFileRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("crate_name".into(), Value::String(self.crate_name.clone()));
            m.insert("manifest_path".into(), Value::String(self.manifest_path.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl CargoDependencyEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src_crate_name".into(), Value::String(self.src_crate_name.clone()));
            m.insert("tgt_crate_name".into(), Value::String(self.tgt_crate_name.clone()));
            m.insert("section".into(), Value::String(self.section.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl RustImplTraitEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("impl_id".into(), Value::String(self.impl_id.clone()));
            m.insert("trait_name".into(), Value::String(self.trait_name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl RustImplTypeEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("impl_id".into(), Value::String(self.impl_id.clone()));
            m.insert("type_name".into(), Value::String(self.type_name.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl CloneGroupRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert("size".into(), Value::Number(self.size.into()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert(
                "score_min".into(),
                Value::Number(serde_json::Number::from_f64(self.score_min).unwrap_or(0.into())),
            );
            m.insert(
                "score_max".into(),
                Value::Number(serde_json::Number::from_f64(self.score_max).unwrap_or(0.into())),
            );
            m.insert(
                "score_avg".into(),
                Value::Number(serde_json::Number::from_f64(self.score_avg).unwrap_or(0.into())),
            );
            m
        })
    }
}

impl CloneMemberRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("sid".into(), Value::String(self.sid.clone()));
            m
        })
    }
}

impl CloneCanonRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("sid".into(), Value::String(self.sid.clone()));
            m
        })
    }
}

impl FileCloneGroupRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert("size".into(), Value::Number(self.size.into()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert(
                "score_min".into(),
                Value::Number(serde_json::Number::from_f64(self.score_min).unwrap_or(0.into())),
            );
            m.insert(
                "score_max".into(),
                Value::Number(serde_json::Number::from_f64(self.score_max).unwrap_or(0.into())),
            );
            m.insert(
                "score_avg".into(),
                Value::Number(serde_json::Number::from_f64(self.score_avg).unwrap_or(0.into())),
            );
            m
        })
    }
}

impl FileCloneMemberRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl FileCloneCanonRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ImportSymbolEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl ImplicitImportSymbolEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl ExportSymbolEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl LaunchEdgeRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("tgt".into(), Value::String(self.tgt_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl FileNode {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m.insert("is_test".into(), Value::Bool(self.is_test));
            m
        })
    }
}

impl SymbolNode {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("kind".into(), Value::String(self.kind.clone()));
            m.insert(
                "qualified_name".into(),
                self.qualified_name
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m.insert("start_line".into(), Value::Number(self.start_line.into()));
            m.insert("end_line".into(), Value::Number(self.end_line.into()));
            m.insert(
                "start_byte".into(),
                Value::Number(serde_json::Number::from(self.start_byte)),
            );
            m.insert(
                "end_byte".into(),
                Value::Number(serde_json::Number::from(self.end_byte)),
            );
            m.insert(
                "signature".into(),
                self.signature
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert(
                "visibility".into(),
                self.visibility
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert("is_exported".into(), Value::Bool(self.is_exported));
            m.insert(
                "doc_comment".into(),
                self.doc_comment
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m
        })
    }
}

impl ImportNode {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("file_id".into(), Value::String(self.file_id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("source".into(), Value::String(self.source.clone()));
            m.insert("is_wildcard".into(), Value::Bool(self.is_wildcard));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m
        })
    }
}

impl RelRow {
    pub(crate) fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("p".into(), Value::String(self.parent.clone()));
            m.insert("c".into(), Value::String(self.child.clone()));
            m
        })
    }
}
