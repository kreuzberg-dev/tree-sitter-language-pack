```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n", config)
}

```
