```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", config)
}

```
