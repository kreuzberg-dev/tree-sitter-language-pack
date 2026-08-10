```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", config)
}

```
