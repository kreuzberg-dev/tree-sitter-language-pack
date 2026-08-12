---
id: fixture_kotlin_android_smoke_hlsl
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
    val config = mapper.readValue("{\"language\":\"hlsl\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("float4 main() : SV_Target { return 0; }", config)
}

```
