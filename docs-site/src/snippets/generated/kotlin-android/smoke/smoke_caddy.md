---
id: fixture_kotlin_android_smoke_caddy
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
    val config = mapper.readValue("{\"language\":\"caddy\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process(":8080 {\n\trespond \"Hello\"\n}", config)
}

```
