---
id: fixture_kotlin_android_download_configure_custom_dir
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
    val config = mapper.readValue("{\"cache_dir\":\"/tmp/tslp_test_cache\"}", PackConfig::class.java)
    TreeSitterLanguagePack.configure(config)
}

```
