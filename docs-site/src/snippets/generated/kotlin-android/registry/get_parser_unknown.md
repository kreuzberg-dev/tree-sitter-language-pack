```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val parser = TreeSitterLanguagePack.getParser("nonexistent_xyz")
}

```
