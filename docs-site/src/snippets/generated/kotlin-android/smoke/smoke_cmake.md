---
id: fixture_kotlin_android_smoke_cmake
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
    val config = mapper.readValue("{\"language\":\"cmake\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process("cmake_minimum_required(VERSION 3.0)", config)
}

```
