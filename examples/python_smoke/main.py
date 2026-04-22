"""Smoke test for tree-sitter-language-pack Python bindings."""

from tree_sitter_language_pack import available_languages, has_language, parse_string

assert has_language("python"), "Python language should be available"

tree = parse_string("python", b"def hello(): pass")
assert tree is not None

langs = available_languages()
assert len(langs) > 0, f"Expected languages, got {len(langs)}"
