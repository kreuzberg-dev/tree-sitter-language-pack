```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", config)
}

```
