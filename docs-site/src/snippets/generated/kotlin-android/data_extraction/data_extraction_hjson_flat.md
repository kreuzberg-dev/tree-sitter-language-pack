```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("{\n  host: \"localhost\"\n  port: 8080\n}\n", config)
}

```
