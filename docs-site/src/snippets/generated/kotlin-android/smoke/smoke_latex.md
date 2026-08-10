```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", config)
}

```
