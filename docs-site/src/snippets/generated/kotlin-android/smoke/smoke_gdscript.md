---
id: fixture_kotlin_android_smoke_gdscript
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
    val config = mapper.readValue("{\"language\":\"gdscript\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("extends Node\nfunc _ready():\n\tpass", config)
}

```
