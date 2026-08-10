```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"hello\")\n}\n", config)
}

```
