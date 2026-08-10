```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("export function greet(name) {\n  return `Hello \${name}`;\n}\n\nexport const VERSION = '1.0';\n", config)
}

```
