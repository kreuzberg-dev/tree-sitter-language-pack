```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("<?xml version=\"1.0\"?>\n<root>hello</root>", config)
}

```
