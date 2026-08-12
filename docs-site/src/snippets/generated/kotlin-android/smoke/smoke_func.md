---
id: fixture_kotlin_android_smoke_func
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
    val config = mapper.readValue("{\"language\":\"func\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("() recv_internal() {}", config)
}

```
