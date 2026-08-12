---
id: fixture_csharp_smoke_sparql
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT ?s WHERE { ?s ?p ?o }", new ProcessConfig { Language = "sparql" });

```
