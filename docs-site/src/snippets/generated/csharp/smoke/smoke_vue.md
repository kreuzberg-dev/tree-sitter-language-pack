---
id: fixture_csharp_smoke_vue
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<template><div>hello</div></template>", new ProcessConfig { Language = "vue" });

```
