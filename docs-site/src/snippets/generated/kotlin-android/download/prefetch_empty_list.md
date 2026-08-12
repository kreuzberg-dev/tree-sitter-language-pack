---
id: fixture_kotlin_android_prefetch_empty_list
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    TreeSitterLanguagePack.prefetch(listOf())
}

```
