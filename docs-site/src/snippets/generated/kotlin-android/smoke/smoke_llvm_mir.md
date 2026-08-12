---
id: fixture_kotlin_android_smoke_llvm_mir
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
    val config = mapper.readValue("{\"language\":\"llvm_mir\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("---\nname: foo\n...\n", config)
}

```
