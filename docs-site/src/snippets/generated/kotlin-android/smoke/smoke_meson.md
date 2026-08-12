---
id: fixture_kotlin_android_smoke_meson
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
    val config = mapper.readValue("{\"language\":\"meson\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("project('hello', 'c')", config)
}

```
