#![allow(dead_code)]

pub const NODE_LABEL_FILE: &str = "File";
pub const NODE_LABEL_IMPORT: &str = "Import";
pub const NODE_LABEL_MODEL: &str = "Model";
pub const NODE_LABEL_EXTERNAL_API: &str = "ExternalAPI";
pub const NODE_LABEL_EXTERNAL_SYMBOL: &str = "ExternalSymbol";
pub const NODE_LABEL_RESOURCE: &str = "Resource";
pub const NODE_LABEL_XCODE_TARGET: &str = "XcodeTarget";
pub const NODE_LABEL_XCODE_WORKSPACE: &str = "XcodeWorkspace";
pub const NODE_LABEL_XCODE_SCHEME: &str = "XcodeScheme";
pub const NODE_LABEL_CARGO_CRATE: &str = "CargoCrate";
pub const NODE_LABEL_CARGO_WORKSPACE: &str = "CargoWorkspace";
pub const NODE_LABEL_CLONE_GROUP: &str = "CloneGroup";
pub const NODE_LABEL_FILE_CLONE_GROUP: &str = "FileCloneGroup";
pub const NODE_LABEL_API_ROUTE: &str = "ApiRoute";

pub const REL_CONTAINS: &str = "CONTAINS";
pub const REL_CALLS: &str = "CALLS";
pub const REL_CALLS_INFERRED: &str = "CALLS_INFERRED";
pub const REL_CALLS_DB: &str = "CALLS_DB";
pub const REL_CALLS_DB_MODEL: &str = "CALLS_DB_MODEL";
pub const REL_CALLS_API: &str = "CALLS_API";
pub const REL_CALLS_SERVICE: &str = "CALLS_SERVICE";
pub const REL_CALLS_API_EXTERNAL: &str = "CALLS_API_EXTERNAL";
pub const REL_CALLS_EXTERNAL_SYMBOL: &str = "CALLS_EXTERNAL_SYMBOL";
pub const REL_IMPORTS: &str = "IMPORTS";
pub const REL_IMPORTS_SYMBOL: &str = "IMPORTS_SYMBOL";
pub const REL_IMPLICIT_IMPORTS_SYMBOL: &str = "IMPLICIT_IMPORTS_SYMBOL";
pub const REL_EXPORTS_SYMBOL: &str = "EXPORTS_SYMBOL";
pub const REL_EXPORTS_SYMBOL_AS: &str = "EXPORTS_SYMBOL_AS";
pub const REL_CALLS_API_ROUTE: &str = "CALLS_API_ROUTE";
pub const REL_HANDLED_BY: &str = "HANDLED_BY";
pub const REL_BACKED_BY_FILE: &str = "BACKED_BY_FILE";
pub const REL_BUNDLED_IN_TARGET: &str = "BUNDLED_IN_TARGET";
pub const REL_BUNDLES_FILE: &str = "BUNDLES_FILE";
pub const REL_REFERENCES_PROJECT: &str = "REFERENCES_PROJECT";
pub const REL_BUILDS_TARGET: &str = "BUILDS_TARGET";
pub const REL_DEFINED_IN_FILE: &str = "DEFINED_IN_FILE";
pub const REL_HAS_PACKAGE: &str = "HAS_PACKAGE";
pub const REL_DEPENDS_ON_PACKAGE: &str = "DEPENDS_ON_PACKAGE";
pub const REL_IMPLEMENTS_TRAIT: &str = "IMPLEMENTS_TRAIT";
pub const REL_IMPLEMENTS_TYPE: &str = "IMPLEMENTS_TYPE";
pub const REL_SWIFT_EXTENDS_TYPE: &str = "SWIFT_EXTENDS_TYPE";
pub const REL_MEMBER_OF_CLONE_GROUP: &str = "MEMBER_OF_CLONE_GROUP";
pub const REL_MEMBER_OF_FILE_CLONE_GROUP: &str = "MEMBER_OF_FILE_CLONE_GROUP";
pub const REL_HAS_CANONICAL: &str = "HAS_CANONICAL";
pub const REL_LAUNCHES: &str = "LAUNCHES";
pub const REL_ASSET_LINKS: &str = "ASSET_LINKS";
pub const REL_FILE_GRAPH_LINK: &str = "FILE_GRAPH_LINK";
pub const REL_CALLS_FILE: &str = "CALLS_FILE";

pub const ALL_NODE_LABELS: &[&str] = &[
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

pub const ALL_REL_TYPES: &[&str] = &[
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
    REL_SWIFT_EXTENDS_TYPE,
    REL_MEMBER_OF_CLONE_GROUP,
    REL_MEMBER_OF_FILE_CLONE_GROUP,
    REL_HAS_CANONICAL,
    REL_LAUNCHES,
    REL_ASSET_LINKS,
    REL_FILE_GRAPH_LINK,
    REL_CALLS_FILE,
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
            REL_FILE_GRAPH_LINK,
            REL_CALLS_FILE,
        ] {
            assert!(ALL_REL_TYPES.contains(&rel));
        }
    }
}
