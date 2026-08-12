---
id: fixture_kotlin_android_parsing_rust_struct
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
    val config = mapper.readValue("{\"language\":\"rust\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("struct Point { x: f64, y: f64 }", config)
}

```
