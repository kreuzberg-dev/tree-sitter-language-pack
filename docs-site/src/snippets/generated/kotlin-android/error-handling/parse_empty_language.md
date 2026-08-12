---
id: fixture_kotlin_android_parse_empty_language
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper

fun main() {
    val mapper = jacksonObjectMapper()
    try {
    val config = mapper.readValue("{\"language\":\"\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("x = 1", config)
    } catch (error: Exception) {
        System.err.println("Call failed as expected: ${error.message}")
        return    }
    throw AssertionError("expected call to fail")
}

```
