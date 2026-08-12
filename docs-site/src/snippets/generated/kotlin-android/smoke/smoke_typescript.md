---
id: fixture_kotlin_android_smoke_typescript
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
    val config = mapper.readValue("{\"language\":\"typescript\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("const x: number = 42;", config)
}

```
