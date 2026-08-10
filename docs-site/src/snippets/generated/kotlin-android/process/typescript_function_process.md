```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, \${name}!`;\n}\n", config)
}

```
