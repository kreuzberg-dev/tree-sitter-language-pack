---
id: fixture_kotlin_android_smoke_gomod
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
    val config = mapper.readValue("{\"language\":\"gomod\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("module example.com/hello\n\ngo 1.21", config)
}

```
