---
id: fixture_kotlin_android_get_language_unknown
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
    val language = TreeSitterLanguagePack.getLanguage("nonexistent_xyz")
    } catch (error: Exception) {
        System.err.println("Call failed as expected: ${error.message}")
        return    }
    throw AssertionError("expected call to fail")
}

```
