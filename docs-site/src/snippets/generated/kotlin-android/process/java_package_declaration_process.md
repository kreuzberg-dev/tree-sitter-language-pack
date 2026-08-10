```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", config)
}

```
