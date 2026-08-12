---
id: fixture_ruby_detect_content_python_shebang
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.detect_language_from_content("\#!/usr/bin/env python3\npass")

```
