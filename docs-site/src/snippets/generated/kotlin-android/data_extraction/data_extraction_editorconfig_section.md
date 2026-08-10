```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("[*.rs]\nindent_style = space\nindent_size = 4\n", config)
}

```
