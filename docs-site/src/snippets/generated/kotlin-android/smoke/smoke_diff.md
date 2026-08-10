```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", config)
}

```
