import unittest
import asyncio

import tree_sitter_language_pack as ts
from tree_sitter_language_pack import _semantic_payload as semantic_payload


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

    def test_execute_semantic_index_rounds_coalesces_writes_per_round(self):
        embedded_calls = []
        written_batches = []

        async def _embed(batch):
            embedded_calls.append(len(batch))
            return [
                {
                    "ref_id": item["ref_id"],
                    "text": item["text"],
                    "vector": [0.1, 0.2],
                    "metadata": item["metadata"],
                }
                for item in batch
            ]

        async def _write(batch):
            written_batches.append(len(batch))
            return len(batch)

        new_chunks = [
            {"ref_id": f"chunk-{i}", "text": f"text {i}", "metadata": {"file": f"src/{i}.py"}}
            for i in range(4)
        ]

        result = asyncio.run(
            semantic_payload.execute_semantic_index_rounds(
                new_chunks,
                batch_size=2,
                concurrency=2,
                embed_batch_fn=_embed,
                write_batch_fn=_write,
            )
        )

        self.assertEqual(result["written"], 4)
        self.assertEqual(embedded_calls, [2, 2])
        self.assertEqual(written_batches, [4])

    def test_execute_semantic_index_prepare_scopes_queries_to_manifest(self):
        class _CursorResult:
            def __init__(self, rows=None, rowcount=0):
                self._rows = rows or []
                self.rowcount = rowcount

            async def fetchall(self):
                return self._rows

        class _Conn:
            def __init__(self):
                self.calls = []

            async def execute(self, query, params):
                self.calls.append((" ".join(str(query).split()), params))
                if "SELECT chunk_id" in query:
                    return _CursorResult(rows=[("chunk-1",)])
                return _CursorResult(rowcount=3)

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

        conn = _Conn()
        result = asyncio.run(
            semantic_payload.execute_semantic_index_prepare(
                conn,
                "proj",
                ["src/a.py", "src/b.py"],
                [[{"ref_id": "chunk-2", "text": "hello", "metadata": {"file": "src/a.py"}}]],
                rebuild=False,
            )
        )

        self.assertEqual(result["orphan_pruned"], 3)
        self.assertEqual(result["existing_ids"], {"chunk-1"})
        select_calls = [call for call in conn.calls if "SELECT chunk_id" in call[0]]
        self.assertEqual(len(select_calls), 1)
        self.assertIn("file_path = ANY(%s)", select_calls[0][0])
        self.assertEqual(select_calls[0][1], ("proj", ["src/a.py", "src/b.py"]))
        delete_calls = [call for call in conn.calls if "DELETE FROM codebase_embeddings" in call[0]]
        self.assertEqual(len(delete_calls), 1)
        self.assertIn("NOT (file_path = ANY(%s))", delete_calls[0][0])

    def test_execute_semantic_index_prepare_rebuild_ignores_existing_ids(self):
        class _CursorResult:
            def __init__(self, rows=None, rowcount=0):
                self._rows = rows or []
                self.rowcount = rowcount

            async def fetchall(self):
                return self._rows

        class _Conn:
            def __init__(self):
                self.calls = []

            async def execute(self, query, params):
                self.calls.append((" ".join(str(query).split()), params))
                if "SELECT chunk_id" in query:
                    return _CursorResult(rows=[("chunk-1",)])
                if "DELETE FROM codebase_embeddings" in query:
                    return _CursorResult(rowcount=5)
                return _CursorResult()

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

        conn = _Conn()
        result = asyncio.run(
            semantic_payload.execute_semantic_index_prepare(
                conn,
                "proj",
                ["src/a.py"],
                [[{"ref_id": "chunk-1", "text": "hello", "metadata": {"file": "src/a.py"}}]],
                rebuild=True,
            )
        )

        self.assertEqual(result["existing_ids"], set())
        self.assertEqual(len(result["new_chunks"]), 1)
        select_calls = [call for call in conn.calls if "SELECT chunk_id" in call[0]]
        self.assertEqual(select_calls, [])
        delete_calls = [call for call in conn.calls if "DELETE FROM codebase_embeddings" in call[0]]
        self.assertEqual(len(delete_calls), 1)

    def test_execute_semantic_index_driver_emits_prepare_done_progress(self):
        class _Cursor:
            async def fetchall(self):
                return []

        class _Conn:
            async def execute(self, *_args, **_kwargs):
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
                return None

        events = []

        async def _progress(event):
            events.append(event)

        async def _embed(batch):
            return batch

        async def _write(batch):
            return len(batch)

        result = asyncio.run(
            semantic_payload.execute_semantic_index_driver(
                _Conn(),
                "proj",
                ["src/sample.py"],
                [[{"ref_id": "chunk-1", "text": "hello", "metadata": {"file": "src/sample.py"}}]],
                rebuild=False,
                batch_size=1,
                concurrency=1,
                embed_batch_fn=_embed,
                write_batch_fn=_write,
                progress_fn=_progress,
            )
        )

        self.assertEqual(result["written"], 1)
        prepare_events = [event for event in events if event.get("phase") == "prepare_done"]
        self.assertEqual(len(prepare_events), 1)
        self.assertIn("prepare_seconds", prepare_events[0])


if __name__ == "__main__":
    unittest.main()
