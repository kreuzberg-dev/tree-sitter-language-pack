---
id: fixture_kotlin_android_data_extraction_kdl_nested
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
    val config = mapper.readValue("{\"data_extraction\":true,\"language\":\"kdl\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("server {\n  host \"localhost\"\n  port 8080\n}\n", config)
}

```
