---
id: fixture_kotlin_android_data_extraction_toml_table
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
    val config = mapper.readValue("{\"data_extraction\":true,\"language\":\"toml\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("[server]\nhost = \"localhost\"\nport = 8080\n", config)
}

```
