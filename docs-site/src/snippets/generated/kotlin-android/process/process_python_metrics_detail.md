```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print('hello')\n\ndef world():\n    print('world')\n", config)
}

```
