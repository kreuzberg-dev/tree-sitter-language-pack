---
id: fixture_kotlin_android_smoke_smali
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
    val config = mapper.readValue("{\"language\":\"smali\"}", ProcessConfig::class.java)
    val result = TreeSitterLanguagePack.process(".class public LMain;\n.super Ljava/lang/Object;", config)
}

```
