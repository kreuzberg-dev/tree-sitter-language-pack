---
id: fixture_kotlin_android_download_invalid_language
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    try {
    val result = TreeSitterLanguagePack.download(listOf("zzz_definitely_not_a_real_language_xyz"))
    } catch (error: Exception) {
        System.err.println("Call failed as expected: ${error.message}")
        return    }
    throw AssertionError("expected call to fail")
}

```
