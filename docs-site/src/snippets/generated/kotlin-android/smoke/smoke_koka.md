---
id: fixture_kotlin_android_smoke_koka
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
    val config = mapper.readValue("{\"language\":\"koka\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("fun main()\n  1\n", config)
}

```
