---
id: fixture_kotlin_android_process_javascript_exports_detail
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
    val config = mapper.readValue("{\"language\":\"javascript\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("export function greet(name) {\n  return `Hello \${name}`;\n}\n\nexport const VERSION = '1.0';\n", config)
}

```
