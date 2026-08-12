---
id: fixture_kotlin_android_download_multiple_languages
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    val result = TreeSitterLanguagePack.download(listOf("python", "rust"))
}

```
