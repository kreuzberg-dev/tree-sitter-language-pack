"""Smoke test for tree-sitter-language-pack Python bindings."""

from tree_sitter_language_pack import available_languages, get_parser, has_language

assert has_language("python"), "Python language should be available"

parser = get_parser("python")
tree = parser.parse(b"def hello(): pass")
assert tree is not None

langs = available_languages()
assert len(langs) > 0, f"Expected languages, got {len(langs)}"
