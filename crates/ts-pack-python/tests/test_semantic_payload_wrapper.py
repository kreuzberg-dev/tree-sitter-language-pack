import unittest

import tree_sitter_language_pack as ts


class SemanticPayloadWrapperTests(unittest.TestCase):
    def test_trace_graph_provenance_export_exists(self):
        self.assertTrue(hasattr(ts, "trace_graph_provenance"))

    def test_build_semantic_payload_enriches_usage_metadata(self):
        if not ts.has_language("python"):
            self.skipTest("python parser unavailable in test environment")

        payload = ts.build_semantic_payload(
            """
parser.parse(source)
""",
            "python",
            "examples/python_smoke/main.py",
            "proj",
        )

        chunks = payload.get("chunks") or []
        self.assertTrue(chunks)
        usage_chunks = [chunk for chunk in chunks if chunk.get("metadata", {}).get("chunk_role") == "example_usage"]
        self.assertTrue(usage_chunks)
        metadata = usage_chunks[0]["metadata"]
        self.assertEqual(metadata["member_usages"], ["parser.parse"])
        self.assertIn("parse", metadata["call_like_symbols"])

    def test_build_line_window_chunks_adds_entrypoint_anchor(self):
        chunks = ts.build_line_window_chunks(
            """
use anyhow::Result;

fn helper() {}

fn main() -> Result<()> {
    helper();
    Ok(())
}
""",
            "packages/desktop/src-tauri/src/main.rs",
            "proj",
            language="rust",
        )

        self.assertTrue(chunks)
        entrypoint_chunks = [chunk for chunk in chunks if chunk.get("metadata", {}).get("contains_entrypoint")]
        self.assertEqual(len(entrypoint_chunks), 1)
        entrypoint = entrypoint_chunks[0]
        self.assertIn("fn main()", entrypoint["text"])
        self.assertIn("main", entrypoint["metadata"]["declared_symbols"])
        self.assertEqual(entrypoint["metadata"]["chunk_role"], "definition")

    def test_build_line_window_chunks_extracts_swift_declared_symbols(self):
        chunks = ts.build_line_window_chunks(
            """
@main
struct DrawThingsCLI: ParsableCommand {
    static var configuration: CommandConfiguration { .init(commandName: "drawthings") }
}
""",
            "Apps/DrawThingsCLI/DrawThingsCLI.swift",
            "proj",
            language="swift",
        )

        self.assertTrue(chunks)
        metadata = chunks[0]["metadata"]
        self.assertIn("DrawThingsCLI", metadata["declared_symbols"])
        self.assertTrue(metadata["contains_definition"])
        self.assertEqual(metadata["chunk_role"], "definition")

    def test_build_swift_chunks_emit_type_definition_chunk(self):
        if not ts.has_language("swift"):
            self.skipTest("swift parser unavailable in test environment")

        chunks = ts.build_swift_chunks(
            """
import SwiftUI

struct SidebarView: View {
    var body: some View {
        Text("Sidebar")
    }
}
""",
            "FrameCreator/Views/SidebarView.swift",
            "proj",
        )

        self.assertTrue(chunks)
        definition_chunks = [
            chunk
            for chunk in chunks
            if "SidebarView" in (chunk.get("metadata", {}).get("declared_symbols") or [])
        ]
        self.assertTrue(definition_chunks)
        definition = definition_chunks[0]
        self.assertIn("struct SidebarView: View", definition["text"])
        self.assertTrue(definition["metadata"]["contains_definition"])
        self.assertEqual(definition["metadata"]["chunk_role"], "definition")
        self.assertEqual(definition["metadata"]["context_path"], ["SidebarView"])


if __name__ == "__main__":
    unittest.main()
