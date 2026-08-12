---
id: fixture_csharp_detect_content_python_shebang
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.DetectLanguageFromContent("#!/usr/bin/env python3\npass");

```
