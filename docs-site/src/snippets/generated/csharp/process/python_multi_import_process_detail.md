---
id: fixture_csharp_python_multi_import_process_detail
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", new ProcessConfig { Language = "python" });

```
