---
id: fixture_kotlin_android_detect_content_bash_shebang
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() {
    val result = TreeSitterLanguagePack.detectLanguageFromContent("#!/bin/bash\necho hi")
}

```
