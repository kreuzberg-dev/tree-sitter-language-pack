#![allow(dead_code)]

pub(crate) const NODE_LABEL_FILE: &str = "File";
pub(crate) const NODE_LABEL_IMPORT: &str = "Import";
pub(crate) const NODE_LABEL_MODEL: &str = "Model";
pub(crate) const NODE_LABEL_EXTERNAL_API: &str = "ExternalAPI";
pub(crate) const NODE_LABEL_EXTERNAL_SYMBOL: &str = "ExternalSymbol";
pub(crate) const NODE_LABEL_RESOURCE: &str = "Resource";
pub(crate) const NODE_LABEL_XCODE_TARGET: &str = "XcodeTarget";
pub(crate) const NODE_LABEL_XCODE_WORKSPACE: &str = "XcodeWorkspace";
pub(crate) const NODE_LABEL_XCODE_SCHEME: &str = "XcodeScheme";
pub(crate) const NODE_LABEL_CARGO_CRATE: &str = "CargoCrate";
pub(crate) const NODE_LABEL_CARGO_WORKSPACE: &str = "CargoWorkspace";
pub(crate) const NODE_LABEL_CLONE_GROUP: &str = "CloneGroup";
pub(crate) const NODE_LABEL_FILE_CLONE_GROUP: &str = "FileCloneGroup";
pub(crate) const NODE_LABEL_API_ROUTE: &str = "ApiRoute";

pub(crate) const REL_CONTAINS: &str = "CONTAINS";
pub(crate) const REL_CALLS: &str = "CALLS";
pub(crate) const REL_CALLS_INFERRED: &str = "CALLS_INFERRED";
pub(crate) const REL_CALLS_DB: &str = "CALLS_DB";
pub(crate) const REL_CALLS_DB_MODEL: &str = "CALLS_DB_MODEL";
pub(crate) const REL_CALLS_API: &str = "CALLS_API";
pub(crate) const REL_CALLS_SERVICE: &str = "CALLS_SERVICE";
pub(crate) const REL_CALLS_API_EXTERNAL: &str = "CALLS_API_EXTERNAL";
pub(crate) const REL_CALLS_EXTERNAL_SYMBOL: &str = "CALLS_EXTERNAL_SYMBOL";
pub(crate) const REL_IMPORTS: &str = "IMPORTS";
pub(crate) const REL_IMPORTS_SYMBOL: &str = "IMPORTS_SYMBOL";
pub(crate) const REL_IMPLICIT_IMPORTS_SYMBOL: &str = "IMPLICIT_IMPORTS_SYMBOL";
pub(crate) const REL_EXPORTS_SYMBOL: &str = "EXPORTS_SYMBOL";
pub(crate) const REL_EXPORTS_SYMBOL_AS: &str = "EXPORTS_SYMBOL_AS";
pub(crate) const REL_CALLS_API_ROUTE: &str = "CALLS_API_ROUTE";
pub(crate) const REL_HANDLED_BY: &str = "HANDLED_BY";
pub(crate) const REL_BACKED_BY_FILE: &str = "BACKED_BY_FILE";
pub(crate) const REL_BUNDLED_IN_TARGET: &str = "BUNDLED_IN_TARGET";
pub(crate) const REL_BUNDLES_FILE: &str = "BUNDLES_FILE";
pub(crate) const REL_REFERENCES_PROJECT: &str = "REFERENCES_PROJECT";
pub(crate) const REL_BUILDS_TARGET: &str = "BUILDS_TARGET";
pub(crate) const REL_DEFINED_IN_FILE: &str = "DEFINED_IN_FILE";
pub(crate) const REL_HAS_PACKAGE: &str = "HAS_PACKAGE";
pub(crate) const REL_DEPENDS_ON_PACKAGE: &str = "DEPENDS_ON_PACKAGE";
pub(crate) const REL_IMPLEMENTS_TRAIT: &str = "IMPLEMENTS_TRAIT";
pub(crate) const REL_IMPLEMENTS_TYPE: &str = "IMPLEMENTS_TYPE";
pub(crate) const REL_MEMBER_OF_CLONE_GROUP: &str = "MEMBER_OF_CLONE_GROUP";
pub(crate) const REL_MEMBER_OF_FILE_CLONE_GROUP: &str = "MEMBER_OF_FILE_CLONE_GROUP";
pub(crate) const REL_HAS_CANONICAL: &str = "HAS_CANONICAL";
pub(crate) const REL_LAUNCHES: &str = "LAUNCHES";
pub(crate) const REL_ASSET_LINKS: &str = "ASSET_LINKS";

pub(crate) const ALL_NODE_LABELS: &[&str] = &[
    NODE_LABEL_FILE,
    NODE_LABEL_IMPORT,
    NODE_LABEL_MODEL,
    NODE_LABEL_EXTERNAL_API,
    NODE_LABEL_EXTERNAL_SYMBOL,
    NODE_LABEL_RESOURCE,
    NODE_LABEL_XCODE_TARGET,
    NODE_LABEL_XCODE_WORKSPACE,
    NODE_LABEL_XCODE_SCHEME,
    NODE_LABEL_CARGO_CRATE,
    NODE_LABEL_CARGO_WORKSPACE,
    NODE_LABEL_CLONE_GROUP,
    NODE_LABEL_FILE_CLONE_GROUP,
    NODE_LABEL_API_ROUTE,
];

pub(crate) const ALL_REL_TYPES: &[&str] = &[
    REL_CONTAINS,
    REL_CALLS,
    REL_CALLS_INFERRED,
    REL_CALLS_DB,
    REL_CALLS_DB_MODEL,
    REL_CALLS_API,
    REL_CALLS_SERVICE,
    REL_CALLS_API_EXTERNAL,
    REL_CALLS_EXTERNAL_SYMBOL,
    REL_IMPORTS,
    REL_IMPORTS_SYMBOL,
    REL_IMPLICIT_IMPORTS_SYMBOL,
    REL_EXPORTS_SYMBOL,
    REL_EXPORTS_SYMBOL_AS,
    REL_CALLS_API_ROUTE,
    REL_HANDLED_BY,
    REL_BACKED_BY_FILE,
    REL_BUNDLED_IN_TARGET,
    REL_BUNDLES_FILE,
    REL_REFERENCES_PROJECT,
    REL_BUILDS_TARGET,
    REL_DEFINED_IN_FILE,
    REL_HAS_PACKAGE,
    REL_DEPENDS_ON_PACKAGE,
    REL_IMPLEMENTS_TRAIT,
    REL_IMPLEMENTS_TYPE,
    REL_MEMBER_OF_CLONE_GROUP,
    REL_MEMBER_OF_FILE_CLONE_GROUP,
    REL_HAS_CANONICAL,
    REL_LAUNCHES,
    REL_ASSET_LINKS,
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn graph_schema_constants_are_unique() {
        let node_labels: HashSet<_> = ALL_NODE_LABELS.iter().copied().collect();
        assert_eq!(node_labels.len(), ALL_NODE_LABELS.len());

        let rel_types: HashSet<_> = ALL_REL_TYPES.iter().copied().collect();
        assert_eq!(rel_types.len(), ALL_REL_TYPES.len());
    }

    #[test]
    fn graph_schema_contains_core_contract_edges() {
        for rel in [
            REL_CONTAINS,
            REL_CALLS,
            REL_CALLS_INFERRED,
            REL_IMPORTS_SYMBOL,
            REL_EXPORTS_SYMBOL,
            REL_CALLS_EXTERNAL_SYMBOL,
            REL_LAUNCHES,
        ] {
            assert!(ALL_REL_TYPES.contains(&rel));
        }
    }
}
