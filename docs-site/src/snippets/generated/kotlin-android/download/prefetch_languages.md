```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    TreeSitterLanguagePack.prefetch(listOf("python"))
}

```
