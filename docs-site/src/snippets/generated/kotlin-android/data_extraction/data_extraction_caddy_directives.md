---
id: fixture_kotlin_android_data_extraction_caddy_directives
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
    val config = mapper.readValue("{\"data_extraction\":true,\"language\":\"caddy\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("localhost\nroot * /var/www\nfile_server\n", config)
}

```
