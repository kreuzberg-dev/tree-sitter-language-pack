```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n", config)
}

```
