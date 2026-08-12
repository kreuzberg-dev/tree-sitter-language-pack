---
id: fixture_kotlin_android_smoke_m68k
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
    val config = mapper.readValue("{\"language\":\"m68k\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process(" move.l d0,d1\n", config)
}

```
