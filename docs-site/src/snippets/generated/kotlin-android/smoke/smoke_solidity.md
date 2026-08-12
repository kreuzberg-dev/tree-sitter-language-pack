---
id: fixture_kotlin_android_smoke_solidity
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
    val config = mapper.readValue("{\"language\":\"solidity\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("pragma solidity ^0.8.0;\ncontract Main {}", config)
}

```
