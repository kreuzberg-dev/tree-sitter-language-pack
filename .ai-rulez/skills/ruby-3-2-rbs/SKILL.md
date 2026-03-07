---
priority: high
description: "Ruby 3.2+ with RBS & Steep"
---

# Ruby 3.2+ with RBS & Steep

**Ruby 3.2+ · RBS type definitions · Steep · RSpec · Rubocop**

- Ruby 3.2+ with .ruby-version; rbenv for version management
- RBS files in sig/ directory parallel to source: lib/foo.rb → sig/foo.rbs
- Steep for type checking; avoid Any types, use union types explicitly
- RSpec testing: 80%+ coverage, function-like tests
- Rubocop with auto-fix: line length ≤120
- Code quality: methods <10 lines, guard clauses, modules for mixins
