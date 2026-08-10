```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", config)
}

```
