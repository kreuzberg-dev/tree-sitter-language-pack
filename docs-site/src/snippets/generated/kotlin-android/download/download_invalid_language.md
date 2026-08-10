```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.download(listOf("zzz_definitely_not_a_real_language_xyz"))
}

```
