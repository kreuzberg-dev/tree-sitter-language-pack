```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", config)
}

```
