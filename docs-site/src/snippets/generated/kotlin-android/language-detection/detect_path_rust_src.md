---
id: fixture_kotlin_android_detect_path_rust_src
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    val result = TreeSitterLanguagePack.detectLanguageFromPath("src/main.rs")
}

```
