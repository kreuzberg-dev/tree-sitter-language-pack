---
id: fixture_kotlin_android_data_extraction_dtd_element_decl
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
    val config = mapper.readValue("{\"data_extraction\":true,\"language\":\"dtd\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", config)
}

```
