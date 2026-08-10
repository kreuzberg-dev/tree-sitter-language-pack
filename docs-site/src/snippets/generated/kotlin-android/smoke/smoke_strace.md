```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("open(\"/x\", O_RDONLY) = 3\n", config)
}

```
