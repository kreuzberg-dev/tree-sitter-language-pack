---
id: fixture_kotlin_android_highlights_nonexistent_language
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    val result = TreeSitterLanguagePack.getHighlightsQuery("zzz_nonexistent_lang")
}

```
