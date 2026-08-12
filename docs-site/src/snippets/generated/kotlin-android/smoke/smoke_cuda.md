---
id: fixture_kotlin_android_smoke_cuda
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
    val config = mapper.readValue("{\"language\":\"cuda\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("__global__ void kernel() {}", config)
}

```
