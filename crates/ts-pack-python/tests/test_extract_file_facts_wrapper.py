import unittest

import tree_sitter_language_pack as ts


class ExtractFileFactsWrapperTests(unittest.TestCase):
    def test_extract_file_facts_uses_language_then_file_path(self):
        if not ts.has_language("typescript"):
            self.skipTest("typescript parser unavailable in test environment")

        facts = ts.extract_file_facts(
            """
            async function api(path) {
                return fetch(path);
            }

            await api("/api/leases");
            """,
            "typescript",
            "src/public/assets/app.js",
        )

        self.assertIn(
            {"client": "fetch", "method": "ANY", "path": "/api/leases"},
            facts.get("http_calls", []),
        )


if __name__ == "__main__":
    unittest.main()
