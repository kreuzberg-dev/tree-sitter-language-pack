```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", config)
}

```
