import unittest
import asyncio

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

    def test_build_swift_chunks_split_oversized_single_line_members(self):
        if not ts.has_language("swift"):
            self.skipTest("swift parser unavailable in test environment")

        huge_literal = "a" * 47000
        chunks = ts.build_swift_chunks(
            f"""
struct StringNormalizationCases {{
    static let cases = ["{huge_literal}"]
}}
""",
            "validation-test/stdlib/StringNormalization.swift",
            "proj",
            chunk_max_size=4000,
            chunk_lines=60,
            overlap_lines=10,
        )

        self.assertTrue(chunks)
        max_body_bytes = max(
            len("\n".join(chunk["text"].splitlines()[1:]).encode("utf-8"))
            for chunk in chunks
        )
        self.assertLessEqual(max_body_bytes, 4000)
        self.assertTrue(
            any(
                "StringNormalizationCases" in (chunk.get("metadata", {}).get("declared_symbols") or [])
                for chunk in chunks
            )
        )

    def test_build_indexing_chunks_falls_back_for_pathological_swift_nesting(self):
        huge_parens = "(_:" * 3000 + "0" + ")" * 3000
        payload = ts.build_indexing_chunks(
            f"let x =\n{huge_parens}\n",
            "test/Parse/structure_overflow_paren_exprs.swift",
            "proj",
            language="swift",
            chunk_max_size=4000,
            chunk_overlap=200,
            chunk_lines=60,
            overlap_lines=10,
        )

        chunks = payload.get("chunks") or []
        self.assertTrue(chunks)
        self.assertEqual(payload.get("language"), "swift")
        self.assertTrue(all("metadata" in chunk for chunk in chunks))

    def test_build_indexing_chunks_falls_back_for_objcpp_and_keeps_objc_symbols(self):
        source = """
#import <Foundation/Foundation.h>

@interface Counter : NSObject
- (int)add:(int)a to:(int)b;
@end

@implementation Counter
- (int)add:(int)a to:(int)b {
    std::vector<int> values = {a, b};
    return values[0] + values[1];
}
@end
"""
        payload = ts.build_indexing_chunks(
            source,
            "src/Bridge.mm",
            "proj",
            language="objc",
            chunk_max_size=4000,
            chunk_overlap=200,
            chunk_lines=60,
            overlap_lines=10,
        )

        chunks = payload.get("chunks") or []
        self.assertTrue(chunks)
        all_declared = {
            symbol
            for chunk in chunks
            for symbol in ((chunk.get("metadata") or {}).get("declared_symbols") or [])
        }
        self.assertIn("Counter", all_declared)

    def test_build_indexing_chunks_strips_nul_bytes_from_chunk_text(self):
        payload = ts.build_indexing_chunks(
            "let weird = \"a\\x00b\\x00c\"\n",
            "test/Parse/strange-characters.swift",
            "proj",
            language="swift",
            chunk_max_size=4000,
            chunk_overlap=200,
            chunk_lines=60,
            overlap_lines=10,
        )

        chunks = payload.get("chunks") or []
        self.assertTrue(chunks)
        self.assertTrue(all("\x00" not in chunk["text"] for chunk in chunks))

    def test_execute_semantic_index_driver_commits_before_rounds(self):
        class _Conn:
            def __init__(self):
                self.commits = 0

            async def execute(self, *_args, **_kwargs):
                class _Cursor:
                    async def fetchall(self):
                        return []

                    def __aiter__(self):
                        async def _iter():
                            if False:
                                yield None
                        return _iter()

                return _Cursor()

            def cursor(self):
                class _PruneCursor:
                    rowcount = 0

                    async def __aenter__(self):
                        return self

                    async def __aexit__(self, exc_type, exc, tb):
                        return False

                    async def execute(self, *_args, **_kwargs):
                        return None

                return _PruneCursor()

            async def commit(self):
                self.commits += 1

        conn = _Conn()
        observed = {"commit_seen": False}

        async def _embed(batch):
            return batch

        async def _write(batch):
            observed["commit_seen"] = conn.commits > 0
            return len(batch)

        async def _run():
            return await ts.execute_semantic_index_driver(
                conn,
                "proj",
                ["src/sample.py"],
                [[{"ref_id": "chunk-1", "text": "hello", "metadata": {"file": "src/sample.py"}}]],
                rebuild=False,
                batch_size=1,
                concurrency=1,
                embed_batch_fn=_embed,
                write_batch_fn=_write,
            )

        result = asyncio.run(_run())

        self.assertEqual(conn.commits, 1)
        self.assertTrue(observed["commit_seen"])
        self.assertEqual(result["written"], 1)


if __name__ == "__main__":
    unittest.main()
