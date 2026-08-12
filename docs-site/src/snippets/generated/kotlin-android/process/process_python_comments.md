---
id: fixture_kotlin_android_process_python_comments
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
    val config = mapper.readValue("{\"comments\":true,\"language\":\"python\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", config)
}

```
