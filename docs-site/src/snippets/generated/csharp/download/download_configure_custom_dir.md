---
id: fixture_csharp_download_configure_custom_dir
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

TreeSitterLanguagePackConverter.Configure(new PackConfig { CacheDir = "/tmp/tslp_test_cache" });

```
