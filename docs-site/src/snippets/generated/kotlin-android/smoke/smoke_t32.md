---
id: fixture_kotlin_android_smoke_t32
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
    val config = mapper.readValue("{\"language\":\"t32\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("PRINT 1\n", config)
}

```
