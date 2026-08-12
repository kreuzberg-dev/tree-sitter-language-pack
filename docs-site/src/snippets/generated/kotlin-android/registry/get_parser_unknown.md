---
id: fixture_kotlin_android_get_parser_unknown
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
    val parser = TreeSitterLanguagePack.getParser("nonexistent_xyz")
    } catch (error: Exception) {
        System.err.println("Call failed as expected: ${error.message}")
        return    }
    throw AssertionError("expected call to fail")
}

```
