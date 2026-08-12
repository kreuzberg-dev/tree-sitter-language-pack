---
id: fixture_kotlin_android_rust_function_process
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
    val result = TreeSitterLanguagePack.process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", config)
}

```
